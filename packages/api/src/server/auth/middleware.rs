use crate::server::auth::find_user_by_session;
use crate::server::auth::oidc::{
    add_oidc_cookies, get_user_from_authorization_token, refresh_authorization_token,
};
use crate::server::{AppState, constants};
use anyhow::anyhow;
use dioxus::fullstack::axum::middleware::Next;
use dioxus::fullstack::extract::Request;
use dioxus::fullstack::response::Response;
use dioxus::prelude::*;
use entity::prelude::Session;
use openidconnect::OAuth2TokenResponse;
use sea_orm::{DatabaseConnection, EntityTrait};
use tower_cookies::Cookies;

#[derive(Clone, Debug)]
pub struct AuthenticationState {
    pub user: Option<entity::user::Model>,
    session_id: Option<i32>,
}

impl AuthenticationState {
    #[must_use]
    pub fn is_authenticated(&self) -> bool {
        self.user.is_some()
    }

    #[must_use]
    pub fn is_anonymous(&self) -> bool {
        self.user.is_none()
    }

    /// Logs the authenticated user out
    ///
    /// # Errors
    ///
    /// Returns [`anyhow::Error`] ([`DBError`]) when deleting the session fails
    pub async fn logout(&mut self, database: &DatabaseConnection) -> Result<(), anyhow::Error> {
        let Some(session_id) = self.session_id else {
            return Err(anyhow!("Session ID is None"));
        };
        let delete_result = Session::delete_by_id(session_id).exec(database).await?;
        if delete_result.rows_affected == 0 {
            return Err(anyhow!("Session not found"));
        }
        self.session_id = None;
        self.user = None;
        Ok(())
    }
}

fn extract_bearer(value: &str) -> Option<&str> {
    value.strip_prefix("Bearer ")
}

/// Middleware handling the auth state of a request.
/// Auth gets checked in the following order:\
/// Header (Authorization Token) -> Cookie (Session Token) -> Cookie (Authorization Token) -> Cookie (Refresh Token)
///
/// If the authorization using session and authorization token fails, the session gets refreshed using the OIDC provider.
///
/// # Arguments
///
/// * `request`:  Incoming Request
/// * `next`:  Next middleware in the middleware stack
///
/// returns: Result<Response<Body>, `StatusCode`>
///
/// # Errors
/// * [`StatusCode::INTERNAL_SERVER_ERROR`]: If extracting either cookies or the app state fails
pub async fn authentication_middleware(
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let mut authentication_state = AuthenticationState {
        user: None,
        session_id: None,
    };

    let app_state = request
        .extensions()
        .get::<AppState>()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    let database = &app_state.database;

    let cookies = request
        .extensions()
        .get::<Cookies>()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(token) = request.headers().get("Authorization")
        && let Ok(token) = token.to_str()
        && let Some(token) = extract_bearer(token)
        && let Ok(user) = get_user_from_authorization_token(token, app_state).await
    {
        authentication_state.user = user;
    } else if let Some(cookie) = cookies.get(constants::SESSION_COOKIE_NAME)
        && let Ok(Some((user, session_id))) = find_user_by_session(cookie.value(), database).await
    {
        authentication_state.user = Some(user);
        authentication_state.session_id = Some(session_id);
    } else if let Some(cookie) = cookies.get(constants::OIDC_AUTHORIZATION_COOKIE_NAME)
        && let Some(token) = extract_bearer(cookie.value())
        && let Ok(user) = get_user_from_authorization_token(token, app_state).await
    {
        authentication_state.user = user;
    } else if let Some(cookie) = cookies.get(constants::OIDC_REFRESH_COOKIE_NAME)
        && let Some(refresh_token) = extract_bearer(cookie.value())
    {
        match refresh_authorization_token(refresh_token, app_state).await {
            Ok(token_response) => {
                if let Err(err) = add_oidc_cookies(cookies, &token_response) {
                    error!("Error adding OAuth Cookies: {err}");
                }
                let access_token = token_response.access_token().secret();
                if let Ok(user) = get_user_from_authorization_token(access_token, app_state).await {
                    authentication_state.user = user;
                }
            }
            Err(err) => {
                warn!("Refresh token error: {err}");
            }
        }
    }

    request.extensions_mut().insert(authentication_state);

    Ok(next.run(request).await)
}
