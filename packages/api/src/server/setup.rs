use super::*;
use crate::server::auth::find_user_by_session;
use crate::server::auth::oidc::{
    OidcConfig, create_oidc_config, jwks_refresh_loop, validate_authorization_token,
};
use anyhow::anyhow;
use dioxus::core::Element;
use dioxus::fullstack::Cookie;
use dioxus::fullstack::axum::middleware::Next;
use dioxus::fullstack::extract::Request;
use dioxus::fullstack::headers::HeaderMapExt;
use dioxus::fullstack::response::Response;
use dioxus::prelude::*;
use dioxus::server::axum;
use dioxus::server::axum::Extension;
use entity::prelude::Session;
use sea_orm::{DatabaseConnection, EntityTrait};
use std::env;
use time::Duration;
use tower_cookies::CookieManagerLayer;
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};

pub async fn setup_api(app: fn() -> Element) -> Result<axum::Router, anyhow::Error> {
    let database: DatabaseConnection = database::establish_connection().await?;

    // TODO: For the start of the project this is the simplest way to keep the DB in sync. At some point we should switch to migrations tho
    database
        .get_schema_registry("entity::*")
        .sync(&database)
        .await?;

    let database_clone = database.clone();
    let oidc_config = if env::var("OAUTH_ENABLED")?.trim().to_lowercase() == "true" {
        let config = create_oidc_config().await?;
        tokio::spawn(jwks_refresh_loop(config.jwks_state.clone()));
        Some(config)
    } else {
        None
    };

    let app_state = AppState {
        database: database_clone,
        oidc_config,
    };

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(!cfg!(debug_assertions))
        .with_expiry(Expiry::OnInactivity(Duration::seconds(60)));

    let router = axum::Router::new()
        .serve_dioxus_application(ServeConfig::default().enable_out_of_order_streaming(), app)
        .layer(CookieManagerLayer::new())
        .layer(session_layer)
        .layer(axum::middleware::from_fn(tracing_middleware))
        .layer(axum::middleware::from_fn(
            move |request: Request, next: Next| {
                authentication_middleware(request, next, database.clone())
            },
        ))
        .layer(Extension(app_state));

    Ok(router)
}

#[derive(Clone)]
pub struct AppState {
    pub database: DatabaseConnection,
    pub oidc_config: Option<OidcConfig>,
}

async fn authentication_middleware(
    mut request: Request,
    next: Next,
    database: DatabaseConnection,
) -> Response {
    let mut authentication_state = AuthenticationState {
        user: None,
        session_id: None,
    };

    if let Some(token) = request.headers().get("Authorization")
        && let Ok(token) = token.to_str()
        && let Some(token) = token.strip_prefix("Token ")
        && let Ok(Some((user, session_id))) = find_user_by_session(token, &database).await
    {
        authentication_state.user = Some(user);
        authentication_state.session_id = Some(session_id);
    } else if let Some(cookies) = request.headers().typed_get::<Cookie>()
        && let Some(token) = cookies.get("session")
        && let Ok(Some((user, session_id))) = find_user_by_session(token, &database).await
    {
        authentication_state.user = Some(user);
        authentication_state.session_id = Some(session_id);
    } else if let Some(cookies) = request.headers().typed_get::<Cookie>()
        && let Some(token) = cookies.get("authorization")
        && let Some(token) = token.strip_prefix("Token ")
    {
        let app_state = request.extensions().get::<AppState>().unwrap();
        let oidc_config = app_state.oidc_config.as_ref().expect("OIDC is disabled!");

        let claims = validate_authorization_token(&oidc_config.jwks_state, token)
            .await
            .unwrap();
        let user = entity::user::Entity::find_by_email(claims.email)
            .one(&database)
            .await
            .unwrap();

        authentication_state.user = user
    }

    request.extensions_mut().insert(authentication_state);

    next.run(request).await
}

async fn tracing_middleware(request: Request, next: Next) -> Response {
    let method = request.method().clone();
    let path = request.uri().path().to_string();
    let username = request.extensions().get::<AuthenticationState>().map_or(
        "Anonymous".to_string(),
        |state| {
            state
                .user
                .as_ref()
                .map(|user| user.email.clone())
                .unwrap_or("Anonymous".to_string())
        },
    );

    let response = next.run(request).await;

    if !env::var("ACCESS_LOG").is_ok_and(|value| value.to_lowercase() == "true") {
        return response;
    }

    let base_message = format!("{method} {path} {} - {username}", response.status());

    if response.status().is_server_error() {
        error!("{}", base_message);
    } else if response.status().is_client_error() {
        warn!("{}", base_message);
    } else {
        info!("{}", base_message);
    }

    response
}

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
