use crate::server;
use crate::server::utils::get_env_or;
use dioxus::prelude::*;
use entity::prelude::User;
use jsonwebtoken::jwk::JwkSet;
use jsonwebtoken::{DecodingKey, Validation};
use openidconnect::core::{
    CoreAuthDisplay, CoreAuthPrompt, CoreAuthenticationFlow, CoreClient, CoreErrorResponseType,
    CoreGenderClaim, CoreJsonWebKey, CoreJweContentEncryptionAlgorithm, CoreProviderMetadata,
    CoreRevocableToken, CoreRevocationErrorResponse, CoreTokenIntrospectionResponse,
    CoreTokenResponse,
};
use openidconnect::url::Url;
use openidconnect::{
    AuthorizationCode, Client, ClientId, ClientSecret, CsrfToken, EmptyAdditionalClaims,
    EndpointMaybeSet, EndpointNotSet, EndpointSet, IssuerUrl, JsonWebKeySetUrl, Nonce,
    OAuth2TokenResponse, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, RefreshToken, Scope,
    StandardErrorResponse, reqwest,
};
use serde::{Deserialize, Serialize};
use server::{AppState, constants};
use std::env;
use std::sync::Arc;
use std::time::Duration;
use time::ext::NumericalDuration;
use tokio::sync::RwLock;
use tower_cookies::cookie::SameSite;
use tower_cookies::{Cookie, Cookies};

pub type OidcClient = Client<
    EmptyAdditionalClaims,
    CoreAuthDisplay,
    CoreGenderClaim,
    CoreJweContentEncryptionAlgorithm,
    CoreJsonWebKey,
    CoreAuthPrompt,
    StandardErrorResponse<CoreErrorResponseType>,
    CoreTokenResponse,
    CoreTokenIntrospectionResponse,
    CoreRevocableToken,
    CoreRevocationErrorResponse,
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointMaybeSet,
    EndpointMaybeSet,
>;

#[derive(Clone)]
pub struct JwksState {
    pub jwks: Arc<RwLock<JwkSet>>,
    pub jwks_uri: JsonWebKeySetUrl,
}

#[derive(Clone)]
pub struct OidcConfig {
    pub client: OidcClient,
    pub metadata: CoreProviderMetadata,
    pub jwks_state: JwksState,
}

impl OidcConfig {
    #[must_use]
    pub fn new(
        client: OidcClient,
        metadata: CoreProviderMetadata,
        jwks: JwkSet,
        jwks_uri: JsonWebKeySetUrl,
    ) -> Self {
        Self {
            client,
            metadata,
            jwks_state: JwksState {
                jwks: Arc::new(RwLock::new(jwks)),
                jwks_uri,
            },
        }
    }
}

impl From<OidcConfig> for OidcClient {
    fn from(value: OidcConfig) -> Self {
        value.client
    }
}

pub(crate) fn build_http_client() -> Result<reqwest::Client, anyhow::Error> {
    let http_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;
    Ok(http_client)
}

pub(crate) async fn fetch_jwks(jwks_uri: &JsonWebKeySetUrl) -> Result<JwkSet, anyhow::Error> {
    let http_client = build_http_client()?;
    let jwks = http_client
        .get(jwks_uri.url().as_str())
        .send()
        .await?
        .json::<JwkSet>()
        .await?;
    debug!("JWKS: {:?}", jwks);
    Ok(jwks)
}

pub(crate) async fn jwks_refresh_loop(state: JwksState, refresh_duration: time::Duration) {
    let mut interval =
        tokio::time::interval(refresh_duration.try_into().expect("invalid duration"));
    loop {
        interval.tick().await;

        match fetch_jwks(&state.jwks_uri).await {
            Ok(new_jwks) => {
                info!("JWKS updated with {} keys", new_jwks.keys.len());
                let mut jwks = state.jwks.write().await;
                *jwks = new_jwks;
            }
            Err(err) => {
                warn!("JWKS refresh failed: {err}");
            }
        }
    }
}

pub(crate) async fn create_oidc_config() -> Result<OidcConfig, anyhow::Error> {
    let http_client = build_http_client()?;

    let provider_metadata = CoreProviderMetadata::discover_async(
        IssuerUrl::new(env::var(constants::OIDC_ISSUER_URL_ENV_VAR)?)?,
        &http_client,
    )
    .await?;
    let jwks_uri = provider_metadata.jwks_uri().clone();

    let redirect_url = format!(
        "{}/api/oidc/redirect",
        env::var(constants::SERVER_URL_ENV_VAR)?
    );
    let client = CoreClient::from_provider_metadata(
        provider_metadata.clone(),
        ClientId::new(env::var(constants::OIDC_CLIENT_ID_ENV_VAR)?),
        Some(ClientSecret::new(env::var(
            constants::OIDC_CLIENT_SECRET_ENV_VAR,
        )?)),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url)?);

    let jwks = fetch_jwks(&jwks_uri).await?;

    Ok(OidcConfig::new(client, provider_metadata, jwks, jwks_uri))
}

pub struct OidcMetadata {
    pub url: Url,
    pub csrf_token: CsrfToken,
    pub pkce_code_verifier: PkceCodeVerifier,
    pub nonce: Nonce,
}

#[derive(Serialize, Deserialize)]
pub struct OidcSession {
    pub(crate) pkce_code_verifier: PkceCodeVerifier,
    pub(crate) csrf_token: CsrfToken,
    pub(crate) nonce: Nonce,
}

