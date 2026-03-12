use dioxus::prelude::*;
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
    PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope, StandardErrorResponse, reqwest,
};
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::sleep;

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
    pub jwks_state: JwksState,
}

impl OidcConfig {
    pub fn new(client: OidcClient, jwks: JwkSet, jwks_uri: JsonWebKeySetUrl) -> Self {
        Self {
            client,
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

pub(crate) async fn jwks_refresh_loop(state: JwksState) {
    loop {
        sleep(Duration::from_secs(900)).await;

        match fetch_jwks(&state.jwks_uri).await {
            Ok(new_jwks) => {
                info!("JWKS updated with {}", new_jwks.keys.len());
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
        IssuerUrl::new(env::var("OIDC_ISSUER_URL")?)?,
        &http_client,
    )
    .await?;
    let jwks_uri = provider_metadata.jwks_uri().clone();

    let redirect_url = format!("{}/api/oidc/redirect", env::var("SERVER_URL")?);
    let client = CoreClient::from_provider_metadata(
        provider_metadata,
        ClientId::new(env::var("OIDC_CLIENT_ID")?),
        Some(ClientSecret::new(env::var("OIDC_CLIENT_SECRET")?)),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url)?);

    let jwks = fetch_jwks(&jwks_uri).await?;

    Ok(OidcConfig::new(client, jwks, jwks_uri))
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

pub(crate) fn create_oidc_challenge(client: &OidcClient) -> Result<OidcMetadata, anyhow::Error> {
    let (pkce_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
    let mut authorization_request = client.authorize_url(
        CoreAuthenticationFlow::AuthorizationCode,
        CsrfToken::new_random,
        Nonce::new_random,
    );

    let env_scopes = env::var("OAUTH_SCOPES").unwrap_or("openid email profile".to_string());
    let scopes = env_scopes.split_whitespace();

    for scope in scopes {
        authorization_request = authorization_request.add_scope(Scope::new(scope.to_string()));
    }
    authorization_request = authorization_request.set_pkce_challenge(pkce_challenge);

    let (auth_url, csrf_token, nonce) = authorization_request.url();

    let metadata = OidcMetadata {
        url: auth_url,
        pkce_code_verifier,
        nonce,
        csrf_token,
    };
    Ok(metadata)
}

pub async fn verify_oidc_challenge(
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
    pub aud: String,
    pub exp: usize,
    pub email: String,
}

pub(crate) async fn validate_authorization_token(
    jwk_state: &JwksState,
    token: &str,
) -> Result<Claims, anyhow::Error> {
    let header = jsonwebtoken::decode_header(token)?;
    let kid = header.kid.ok_or(anyhow::anyhow!("Missing kid"))?;
    debug!("Kid: {kid}");
    let jwk_lock = jwk_state.jwks.read().await;
    let jwk = jwk_lock.find(&kid).ok_or(anyhow::anyhow!("Missing jwk"))?;
    debug!("JWK: {jwk:?}");

    let mut validation = Validation::new(header.alg);
    validation.set_issuer(&["http://auth.roommates.local/realms/roommates"]);
    validation.set_audience(&["account"]);

    let token_data = jsonwebtoken::decode(
        token,
        &DecodingKey::from_jwk(jwk)
            .unwrap_or_else(|err| panic!("Failed to create decoding key: {err}")),
        &validation,
    )
    .unwrap_or_else(|err| panic!("Token decode error: {err}"));

    Ok(token_data.claims)
}
