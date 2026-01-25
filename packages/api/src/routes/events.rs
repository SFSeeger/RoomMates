use crate::server;
use dioxus::prelude::*;
use entity::prelude::*;

#[cfg(feature = "server")]
use dioxus::server::axum::Extension;

#[get("/api/events", ext: Extension<server::AppState>, auth: Extension<server::AuthenticationState>)]
pub async fn list_events() -> Result<Vec<entity::event::Model>, ServerFnError> {
    use sea_orm::ModelTrait;

    let user = auth.user.as_ref().or_unauthorized("Not authenticated")?;

    let events = user
        .find_related(Event)
        .all(&ext.database)
        .await
        .or_internal_server_error("Error loading events")?;

    Ok(events)
}
