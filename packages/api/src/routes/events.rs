use crate::server;
#[cfg(feature = "server")]
use dioxus::server::axum::Extension;
use dioxus::{fullstack::NoContent, prelude::*};
use entity::event::PartialEventModel;
use entity::links::EventUserMembers;
use entity::prelude::*;

pub mod invitations;

#[get("/api/events?mindate&maxdate", ext: Extension<server::AppState>, auth: Extension<server::AuthenticationState>)]
pub async fn list_events(
    mindate: Option<time::Date>,
    maxdate: Option<time::Date>,
) -> Result<Vec<entity::event::FullEvent>, ServerFnError> {
    use entity::event::Column as EventColumn;
    use entity::group::Column as GroupColumn;
    use entity::invitation::Column as InvitationColumn;
    use entity::shared_group_event::Column as GroupEventColumn;
    use sea_orm::{
        ColumnTrait, Condition, EntityTrait, JoinType, ModelTrait, QueryFilter, QueryOrder,
        QuerySelect, QueryTrait, RelationTrait,
    };

    let user = auth.user.as_ref().or_unauthorized("Not authenticated")?;

    let events = entity::prelude::Event::find()
        .join_as(
            JoinType::LeftJoin,
            entity::invitation::Relation::Event.def().rev(),
            "invitation",
        )
        .join_as(
            JoinType::LeftJoin,
            entity::shared_group_event::Relation::Event.def().rev(),
            "shared_group_event",
        )
        .column_as(InvitationColumn::Id.is_not_null(), "is_shared_with_user")
        .column_as(GroupEventColumn::GroupId.is_not_null(), "is_group_event")
        .filter(
            Condition::any()
                .add(EventColumn::OwnerId.eq(user.id))
                .add(
                    Condition::all()
                        .add(InvitationColumn::RecievingUser.eq(user.id))
                        .add(
                            InvitationColumn::Status
                                .eq(entity::invitation::InvitationStatus::Accepted),
                        ),
                )
                .add(
                    Condition::all()
                        .add(
                            GroupEventColumn::GroupId.in_subquery(
                                user.find_related(Group)
                                    .select_only()
                                    .column(GroupColumn::Id)
                                    .into_query(),
                            ),
                        )
                        .add(EventColumn::Private.eq(false)),
                ),
        )
        .apply_if(mindate, |query, v: time::Date| {
            let weekday_num = v.weekday().number_from_monday();
            query.filter(
                Condition::any()
                    .add(
                        Condition::all()
                            .add(EventColumn::Date.gte(v))
                            .add(EventColumn::Reoccurring.eq(false)),
                    )
                    .add(
                        Condition::all()
                            .add(EventColumn::Reoccurring.eq(true))
                            .add(EventColumn::Weekday.gte(weekday_num)),
                    ),
            )
        })
        .apply_if(maxdate, |query, v| {
            let weekday_num = v.weekday().number_from_monday();
            query.filter(
                Condition::any()
                    .add(
                        Condition::all()
                            .add(EventColumn::Date.lte(v))
                            .add(EventColumn::Reoccurring.eq(false)),
                    )
                    .add(
                        Condition::all()
                            .add(EventColumn::Reoccurring.eq(true))
                            .add(EventColumn::Weekday.lte(weekday_num)),
                    ),
            )
        })
        .order_by_asc(EventColumn::StartTime)
        .order_by_asc(EventColumn::EndTime)
        .distinct()
        .into_model()
        .all(&ext.database)
        .await
        .inspect_err(|e| error!("Error loading Events for user: {e}"))
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

#[delete("/api/events/{event_id}", ext: Extension<server::AppState>, auth: Extension<server::AuthenticationState>)]
pub async fn delete_event(event_id: i32) -> Result<NoContent, ServerFnError> {
    use crate::server::events::{
        remove_event_invites, remove_shared_event_groups, remove_shared_event_members,
    };
    use entity::event::Entity as Event;
    use sea_orm::EntityTrait;

    let user_id = auth.user.as_ref().or_unauthorized("Not authenticated")?.id;

    let event = Event::find_by_id(event_id)
        .one(&ext.database)
        .await
        .or_internal_server_error("Error loading event from database")?
        .or_not_found("event not found")?;

    (event.owner_id == user_id).or_unauthorized("Unauthorized to delete this event")?;

    remove_event_invites(event_id, &ext.database).await?;
    remove_shared_event_groups(event_id, &ext.database).await?;
    remove_shared_event_members(event_id, &ext.database).await?;

    let delete_result = Event::delete_by_id(event_id)
        .exec(&ext.database)
        .await
        .or_internal_server_error("Error deleting event")?;

    (delete_result.rows_affected == 1).or_not_found("Event not found")?;

    Ok(NoContent)
}

#[post("/api/events", ext: Extension<server::AppState>, auth: Extension<server::AuthenticationState>)]
pub async fn create_event(
    info: PartialEventModel,
    group: Option<i32>,
) -> Result<entity::event::Model, ServerFnError> {
    use sea_orm::{ActiveModelTrait, TransactionError, TransactionTrait, TryIntoModel};
    let user_id = auth.user.as_ref().or_unauthorized("Not authenticated")?.id;

    let is_user_in_group = if let Some(group_id) = group {
        crate::server::events::is_user_in_group(&ext.database, group_id, user_id)
            .await
            .or_forbidden("No permission to add event to this group")?
    } else {
        false
    };

    ext.database
        .transaction::<_, entity::event::Model, ServerFnError>(|txn| {
            Box::pin(async move {
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
                    owner_id: sea_orm::Set(user_id),
                    ..Default::default()
                };

                let event = event
                    .save(txn)
                    .await
                    .or_internal_server_error("Error saving new event to database")?
                    .try_into_model()
                    .or_internal_server_error("Error converting event to model")?;

                if let Some(group_id) = group
                    && is_user_in_group
                {
                    let pair = entity::shared_group_event::ActiveModel {
                        group_id: sea_orm::Set(group_id),
                        event_id: sea_orm::Set(event.id),
                    };

                    pair.insert(txn)
                        .await
                        .or_internal_server_error("Error inserting pair into database")?;
                }
                Ok(event)
            })
        })
        .await
        .map_err(|error| {
            error!("{error}");
            match error {
                TransactionError::Connection(_) => ServerFnError::ServerError {
                    message: String::from("Error creating event"),
                    code: 500,
                    details: None,
                },
                TransactionError::Transaction(error) => error,
            }
        })
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

    (owner == user.id).or_unauthorized("Unauthorized to edit event")?;

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

#[get("/api/events/{event_id}/groups", ext: Extension<server::AppState>)]
pub async fn list_event_groups(event_id: i32) -> Result<Vec<entity::group::Model>, ServerFnError> {
    use entity::event::Entity as Event;
    use sea_orm::{EntityTrait, ModelTrait};

    let event = Event::find_by_id(event_id)
        .one(&ext.database)
        .await
        .or_internal_server_error("Error loading event from database")?
        .or_not_found("Event not found")?;

    Ok(event
        .find_related(Group)
        .all(&ext.database)
        .await
        .or_internal_server_error("Error loading events from database")?)
}

#[put("/api/events/{event_id}/groups", ext: Extension<server::AppState>, auth: Extension<server::AuthenticationState>)]
pub async fn add_event_to_group(event_id: i32, group_id: i32) -> Result<NoContent, ServerFnError> {
    use crate::server::events::is_event_in_group;
    use crate::server::events::is_user_in_group;
    use entity::event::Entity as Event;
    use entity::shared_group_event;
    use sea_orm::{ActiveModelTrait, EntityTrait, Set};

    let user = auth.user.as_ref().or_unauthorized("Not authenticated")?;

    is_user_in_group(&ext.database, group_id, user.id)
        .await?
        .or_forbidden("No permission to add a new user.")?;

    let new_event = Event::find_by_id(event_id)
        .one(&ext.database)
        .await
        .or_internal_server_error("Error loading event from database")?
        .or_not_found("Event not found")?;

    let checker = is_event_in_group(&ext.database, group_id, new_event.id).await?;

    (!checker).or_bad_request("Event already in group")?;

    let pair = shared_group_event::ActiveModel {
        group_id: Set(group_id),
        event_id: Set(new_event.id),
    };

    let _pair = pair
        .insert(&ext.database)
        .await
        .or_internal_server_error("Error inserting pair into database")?;

    Ok(NoContent)
}

#[post("/api/events/{event_id}/groups/remove-group", ext: Extension<server::AppState>, auth: Extension<server::AuthenticationState>)]
pub async fn remove_event_from_group(
    group_id: i32,
    event_id: i32,
) -> Result<NoContent, ServerFnError> {
    use crate::server::events::{is_event_in_group, is_user_in_group};
    use entity::shared_group_event;
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

    let user = auth.user.as_ref().or_unauthorized("Not authenticated")?;
    is_user_in_group(&ext.database, group_id, user.id)
        .await?
        .or_forbidden("Permission to remove group denied!")?;
    is_event_in_group(&ext.database, group_id, event_id)
        .await?
        .or_not_found("Event does not exist")?;

    let result = shared_group_event::Entity::delete_many()
        .filter(shared_group_event::Column::EventId.eq(event_id))
        .filter(shared_group_event::Column::GroupId.eq(group_id))
        .exec(&ext.database)
        .await
        .or_internal_server_error("Error deleting relation")?;

    (result.rows_affected > 0).or_not_found(format!(
        "Failed to remove event {event_id} from group {group_id}"
    ))?;

    //delete_event(event_id).await?;

    Ok(NoContent)
}

#[get("/api/events/{event_id}/members", ext: Extension<server::AppState>)]
pub async fn list_event_members(event_id: i32) -> Result<Vec<entity::user::Model>, ServerFnError> {
    use sea_orm::EntityTrait;
    use sea_orm::ModelTrait;

    let event = entity::event::Entity::find_by_id(event_id)
        .one(&ext.database)
        .await
        .or_internal_server_error("could not load event")?
        .or_not_found("could not find event")?;

    let owner = entity::user::Entity::find_by_id(event.owner_id)
        .one(&ext.database)
        .await
        .or_internal_server_error("could not load event")?
        .or_not_found("could not find event")?;

    let mut shares = event
        .find_linked(EventUserMembers)
        .all(&ext.database)
        .await
        .or_internal_server_error("failed to retrieve other members")?;

    shares.push(owner);

    Ok(shares)
}

#[delete("/api/events/{event_id}/leave", ext: Extension<server::AppState>, auth: Extension<server::AuthenticationState>)]
pub async fn leave_event(event_id: i32) -> Result<NoContent, ServerFnError> {
    use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter};

    let user = auth.user.as_ref().or_unauthorized("Not authenticated")?;

    let shared_event = entity::shared_friend_event::Entity::find()
        .filter(entity::shared_friend_event::Column::EventId.eq(event_id))
        .filter(entity::shared_friend_event::Column::UserId.eq(user.id))
        .one(&ext.database)
        .await
        .or_internal_server_error("Failed to retrieve info from server")?
        .or_not_found("user is not in this event")?;

    shared_event
        .delete(&ext.database)
        .await
        .or_internal_server_error("could not leave event")?;

    Ok(NoContent)
}
