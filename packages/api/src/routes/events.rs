use crate::server;
use chrono::{NaiveDate, NaiveTime};
use dioxus::{fullstack::NoContent, prelude::*};
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

#[delete("/api/event/", ext: Extension<server::AppState>)]
pub async fn delete_event(event_id: i32) -> Result<NoContent, ServerFnError> {
    use entity::event::Entity as Event;
    use sea_orm::EntityTrait;

    let delete_result = Event::delete_by_id(event_id)
        .exec(&ext.database)
        .await
        .or_internal_server_error("Error deleting user")?;

    (delete_result.rows_affected == 1).or_not_found("User not found")?;

    Ok(NoContent)
}

pub struct PartialTimeModel {
    pub date: NaiveDate,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub weekday: entity::event::Weekday,
}

//will fix later, doesnt remove waring cause of macro??
/*
#[allow(clippy::too_many_arguments)]
#[post("/api/events", ext: Extension<server::AppState>)]
pub async fn create_event(
    title: String,
    reocurring: bool,
    private: bool,
    desc: Option<String>,
    loc: Option<String>,
    date: NaiveDate,
    start_time: NaiveTime,
    end_time: NaiveTime,
    weekday: entity::event::Weekday,
    owner: i32,
) -> Result<entity::event::Model, ServerFnError> {
    use sea_orm::{ActiveModelTrait, TryIntoModel};

    let event = entity::event::ActiveModel {
        title: sea_orm::Set(title),
        reocurring: sea_orm::Set(reocurring),
        is_private: sea_orm::Set(private),
        desc: sea_orm::Set(desc),
        location: sea_orm::Set(loc),
        date: sea_orm::Set(date),
        start_time: sea_orm::Set(start_time),
        end_time: sea_orm::Set(end_time),
        weekday: sea_orm::Set(weekday),
        owner_id: sea_orm::Set(owner),
        ..Default::default()
    };

    let event = event
        .save(&ext.database)
        .await
        .or_internal_server_error("Error saving new user to database")?;
    Ok(event
        .try_into_model()
        .or_internal_server_error("Error converting user to model")?)
}
*/
