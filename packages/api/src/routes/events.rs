use crate::server;
use dioxus::prelude::*;
use entity::prelude::*;

#[cfg(feature = "server")]
use dioxus::server::axum::Extension;

#[get("/api/events", ext: Extension<server::AppState>)]
pub async fn list_events() -> Result<Vec<entity::event::Model>, ServerFnError> {
    use sea_orm::EntityTrait;
    use sea_orm::ModelTrait;

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

#[post("/api/users", ext: Extension<server::AppState>)]
pub async fn create_event(
    email: String,
    password: String,
) -> Result<entity::user::Model, ServerFnError> {
    use sea_orm::{ActiveModelTrait, TryIntoModel};

    let user = entity::user::ActiveModel {
        email: sea_orm::Set(email),
        password: sea_orm::Set(password),
        ..Default::default()
    };

    let user = user
        .save(&ext.database)
        .await
        .or_internal_server_error("Error saving new user to database")?;
    Ok(user
        .try_into_model()
        .or_internal_server_error("Error converting user to model")?)
}
