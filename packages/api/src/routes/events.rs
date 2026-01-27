use crate::server;
#[cfg(feature = "server")]
use dioxus::server::axum::Extension;
use dioxus::{fullstack::NoContent, prelude::*};
use entity::event::PartialEventModel;
use entity::prelude::*;

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

#[get("/api/events/{event_id}", ext: Extension<server::AppState>)]
pub async fn retrieve_event(event_id: i32) -> Result<entity::event::Model, ServerFnError> {
    use entity::event::Entity as Event;
    use sea_orm::EntityTrait;

    let event = Event::find_by_id(event_id)
        .one(&ext.database)
        .await
        .or_internal_server_error("Error loading user from database")?
        .or_not_found("event not found")?;
    Ok(event)
}

#[delete("/api/events/{event_id}", ext: Extension<server::AppState>)]
pub async fn delete_event(event_id: i32) -> Result<NoContent, ServerFnError> {
    use entity::event::Entity as Event;
    use sea_orm::EntityTrait;

    let delete_result = Event::delete_by_id(event_id)
        .exec(&ext.database)
        .await
        .or_internal_server_error("Error deleting event")?;

    (delete_result.rows_affected == 1).or_not_found("Event not found")?;

    Ok(NoContent)
}

#[post("/api/events", ext: Extension<server::AppState>, auth: Extension<server::AuthenticationState>)]
pub async fn create_event(info: PartialEventModel) -> Result<entity::event::Model, ServerFnError> {
    use sea_orm::{ActiveModelTrait, TryIntoModel};
    let user = auth.user.as_ref().or_unauthorized("Not authenticated")?;

    let event = entity::event::ActiveModel {
        title: sea_orm::Set(info.title),
        reoccurring: sea_orm::Set(info.reoccurring),
        private: sea_orm::Set(info.private),
        description: sea_orm::Set(info.description),
        location: sea_orm::Set(info.location),
        date: sea_orm::Set(info.date),
        start_time: sea_orm::Set(info.start_time),
        end_time: sea_orm::Set(info.end_time),
        weekday: sea_orm::Set(info.weekday),
        owner_id: sea_orm::Set(user.id),
        ..Default::default()
    };

    let event = event
        .save(&ext.database)
        .await
        .or_internal_server_error("Error saving new event to database")?;
    Ok(event
        .try_into_model()
        .or_internal_server_error("Error converting event to model")?)
}

#[put("/api/events/{event_id}", ext: Extension<server::AppState>,auth: Extension<server::AuthenticationState>)]
pub async fn update_event(
    event_id: i32,
    data: PartialEventModel,
) -> Result<entity::event::Model, ServerFnError> {
    use sea_orm::{ActiveModelTrait, EntityTrait, TryIntoModel};

    let user = auth.user.as_ref().or_unauthorized("Not authenticated")?;

    let event = entity::event::Entity::find_by_id(event_id)
        .one(&ext.database)
        .await
        .or_internal_server_error("Failed to load event")?
        .or_not_found("event somehow not found")?;

    let owner = event.owner_id;

    (owner == user.id).or_unauthorized("Unauthorized to delete todo list")?;

    let mut active_event: entity::event::ActiveModel = event.into();

    active_event.title = sea_orm::Set(data.title);
    active_event.reoccurring = sea_orm::Set(data.reoccurring);
    active_event.location = sea_orm::Set(data.location);
    active_event.private = sea_orm::Set(data.private);
    active_event.description = sea_orm::Set(data.description);
    active_event.date = sea_orm::Set(data.date);
    active_event.start_time = sea_orm::Set(data.start_time);
    active_event.end_time = sea_orm::Set(data.end_time);
    active_event.weekday = sea_orm::Set(data.weekday);

    active_event.id = sea_orm::Unchanged(event_id);
    active_event.owner_id = sea_orm::Unchanged(owner);

    let event: entity::event::Model = active_event
        .update(&ext.database)
        .await
        .or_internal_server_error("Failed to update event")?;

    Ok(event
        .try_into_model()
        .or_internal_server_error("Error parsing event")?)
}
