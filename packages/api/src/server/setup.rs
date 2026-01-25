use super::*;
use crate::server::auth::find_user_by_session;
use dioxus::core::Element;
use dioxus::fullstack::Cookie;
use dioxus::fullstack::axum::middleware::Next;
use dioxus::fullstack::extract::Request;
use dioxus::fullstack::headers::HeaderMapExt;
use dioxus::fullstack::response::Response;
use dioxus::prelude::*;
use dioxus::server::axum;
use dioxus::server::axum::Extension;
use sea_orm::DatabaseConnection;

pub async fn setup_api(app: fn() -> Element) -> Result<axum::Router, anyhow::Error> {
    let database: DatabaseConnection = database::establish_connection().await?;

    // TODO: For the start of the project this is the simplest way to keep the DB in sync. At some point we should switch to migrations tho
    database
        .get_schema_registry("entity::*")
        .sync(&database)
        .await?;

    let database_clone = database.clone();

    let app_state = AppState {
        database: database_clone,
    };
    let router = axum::Router::new()
        .serve_dioxus_application(ServeConfig::default().enable_out_of_order_streaming(), app)
        .layer(Extension(app_state))
        .layer(axum::middleware::from_fn(
            move |request: Request, next: Next| {
                authentication_middleware(request, next, database.clone())
            },
        ));

    Ok(router)
}

#[derive(Clone)]
pub struct AppState {
    pub database: DatabaseConnection,
}

async fn authentication_middleware(
    mut request: Request,
    next: Next,
    database: DatabaseConnection,
) -> Response {
    let mut authentication_state = AuthenticationState { user: None };

    if let Some(token) = request.headers().get("Authorization") {
        if let Ok(token) = token.to_str()
            && let Some(token) = token.strip_prefix("Token ")
            && let Ok(Some(user)) = find_user_by_session(token, &database).await
        {
            authentication_state.user = Some(user);
        };
    } else if let Some(cookies) = request.headers().typed_get::<Cookie>()
        && let Some(token) = cookies.get("session")
        && let Ok(Some(user)) = find_user_by_session(token, &database).await
    {
        authentication_state.user = Some(user);
    }

    request.extensions_mut().insert(authentication_state);

    next.run(request).await
}

#[derive(Clone, Debug)]
pub struct AuthenticationState {
    pub user: Option<entity::user::Model>,
}

impl AuthenticationState {
    pub fn is_authenticated(&self) -> bool {
        self.user.is_some()
    }

    pub fn is_anonymous(&self) -> bool {
        self.user.is_none()
    }
}
