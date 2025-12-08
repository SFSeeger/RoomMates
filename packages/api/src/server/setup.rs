use super::*;
use dioxus::core::Element;
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

    let app_state = AppState { database };
    let router = axum::Router::new()
        .serve_dioxus_application(ServeConfig::default().enable_out_of_order_streaming(), app)
        .layer(Extension(app_state));

    Ok(router)
}
#[derive(Clone)]
pub struct AppState {
    pub database: DatabaseConnection,
}
