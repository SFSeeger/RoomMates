use super::*;
use crate::dioxus_fullstack::routing::Router;
use dioxus::core::Element;
use dioxus::server::axum::Extension;
use sea_orm::DatabaseConnection;

pub async fn setup_api(app: fn() -> Element) -> Result<Router, anyhow::Error> {
    let database: DatabaseConnection = database::establish_connection().await?;

    // TODO: For the start of the project this is the simplest way to keep the DB in sync. At some point we should switch to migrations tho
    database
        .get_schema_registry("entity::*")
        .sync(&database)
        .await?;

    let app_state = AppState { database };
    let router = dioxus::server::router(app).layer(Extension(app_state));

    Ok(router)
}
#[derive(Clone)]
pub struct AppState {
    pub database: DatabaseConnection,
}
