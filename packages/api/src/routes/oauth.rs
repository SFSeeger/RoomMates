#[cfg(feature = "server")]
use crate::server;
use dioxus::fullstack::Redirect;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use dioxus::server::axum::Extension;
use sea_orm::PaginatorTrait;

const OIDC_SESSION_KEY: &str = "oidc_metadata";
#[allow(clippy::unused_async)]
#[get("/api/oidc/login", state: Extension<server::AppState>,  session: Extension<tower_sessions::Session> )]
pub async fn oauth_login() -> Result<Redirect, ServerFnError> {
    let oauth_client = state
        .oidc_client
        .as_ref()
        .or_internal_server_error("OAuth Client Missing or disabled!")?;
    let metadata = server::auth::create_oidc_challenge(oauth_client)
        .or_internal_server_error("Error creating pkce challenge")?;

    let redirect_url = metadata.url.as_str().to_string();
    let oidc_session: server::auth::OidcSession = metadata.into();

    session
        .insert(OIDC_SESSION_KEY, oidc_session)
        .await
        .or_internal_server_error("Failed to create session")?;

    Ok(Redirect::to(&redirect_url))
}

#[get("/api/oidc/redirect?state&code", ext: Extension<server::AppState>, _auth: Extension<server::AuthenticationState>, cookies: Extension<tower_cookies::Cookies>, session: Extension<tower_sessions::Session> )]
pub async fn oauth_redirect(
    state: Option<String>,
    code: Option<String>,
) -> Result<Redirect, ServerFnError> {
    use openidconnect::{AccessTokenHash, OAuth2TokenResponse, TokenResponse};
    use tower_cookies::Cookie;
    use entity::prelude::*;

    let oidc_client = ext
        .oidc_client
        .as_ref()
        .or_internal_server_error("OAuth Client Missing or disabled!")?;

    debug!("Trying oauth login with state {state:?} and code {code:?}");

    let oidc_session: server::auth::OidcSession = session
        .get(OIDC_SESSION_KEY)
        .await
        .or_internal_server_error("Failed to retrieve session")?
        .or_bad_request("Failed to get session with required metadata")?;

    (*oidc_session.csrf_token.secret() == state.or_bad_request("Missing State parameter")?)
        .or_bad_request("CSRF Mismatch")?;

    let token_response = server::auth::verify_oidc_challenge(
        oidc_client,
        code.or_bad_request("Missing Authorization code")?,
        oidc_session.pkce_code_verifier,
    )
    .await
    .inspect_err(|e| error!("{e}"))
    .or_forbidden("Error receiving token")?;

    let id_token = token_response
        .id_token()
        .or_internal_server_error("Server did not return ID Token")?;
    let id_token_verifier = oidc_client.id_token_verifier();
    let claims = id_token
        .claims(&id_token_verifier, &oidc_session.nonce)
        .or_unauthorized("Unable to validate claims")?;

    if let Some(expected_access_token_hash) = claims.access_token_hash() {
        let actual_access_token_hash = AccessTokenHash::from_token(
            token_response.access_token(),
            id_token
                .signing_alg()
                .or_internal_server_error("Error extracting sining algorithm")?,
            id_token
                .signing_key(&id_token_verifier)
                .or_internal_server_error("Error extracting signing key")?,
        )
        .or_internal_server_error("Error constructing expected access token")?;
        (actual_access_token_hash == *expected_access_token_hash)
            .or_unauthorized("Invalid access token")?;
    }

    debug!(
        "User {} with e-mail address {} has authenticated successfully",
        claims.subject().as_str(),
        claims
            .email()
            .map_or("<not provided>", |email| email.as_str())
    );

    let email: &str = claims.email().map(|email| email.as_str()).or_bad_request("Email is required")?;
    let first_name: &str = claims.given_name().map(|name| name.as_str()).or_bad_request("Missing given name")?;
    let last_name: &str= claims.family_name().map(|name| name.as_str()).or_bad_request("Missing family name")?;

    if User::find_by_email(email).count(&ext.database).await.or_internal_server_error("Failed to retrieve user")? == 0 {
        let new_user = entity::user::ActiveModel {
            email: sea_orm::Set(email.to_string()),
            first_name: sea_orm::Set(first_name.to_string()),
            last_name: sea_orm::Set(last_name.to_string()),
            password: Default::default(),
            ..Default::default()
        }
    }

    cookies.add(
        Cookie::build((
            "authorization",
            format!("Token {}", token_response.access_token().secret()),
        ))
        .http_only(true)
        .path("/")
        .build(),
    );

    Ok(Redirect::to("/"))
}
