use super::{constants, database};
use crate::server::auth::middleware::authentication_middleware;
use crate::server::auth::oidc::{OidcConfig, create_oidc_config, jwks_refresh_loop};
use crate::server::middleware::tracing_middleware;
use crate::server::utils::{convert_env_to_bool, get_env_or, parse_env_string};
use dioxus::core::Element;
use dioxus::prelude::*;
use dioxus::server::axum;
use dioxus::server::axum::Extension;
use sea_orm::DatabaseConnection;
use time::Duration;
use time::ext::NumericalDuration;
use tower_cookies::CookieManagerLayer;
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};

pub async fn setup_api(app: fn() -> Element) -> Result<axum::Router, anyhow::Error> {
    let database: DatabaseConnection = database::establish_connection().await?;

    // TODO: For the start of the project this is the simplest way to keep the DB in sync. At some point we should switch to migrations tho
    database
        .get_schema_registry("entity::*")
        .sync(&database)
        .await?;

    let oidc_config = if get_env_or(constants::OIDC_ENABLED_ENV_VAR, false, convert_env_to_bool) {
        let config = create_oidc_config().await?;
        let duration: i64 = get_env_or(
            constants::OIDC_JWKS_REFRESH_INTERVAL_ENV_VAR,
            None,
            parse_env_string,
        )
        .map_or(600, |v| v.parse().unwrap_or(600));
        tokio::spawn(jwks_refresh_loop(
            config.jwks_state.clone(),
            Duration::seconds(duration),
        ));
        Some(config)
    } else {
        None
    };

    let app_state = AppState {
        database,
        oidc_config,
    };

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(!cfg!(debug_assertions))
        .with_expiry(Expiry::OnInactivity(60.seconds()));

    let router = axum::Router::new()
        .serve_dioxus_application(ServeConfig::default().enable_out_of_order_streaming(), app)
        .layer(axum::middleware::from_fn(tracing_middleware))
        .layer(axum::middleware::from_fn(authentication_middleware))
        .layer(CookieManagerLayer::new())
        .layer(session_layer)
        .layer(Extension(app_state));

    Ok(router)
}

#[derive(Clone)]
pub struct AppState {
    pub database: DatabaseConnection,
    pub oidc_config: Option<OidcConfig>,
}
