#[cfg(feature = "server")]
use crate::server;
use dioxus::fullstack::Redirect;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use dioxus::server::axum::Extension;

const OIDC_SESSION_KEY: &str = "oidc_metadata";
#[allow(clippy::unused_async)]
#[get("/api/oidc/login", state: Extension<server::AppState>,  session: Extension<tower_sessions::Session> )]
pub async fn oauth_login() -> Result<Redirect, ServerFnError> {
    use crate::server::auth::oidc;

    let oidc_config = state
        .oidc_config
        .as_ref()
        .or_internal_server_error("OAuth Client Missing or disabled!")?;
    let metadata = oidc::create_oidc_challenge(&oidc_config.client);

    let redirect_url = metadata.url.as_str().to_string();
    let oidc_session: oidc::OidcSession = metadata.into();

    session
        .insert(OIDC_SESSION_KEY, oidc_session)
        .await
        .or_internal_server_error("Failed to create session")?;

    Ok(Redirect::to(&redirect_url))
}

#[get("/api/oidc/redirect?state&code",
    ext: Extension<server::AppState>,
    cookies: Extension<tower_cookies::Cookies>,
    session: Extension<tower_sessions::Session>
)]
pub async fn oauth_redirect(state: String, code: String) -> Result<Redirect, ServerFnError> {
    use crate::server::auth::oidc;
    use entity::prelude::*;
    use openidconnect::{AccessTokenHash, OAuth2TokenResponse, TokenResponse};
    use sea_orm::prelude::*;
    use crate::server::auth::oidc::add_oidc_cookies;

    let oidc_client = ext
        .oidc_config
        .as_ref()
        .map(|c| &c.client)
        .or_internal_server_error("Oidc not initialized / enabled")?;

    let oidc_session: oidc::OidcSession = session
        .get(OIDC_SESSION_KEY)
        .await
        .or_internal_server_error("Failed to retrieve session")?
        .or_bad_request("Failed to get session with required metadata")?;

    (*oidc_session.csrf_token.secret() == state).or_bad_request("CSRF Mismatch")?;

    let token_response =
        oidc::verify_oidc_challenge(oidc_client, code, oidc_session.pkce_code_verifier)
            .await
            .inspect_err(|e| error!("{e}"))
            .or_forbidden("Error receiving token")?;

    let id_token = token_response
        .id_token()
        .or_bad_request("Server did not return ID Token")?;
    let id_token_verifier = oidc_client.id_token_verifier();
    let claims = id_token
        .claims(&id_token_verifier, &oidc_session.nonce)
        .or_bad_request("Unable to validate claims")?;

    if let Some(expected_access_token_hash) = claims.access_token_hash() {
        let actual_access_token_hash = AccessTokenHash::from_token(
            token_response.access_token(),
            id_token
                .signing_alg()
                .or_bad_request("Error extracting sining algorithm")?,
            id_token
                .signing_key(&id_token_verifier)
                .or_bad_request("Error extracting signing key")?,
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

    let email: &str = claims
        .email()
        .map(|email| email.as_str())
        .or_bad_request("Email is required")?;
    let first_name: &str = claims
        .given_name()
        .or_bad_request("Missing given name")?
        .get(None)
        .map(|n| n.as_str())
        .or_bad_request("Missing given name")?;
    let last_name: &str = claims
        .family_name()
        .or_bad_request("Missing family name")?
        .get(None)
        .map(|n| n.as_str())
        .or_bad_request("Missing family name")?;

    if User::find_by_email(email)
        .count(&ext.database)
        .await
        .or_internal_server_error("Failed to retrieve user")?
        == 0
    {
        let new_user = entity::user::ActiveModel {
            email: sea_orm::Set(email.to_string()),
            first_name: sea_orm::Set(first_name.to_string()),
            last_name: sea_orm::Set(last_name.to_string()),
            password: sea_orm::Set(None),
            is_oidc_user: sea_orm::Set(true),
            ..Default::default()
        };
        new_user
            .insert(&ext.database)
            .await
            .inspect_err(|e| error!("{e}"))
            .or_internal_server_error("Failed to create user")?;
    }

    add_oidc_cookies(&cookies, &token_response).or_internal_server_error("Failed to add cookies")?;

    Ok(Redirect::to("/"))
}

#[post("/api/oidc/refresh", state: Extension<server::AppState>, cookies: Extension<tower_cookies::Cookies>)]
pub async fn refresh_authorization_token() -> Result<(), ServerFnError> {
    use crate::server::auth::oidc::add_oidc_cookies;
    let refresh_token_cookie = cookies.get("refresh_token").or_bad_request("Invalid refresh token")?;
    let refresh_token = refresh_token_cookie.value();

    let tokens = server::auth::oidc::refresh_authorization_token(refresh_token, &state).await.or_internal_server_error("Failed to refresh token")?;

    add_oidc_cookies(&cookies, &tokens).or_internal_server_error("Failed to set cookies")?;
    Ok(())
}