impl From<OidcMetadata> for OidcSession {
    fn from(value: OidcMetadata) -> Self {
        Self {
            pkce_code_verifier: value.pkce_code_verifier,
            nonce: value.nonce,
            csrf_token: value.csrf_token,
        }
    }
}

pub(crate) fn create_oidc_challenge(client: &OidcClient) -> OidcMetadata {
    let (pkce_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
    let mut authorization_request = client.authorize_url(
        CoreAuthenticationFlow::AuthorizationCode,
        CsrfToken::new_random,
        Nonce::new_random,
    );

    let env_scopes =
        env::var(constants::OIDC_SCOPES_ENV_VAR).unwrap_or("openid email profile".to_string());
    let scopes = env_scopes.split_whitespace();

    for scope in scopes {
        authorization_request = authorization_request.add_scope(Scope::new(scope.to_string()));
    }
    authorization_request = authorization_request.set_pkce_challenge(pkce_challenge);

    let (auth_url, csrf_token, nonce) = authorization_request.url();

    OidcMetadata {
        url: auth_url,
        pkce_code_verifier,
        nonce,
        csrf_token,
    }
}

/// Verifies a pkce challenge and returns the `IdToken`
///
/// # Arguments
///
/// * `client`:
/// * `authorization_code`:
/// * `pkce_code_verifier`:
///
/// returns: Result<`CoreTokenResponse`, Error>
///
/// # Errors
///
/// * `ConfigurationError`: `exchange_code` failed to to uninitialized `OidcClient`
/// * `RequestTokenError`: Fetching the token from the `AuthProvider` failed
/// * Errors retuned by [`build_http_client`]
pub(crate) async fn verify_oidc_challenge(
    client: &OidcClient,
    authorization_code: String,
    pkce_code_verifier: PkceCodeVerifier,
) -> Result<CoreTokenResponse, anyhow::Error> {
    let http_client = build_http_client()?;
    let token = client
        .exchange_code(AuthorizationCode::new(authorization_code))?
        .set_pkce_verifier(pkce_code_verifier)
        .request_async(&http_client)
        .await?;
    Ok(token)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub iss: String,
    #[serde(default)]
    pub aud: Option<serde_json::Value>,
    pub exp: usize,
    pub email: String,
}

pub(crate) async fn validate_authorization_token(
    oidc_config: &OidcConfig,
    token: &str,
) -> Result<Claims, anyhow::Error> {
    let header = jsonwebtoken::decode_header(token)?;
    let kid = header.kid.ok_or(anyhow::anyhow!("Missing kid"))?;
    let jwk_lock = oidc_config.jwks_state.jwks.read().await;
    let jwk = jwk_lock.find(&kid).ok_or(anyhow::anyhow!("Missing jwk"))?;

    let mut validation = Validation::new(header.alg);
    validation.set_issuer(&[oidc_config.metadata.issuer()]);
    let audiences = get_env_or(
        constants::OIDC_AUDIENCE_ENV_VAR,
        vec![oidc_config.client.client_id().as_str().to_string()],
        |val| val.trim().split(',').map(ToString::to_string).collect(),
    );
    validation.set_audience(&audiences);

    let token_data = jsonwebtoken::decode(token, &DecodingKey::from_jwk(jwk)?, &validation)?;

    Ok(token_data.claims)
}

pub(crate) async fn get_user_from_authorization_token(
    token: &str,
    app_state: &AppState,
) -> Result<Option<entity::user::Model>, anyhow::Error> {
    let oidc_config = app_state.oidc_config.as_ref().expect("OIDC is disabled!");

    let claims = validate_authorization_token(oidc_config, token)
        .await
        .inspect_err(|e| error!("Error validating OIDC claims: {}", e))?;
    let user = User::find_by_email(claims.email)
        .one(&app_state.database)
        .await?;
    Ok(user)
}

pub(crate) async fn refresh_authorization_token(
    refresh_token: &str,
    app_state: &AppState,
) -> Result<CoreTokenResponse, anyhow::Error> {
    let oidc_client = &app_state
        .oidc_config
        .as_ref()
        .expect("OIDC is disabled!")
        .client;
    let http_client = build_http_client()?;

    let refresh_token = RefreshToken::new(refresh_token.to_owned());
    let refresh_token_request = oidc_client.exchange_refresh_token(&refresh_token)?;
    let new_token = refresh_token_request.request_async(&http_client).await?;
    Ok(new_token)
}

pub(crate) fn add_oidc_cookies(
    cookies: &Cookies,
    token_response: &CoreTokenResponse,
) -> Result<(), anyhow::Error> {
    let expires_at = token_response
        .expires_in()
        .unwrap_or(Duration::from_secs(3600));

    cookies.add(
        Cookie::build((
            constants::OIDC_AUTHORIZATION_COOKIE_NAME,
            format!("Bearer {}", token_response.access_token().secret()),
        ))
        .secure(!cfg!(debug_assertions))
        .http_only(true)
        .path("/")
        .same_site(SameSite::Strict)
        .expires(time::OffsetDateTime::now_local()? + expires_at)
        .build(),
    );
    if let Some(refresh_token) = token_response.refresh_token() {
        cookies.add(
            Cookie::build((
                constants::OIDC_REFRESH_COOKIE_NAME,
                format!("Bearer {}", refresh_token.secret()),
            ))
            .secure(!cfg!(debug_assertions))
            .http_only(true)
            .path("/")
            .same_site(SameSite::Strict)
            .expires(time::OffsetDateTime::now_local()? + 30.days())
            .build(),
        );
    }
    Ok(())
}
