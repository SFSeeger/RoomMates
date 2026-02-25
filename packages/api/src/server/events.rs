use dioxus::prelude::*;
use entity::is_in_group::Entity as IsInGroup;
use entity::shared_group_event::Entity as SharedGroupEvent;
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
