use crate::server;
use dioxus::prelude::*;
use entity::prelude::*;

#[cfg(feature = "server")]
use dioxus::server::axum::Extension;

#[get("/api/events", ext: Extension<server::AppState>)]
pub async fn list_events() -> Result<Vec<entity::event::Model>, ServerFnError> {
    use sea_orm::EntityTrait;
    use sea_orm::ModelTrait;
    use std::{thread, time::Duration};

    thread::sleep(Duration::from_secs(1)); // Simulate a slow database query

    let user = User::find_by_id(1)
        .one(&ext.database)
        .await
        .or_internal_server_error("Error loading user from database")?
        .or_not_found("User not found")?;
    let events = user
        .find_related(Event)
        .all(&ext.database)
        .await
        .or_internal_server_error("Error loading events")?;

    Ok(events)
}
