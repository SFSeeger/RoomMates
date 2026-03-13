use crate::server::AppState;
use crate::server::auth::find_user_by_session;
use crate::server::auth::oidc::get_user_from_authorization_token;
use anyhow::anyhow;
use dioxus::fullstack::Cookie;
use dioxus::fullstack::axum::middleware::Next;
use dioxus::fullstack::extract::Request;
use dioxus::fullstack::headers::HeaderMapExt;
use dioxus::fullstack::response::Response;
use dioxus::prelude::*;
use entity::prelude::Session;
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
pub async fn authentication_middleware(mut request: Request, next: Next) -> Response {
    let mut authentication_state = AuthenticationState {
        user: None,
        session_id: None,
    };

    let app_state = request.extensions().get::<AppState>().unwrap();
    let database = &app_state.database;

    let cookies = request.extensions().get::<Cookies>().unwrap();

    if let Some(token) = request.headers().get("Authorization")
        && let Ok(token) = token.to_str()
        && let Some(token) = token.strip_prefix("Bearer ")
        && let Ok(user) = get_user_from_authorization_token(token, app_state).await
    {
        authentication_state.user = user;
    } else if let Some(cookie) = cookies.get("session")
        && let Ok(Some((user, session_id))) = find_user_by_session(cookie.value(), database).await
    {
        authentication_state.user = Some(user);
        authentication_state.session_id = Some(session_id);
    } else if let Some(cookie) = cookies.get("authorization")
        && let Some(token) = cookie.value().strip_prefix("Bearer ")
        && let Ok(user) = get_user_from_authorization_token(token, app_state).await
    {
        authentication_state.user = user;
    }

    request.extensions_mut().insert(authentication_state);

    next.run(request).await
}
