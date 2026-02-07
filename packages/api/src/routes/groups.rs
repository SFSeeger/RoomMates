use crate::dioxus_fullstack::NoContent;
use crate::routes::users;
use crate::routes::users::UserInfo;
use crate::server;
use dioxus::prelude::*;
use entity::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use dioxus::server::axum::Extension;

#[post("/api/groups", ext: Extension<server::AppState>)]
pub async fn create_group(group_name: String) -> Result<NoContent, ServerFnError> {
    use entity::is_in_group;
    use sea_orm::{ActiveModelTrait, Set};

    let group = entity::group::ActiveModel {
        name: Set(group_name),
        ..Default::default()
    };

    let group = group
        .insert(&ext.database)
        .await
        .or_internal_server_error("Error saving new group to database")?;

    let user = users::get_me()
        .await
        .or_internal_server_error("Error loading user")?;

    let _pair = is_in_group::ActiveModel {
        user_id: Set(user.id),
        group_id: Set(group.id),
    };

    Ok(NoContent)
}

#[get("/api/groups", ext: Extension<server::AppState>, auth: Extension<server::AuthenticationState>)]
pub async fn list_groups() -> Result<Vec<entity::group::Model>, ServerFnError> {
    use sea_orm::ModelTrait;

    let user = auth.user.as_ref().or_unauthorized("Not authenticated")?;
    let groups = user
        .find_related(Group)
        .all(&ext.database)
        .await
        .or_internal_server_error("Error loading groups")?;

    Ok(groups)
}

#[get("/api/groups/{group_id}/is_user_in_group", ext: Extension<server::AppState>)]
pub async fn is_user_in_group(group_id: i32, user_id: i32) -> Result<bool, ServerFnError> {
    use entity::is_in_group::Entity as IsInGroup;
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

    Ok(IsInGroup::find()
        .filter(entity::is_in_group::Column::GroupId.eq(group_id))
        .filter(entity::is_in_group::Column::UserId.eq(user_id))
        .one(&ext.database)
        .await
        .or_internal_server_error("Error loading from database")?
        .is_some())
}

/// Adds an user to a group
#[post("/api/groups/{group_id}/add_user", ext: Extension<server::AppState>)]
pub async fn add_user_to_group(group_id: i32, email: String) -> Result<NoContent, ServerFnError> {
    use entity::is_in_group;
    use entity::user::Entity as User;
    use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

    let user = User::find()
        .filter(entity::user::Column::Email.eq(email))
        .one(&ext.database)
        .await
        .or_internal_server_error("Error loading user from database")?
        .or_not_found("User not found")?;

    let checker = is_user_in_group(group_id, user.id)
        .await
        .or_internal_server_error("Error checking if user is in group")?;

    if checker {
        return Err(ServerFnError::ServerError {
            message: "User not in group".to_string(),
            code: 409,
            details: None,
        });
    };

    let pair = is_in_group::ActiveModel {
        user_id: Set(user.id),
        group_id: Set(group_id),
    };

    let _pair: is_in_group::ActiveModel = pair
        .insert(&ext.database)
        .await
        .or_internal_server_error("Error inserting pair into database")?
        .into();

    Ok(NoContent)
}

///Deletes an user from a group
#[post("/api/groups/{group_id}/remove_user", ext: Extension<server::AppState>)]
pub async fn remove_user_from_group(
    group_id: i32,
    user_id: i32,
) -> Result<NoContent, ServerFnError> {
    use entity::is_in_group;
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

    let result = is_in_group::Entity::delete_many()
        .filter(is_in_group::Column::UserId.eq(user_id))
        .filter(is_in_group::Column::GroupId.eq(group_id))
        .exec(&ext.database)
        .await
        .or_internal_server_error("Error deleting relation")?;

    (result.rows_affected > 0).or_not_found(format!(
        "Failed to remove user {user_id} from group {group_id}"
    ))?;

    Ok(NoContent)
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GroupCardData {
    pub name: String,
    pub members: Vec<UserInfo>,
    pub events: Vec<entity::event::Model>,
}

///returns default struct of GroupCardData when trying to call a group which does not exist
#[get("/api/groups/{group_id}", ext: Extension<server::AppState>)]
pub async fn retrieve_group(group_id: i32) -> Result<GroupCardData, ServerFnError> {
    use sea_orm::EntityTrait;
    use sea_orm::ModelTrait;

    let groups = Group::find_by_id(group_id)
        .one(&ext.database)
        .await
        .or_not_found("Group not found")?
        .or_internal_server_error("Error loading group from database")?;

    let name = &groups.name;
    let member_info = groups
        .find_related(User)
        .all(&ext.database)
        .await
        .or_internal_server_error("Error loading members from database")?;
    let mut members = Vec::new();
    for member in member_info {
        members.push(UserInfo {
            id: member.id,
            email: member.email,
            first_name: member.first_name,
            last_name: member.last_name,
        })
    }

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

#[post("/api/groups/{group_id}/change_group_name", ext: Extension<server::AppState>)]
pub async fn change_group_name(
    group_id: i32,
    group_name_new: String,
) -> Result<NoContent, ServerFnError> {
    use entity::group;
    use entity::group::Entity as Group;
    use sea_orm::{ActiveModelTrait, EntityTrait, Set};

    let group = Group::find_by_id(group_id)
        .one(&ext.database)
        .await
        .or_not_found("Group not found")
        .or_internal_server_error("Error loading Group")?;

    let mut group: group::ActiveModel = group.unwrap().into();

    group.name = Set(group_name_new.to_owned());

    group
        .update(&ext.database)
        .await
        .or_internal_server_error("Error updating database")?;

    Ok(NoContent)
}
