use dioxus::prelude::*;
use entity::is_in_group::Entity as IsInGroup;
use entity::shared_friend_event;
use entity::shared_group_event::{self, Entity as SharedGroupEvent};
use sea_orm::DatabaseConnection;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

pub async fn is_user_in_group(
    db: &DatabaseConnection,
    group_id: i32,
    user_id: i32,
) -> Result<bool, ServerFnError> {
    Ok(IsInGroup::find()
        .filter(entity::is_in_group::Column::GroupId.eq(group_id))
        .filter(entity::is_in_group::Column::UserId.eq(user_id))
        .one(db)
        .await
        .or_internal_server_error("Error loading from database")?
        .is_some())
}

pub async fn is_event_in_group(
    db: &DatabaseConnection,
    group_id: i32,
    event_id: i32,
) -> Result<bool, ServerFnError> {
    Ok(SharedGroupEvent::find()
        .filter(entity::shared_group_event::Column::GroupId.eq(group_id))
        .filter(entity::shared_group_event::Column::EventId.eq(event_id))
        .one(db)
        .await
        .or_internal_server_error("Error loading from database")?
        .is_some())
}

pub async fn can_invite_user_to_event(
    db: &DatabaseConnection,
    event_id: i32,
    user_id: i32,
) -> Result<bool, ServerFnError> {
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, SelectExt};

    let bool1 = entity::shared_friend_event::Entity::find()
        .filter(entity::shared_friend_event::Column::EventId.eq(event_id))
        .filter(entity::shared_friend_event::Column::UserId.eq(user_id))
        .exists(db)
        .await
        .or_internal_server_error("Error loading from database")?;

    let bool2 = entity::invitation::Entity::find()
        .filter(entity::invitation::Column::EventId.eq(event_id))
        .filter(entity::invitation::Column::RecievingUser.eq(user_id))
        .exists(db)
        .await
        .or_internal_server_error("Error loading from database")?;

    if bool1 || bool2 {
        return Ok(false);
    }

    Ok(true)
}

pub async fn remove_shared_event_members(
    event_id: i32,
    db: &DatabaseConnection,
) -> Result<(), ServerFnError> {
    shared_friend_event::Entity::delete_many()
        .filter(shared_friend_event::Column::EventId.eq(event_id))
        .exec(db)
        .await
        .or_internal_server_error("Error deleting event shares")?;

    Ok(())
}

pub async fn remove_shared_event_groups(
    event_id: i32,
    db: &DatabaseConnection,
) -> Result<(), ServerFnError> {
    SharedGroupEvent::delete_many()
        .filter(shared_group_event::Column::EventId.eq(event_id))
        .exec(db)
        .await
        .or_internal_server_error("Error deleting event shares")?;

    Ok(())
}

pub async fn remove_event_invites(
    event_id: i32,
    db: &DatabaseConnection,
) -> Result<(), ServerFnError> {
    entity::invitation::Entity::delete_many()
        .filter(entity::invitation::Column::EventId.eq(event_id))
        .exec(db)
        .await
        .or_internal_server_error("Error deleting event shares")?;

    Ok(())
}

pub async fn remove_group_events(
    group_id: i32,
    db: &DatabaseConnection,
) -> Result<(), ServerFnError> {
    SharedGroupEvent::delete_many()
        .filter(shared_group_event::Column::GroupId.eq(group_id))
        .exec(db)
        .await
        .or_internal_server_error("Error deleting shared events")?;

    Ok(())
}
