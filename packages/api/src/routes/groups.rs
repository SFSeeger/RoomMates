use crate::dioxus_fullstack::NoContent;
use crate::routes::users::UserInfo;
use crate::server;
use dioxus::prelude::*;
use entity::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use dioxus::server::axum::Extension;

#[post("/api/groups", ext: Extension<server::AppState>, auth: Extension<server::AuthenticationState>)]
pub async fn create_group(group_name: String) -> Result<entity::group::Model, ServerFnError> {
    use entity::is_in_group;
    use sea_orm::{ActiveModelTrait, Set, TransactionError, TransactionTrait};

    ext.database
        .transaction::<_, entity::group::Model, ServerFnError>(|txn| {
            Box::pin(async move {
                let group = entity::group::ActiveModel {
                    name: Set(group_name),
                    ..Default::default()
                };

                let group = group
                    .insert(txn)
                    .await
                    .or_internal_server_error("Error creating group")?;

                let user = auth.user.as_ref().or_unauthorized("Not authenticated")?;

                let pair = is_in_group::ActiveModel {
                    user_id: Set(user.id),
                    group_id: Set(group.id),
                };

                pair.insert(txn)
                    .await
                    .inspect_err(|error| error!("{error:?}"))
                    .or_internal_server_error("Error creating group")?;

                Ok(group)
            })
        })
        .await
        .map_err(|error| {
            error!("{error}");
            match error {
                TransactionError::Connection(_) => ServerFnError::ServerError {
                    message: String::from("Error creating group"),
                    code: 500,
                    details: None,
                },
                TransactionError::Transaction(error) => error,
            }
        })
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

/// Adds an user to a group
#[post("/api/groups/{group_id}/add-user", ext: Extension<server::AppState>, auth: Extension<server::AuthenticationState>)]
pub async fn add_user_to_group(group_id: i32, email: String) -> Result<NoContent, ServerFnError> {
    use crate::routes::groups::server_functions::is_user_in_group;
    use entity::is_in_group;
    use entity::user::Entity as User;
    use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

    let user = auth.user.as_ref().or_unauthorized("Not authenticated")?;

    let authenticated = is_user_in_group(&ext.database, group_id, user.id).await?;
    if authenticated {
        let new_user = User::find()
            .filter(entity::user::Column::Email.eq(email))
            .one(&ext.database)
            .await
            .or_internal_server_error("Error loading user from database")?
            .or_not_found("User not found")?;

        let checker = is_user_in_group(&ext.database, group_id, new_user.id).await?;

        (!checker).or_bad_request("User already in group")?;

        let pair = is_in_group::ActiveModel {
            user_id: Set(new_user.id),
            group_id: Set(group_id),
        };

        let _pair = pair
            .insert(&ext.database)
            .await
            .or_internal_server_error("Error inserting pair into database")?;

        Ok(NoContent)
    } else {
        Err(ServerFnError::ServerError {
            message: "No permission to add a new user.".to_string(),
            code: 401,
            details: None,
        })
    }
}

///Deletes an user from a group
#[post("/api/groups/{group_id}/remove-user", ext: Extension<server::AppState>, auth: Extension<server::AuthenticationState>)]
pub async fn remove_user_from_group(
    group_id: i32,
    user_id: i32,
) -> Result<NoContent, ServerFnError> {
    use crate::routes::groups::server_functions::is_user_in_group;
    use entity::is_in_group;
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

    let user = auth.user.as_ref().or_unauthorized("Not authenticated")?;
    let authenticated = is_user_in_group(&ext.database, group_id, user.id).await?;

    let group = retrieve_group(group_id).await?;
    let group_size = group.members.len();

    if authenticated {
        if group_size > 1 {
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
        } else {
            delete_group(group_id).await?;
            Ok(NoContent)
        }
    } else {
        Err(ServerFnError::ServerError {
            message: "No permission to remove a user from this group.".to_string(),
            code: 401,
            details: None,
        })
    }
}

#[post("/api/groups/{group_id}/remove-group", ext: Extension<server::AppState>, auth: Extension<server::AuthenticationState>)]
pub async fn remove_event_from_group(
    group_id: i32,
    event_id: i32,
) -> Result<NoContent, ServerFnError> {
    use crate::routes::groups::server_functions::{is_event_in_group, is_user_in_group};
    use entity::shared_group_event;
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

    let user = auth.user.as_ref().or_unauthorized("Not authenticated")?;
    let authenticated = is_user_in_group(&ext.database, group_id, user.id).await?;
    let event_exists = is_event_in_group(&ext.database, group_id, event_id).await?;

    if authenticated {
        if event_exists {
            let result = shared_group_event::Entity::delete_many()
                .filter(shared_group_event::Column::EventId.eq(event_id))
                .filter(shared_group_event::Column::GroupId.eq(group_id))
                .exec(&ext.database)
                .await
                .or_internal_server_error("Error deleting relation")?;

            (result.rows_affected > 0).or_not_found(format!(
                "Failed to remove event {event_id} from group {group_id}"
            ))?;

            Ok(NoContent)
        } else {
            Err(ServerFnError::ServerError {
                message: "Event does not exist".to_string(),
                code: 404,
                details: None,
            })
        }
    } else {
        Err(ServerFnError::ServerError {
            message: "No permission to remove an event from this group.".to_string(),
            code: 401,
            details: None,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GroupDetailData {
    pub name: String,
    pub members: Vec<UserInfo>,
    pub events: Vec<entity::event::Model>,
}

///returns default struct of GroupDetailData when trying to call a group which does not exist
#[get("/api/groups/{group_id}", ext: Extension<server::AppState>)]
pub async fn retrieve_group(group_id: i32) -> Result<GroupDetailData, ServerFnError> {
    use sea_orm::EntityTrait;
    use sea_orm::ModelTrait;

    let group = Group::find_by_id(group_id)
        .one(&ext.database)
        .await
        .or_internal_server_error("Error loading group from database")?
        .or_not_found("Group not found")?;

    let members = group
        .find_related(User)
        .all(&ext.database)
        .await
        .or_internal_server_error("Error loading members from database")?
        .into_iter()
        .map(UserInfo::from_user_model)
        .collect();

    let events = group
        .find_related(Event)
        .all(&ext.database)
        .await
        .or_internal_server_error("Error loading events from database")?;

    let group_data = GroupDetailData {
        name: group.name,
        members,
        events,
    };
    Ok(group_data)
}

#[put("/api/groups/{group_id}", ext: Extension<server::AppState>)]
pub async fn change_group_name(
    group_id: i32,
    group_name_new: String,
) -> Result<entity::group::Model, ServerFnError> {
    use entity::group;
    use entity::group::Entity as Group;
    use sea_orm::{ActiveModelTrait, EntityTrait, Set};

    let group = Group::find_by_id(group_id)
        .one(&ext.database)
        .await
        .or_internal_server_error("Error loading Group")?
        .or_not_found("Group not found")?;

    let mut group: group::ActiveModel = group.into();

    group.name = Set(group_name_new);

    let group = group
        .update(&ext.database)
        .await
        .or_internal_server_error("Error updating database")?;

    Ok(group)
}

#[delete("/api/groups/{group_id}", ext: Extension<server::AppState>, auth: Extension<server::AuthenticationState>)]
pub async fn delete_group(group_id: i32) -> Result<NoContent, ServerFnError> {
    use crate::routes::groups::server_functions::is_user_in_group;
    use entity::group::Entity as Group;
    use sea_orm::EntityTrait;

    let user = auth.user.as_ref().or_unauthorized("Not authenticated")?;
    let authenticated = is_user_in_group(&ext.database, group_id, user.id).await?;
    if authenticated {
        let delete_result = Group::delete_by_id(group_id)
            .exec(&ext.database)
            .await
            .or_internal_server_error("Error deleting group")?;

        (delete_result.rows_affected == 1).or_not_found("User not found")?;
        Ok(NoContent)
    } else {
        Err(ServerFnError::ServerError {
            message: "No permission to delte group.".to_string(),
            code: 401,
            details: None,
        })
    }
}

#[cfg(feature = "server")]
mod server_functions {
    use super::*;
    use sea_orm::DatabaseConnection;

    pub async fn is_user_in_group(
        db: &DatabaseConnection,
        group_id: i32,
        user_id: i32,
    ) -> Result<bool, ServerFnError> {
        use entity::is_in_group::Entity as IsInGroup;
        use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

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
        use entity::shared_group_event::Entity as SharedGroupEvent;
        use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

        Ok(SharedGroupEvent::find()
            .filter(entity::shared_group_event::Column::GroupId.eq(group_id))
            .filter(entity::shared_group_event::Column::EventId.eq(event_id))
            .one(db)
            .await
            .or_internal_server_error("Error loading from database")?
            .is_some())
    }
}
