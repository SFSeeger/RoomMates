use crate::dioxus_fullstack::NoContent;
use crate::server;
use dioxus::prelude::*;
use entity::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use dioxus::server::axum::Extension;

#[get("/api/groups", ext: Extension<server::AppState>)]
pub async fn list_groups() -> Result<Vec<entity::group::Model>, ServerFnError> {
    use sea_orm::EntityTrait;
    use sea_orm::ModelTrait;

    let user = User::find_by_id(1)
        .one(&ext.database)
        .await
        .or_internal_server_error("Error loading user from database")?
        .or_not_found("User not found")?;
    let groups = user
        .find_related(Group)
        .all(&ext.database)
        .await
        .or_internal_server_error("Error loading groups")?;

    Ok(groups)
}

#[post("/api/groups", ext: Extension<server::AppState>)]
pub async fn add_user_to_table(user_id: i32, group_id: i32) -> Result<NoContent, ServerFnError> {
    use entity::is_in_group;
    use sea_orm::{ActiveModelTrait, Set};

    let pair = is_in_group::ActiveModel {
        user_id: Set(user_id),
        group_id: Set(group_id),
    };

    let _pair: is_in_group::ActiveModel = pair
        .insert(&ext.database)
        .await
        .or_internal_server_error("Error inserting pair into database")?
        .into();

    Ok(NoContent)
}

#[post("/api/groups/{group_id}/add_user", ext: Extension<server::AppState>)]
pub async fn add_user_to_group(email: i32, group_id: i32) -> Result<NoContent, ServerFnError> {
    use entity::user::Entity as User;
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

    let user = User::find()
        .filter(entity::user::Column::Email.eq(email))
        .one(&ext.database)
        .await
        .or_internal_server_error("Error loading user from database")?
        .or_not_found("User not found")?;

    add_user_to_table(user.id, group_id).await?;

    Ok(NoContent)
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct GroupCardData {
    pub name: String,
    pub members: Vec<entity::user::Model>,
    pub events: Vec<entity::event::Model>,
}

///returns default struct of GroupCardData when trying to call a group which does not exist
#[get("/api/groups", ext: Extension<server::AppState>)]
pub async fn retrieve_group(group_id: i32) -> Result<GroupCardData, ServerFnError> {
    use sea_orm::EntityTrait;
    use sea_orm::ModelTrait;

    let groups = Group::find_by_id(group_id)
        .one(&ext.database)
        .await
        .or_not_found("Group not found")?
        .or_internal_server_error("Error loading group from database")?;

    let name = &groups.name;
    let members = groups
        .find_related(User)
        .all(&ext.database)
        .await
        .or_internal_server_error("Error loading members from database")?;
    let events = groups
        .find_related(Event)
        .all(&ext.database)
        .await
        .or_internal_server_error("Error loading events from database")?;
    let group_data = GroupCardData {
        name: name.to_string(),
        members,
        events,
    };
    Ok(group_data)
}
