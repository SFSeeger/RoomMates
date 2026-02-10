use crate::server;
use dioxus::fullstack::NoContent;
use dioxus::prelude::*;
#[cfg(feature = "server")]
use dioxus::server::axum::Extension;
use entity::prelude::*;
use entity::todo_list::{CreateTodoList, UpdateTodoList};
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
mod server_functions {
    use super::*;

    pub(crate) async fn get_todo_list_permission(
        todo_list_id: i32,
        user_id: i32,
        database: &sea_orm::DatabaseConnection,
    ) -> Result<Option<entity::todo_list_invitation::InvitationPermission>, ServerFnError> {
        use sea_orm::ColumnTrait;
        use sea_orm::EntityTrait;
        use sea_orm::QueryFilter;

        let todo_list = TodoList::find_by_id(todo_list_id)
            .one(database)
            .await
            .or_internal_server_error("Failed to load todo list")?
            .or_not_found("Todo list not found")?;

        if todo_list.owner_id == user_id {
            return Ok(Some(
                entity::todo_list_invitation::InvitationPermission::Admin,
            ));
        }

        let invitation = TodoListInvitation::find()
            .filter(entity::todo_list_invitation::Column::ReceivingUserId.eq(user_id))
            .filter(entity::todo_list_invitation::Column::TodoListId.eq(todo_list_id))
            .filter(entity::todo_list_invitation::Column::IsAccepted.eq(true))
            .one(database)
            .await
            .or_internal_server_error("Failed to load todo list invitation")?;

        Ok(invitation.map(|inv| inv.permission))
    }
}

#[get("/api/todolists", state: Extension<server::AppState>, auth: Extension<server::AuthenticationState>)]
pub async fn list_todo_lists() -> Result<Vec<entity::todo_list::Model>, ServerFnError> {
    use sea_orm::ColumnTrait;
    use sea_orm::Condition;
    use sea_orm::EntityTrait;
    use sea_orm::QueryFilter;
    use sea_orm::QueryOrder;

    let user = auth.user.as_ref().or_unauthorized("Not authenticated")?;

    let todo_lists = TodoList::find()
        .left_join(TodoListInvitation)
        .filter(
            Condition::any()
                .add(entity::todo_list::Column::OwnerId.eq(user.id))
                .add(
                    Condition::all()
                        .add(entity::todo_list_invitation::Column::ReceivingUserId.eq(user.id))
                        .add(entity::todo_list_invitation::Column::IsAccepted.eq(true)),
                ),
        )
        .order_by_asc(entity::todo_list::Column::Title)
        .all(&state.database)
        .await
        .or_internal_server_error("Error loading todo lists")?;

    Ok(todo_lists)
}

#[post("/api/todolists", state: Extension<server::AppState>, auth: Extension<server::AuthenticationState>)]
pub async fn create_todo_list(
    data: CreateTodoList,
) -> Result<entity::todo_list::Model, ServerFnError> {
    use sea_orm::{ActiveModelTrait, IntoActiveModel, TryIntoModel};
    let user = auth.user.as_ref().or_unauthorized("Not authenticated")?;

    let mut todo_list = data.into_active_model();
    todo_list.owner_id = sea_orm::Set(user.id);
    todo_list.is_favorite = sea_orm::Set(false);

    let todo_list = todo_list
        .save(&state.database)
        .await
        .inspect_err(|error| error!("{error:?}"))
        .or_internal_server_error("Error creating todo list")?;
    Ok(todo_list
        .try_into_model()
        .or_internal_server_error("Error parsing todo list")?)
}

#[patch("/api/todolists/{todo_list_id}", state: Extension<server::AppState>, auth: Extension<server::AuthenticationState>)]
pub async fn update_todo_list(
    todo_list_id: i32,
    data: UpdateTodoList,
) -> Result<entity::todo_list::Model, ServerFnError> {
    use sea_orm::{ActiveModelTrait, IntoActiveModel, TryIntoModel};
    let user = auth.user.as_ref().or_unauthorized("Not authenticated")?;

    let permission =
        server_functions::get_todo_list_permission(todo_list_id, user.id, &state.database)
            .await
            .inspect_err(|e| error!("{e}"))?
            .or_unauthorized("Unauthorized to update todo list")?;

    permission
        .can_write()
        .or_unauthorized("Unauthorized to update todo list")?;

    let mut todo_list = data.into_active_model();
    todo_list.id = sea_orm::Unchanged(todo_list_id);
    let todo_list = todo_list
        .save(&state.database)
        .await
        .or_internal_server_error("Failed to update todo list")?;
    Ok(todo_list
        .try_into_model()
        .or_internal_server_error("Error parsing todo list")?)
}

#[delete("/api/todolists/{todo_list_id}", state: Extension<server::AppState>, auth: Extension<server::AuthenticationState>)]
pub async fn delete_todo_list(todo_list_id: i32) -> Result<NoContent, ServerFnError> {
    use sea_orm::EntityTrait;
    let user = auth.user.as_ref().or_unauthorized("Not authenticated")?;

    let permission =
        server_functions::get_todo_list_permission(todo_list_id, user.id, &state.database)
            .await
            .inspect_err(|e| error!("{e}"))?
            .or_unauthorized("Unauthorized to update todo list")?;

    permission
        .can_write()
        .or_unauthorized("Unauthorized to update todo list")?;

    TodoList::delete_by_id(todo_list_id)
        .exec(&state.database)
        .await
        .or_internal_server_error("Failed to delete todo list")?;

    Ok(NoContent)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InviteToTodoListData {
    email: String,
    #[serde(default)]
    permission: entity::todo_list_invitation::InvitationPermission,
}

#[post("/api/todolists/{todo_list_id}/invite", state: Extension<server::AppState>, auth: Extension<server::AuthenticationState>)]
pub async fn invite_to_todo_list(
    todo_list_id: i32,
    data: InviteToTodoListData,
) -> Result<NoContent, ServerFnError> {
    use crate::routes::users::EMAIL_REGEX;
    use regex::Regex;
    use sea_orm::ColumnTrait;
    use sea_orm::EntityTrait;
    use sea_orm::QueryFilter;
    use sea_orm::{ActiveModelTrait, SelectExt};

    let user = auth.user.as_ref().or_unauthorized("Not authenticated")?;

    let email_regex = Regex::new(EMAIL_REGEX).expect("EMAIL_REGEX must be valid");
    email_regex
        .is_match(&data.email)
        .or_bad_request("email is not a valid email")?;

    let to_user = User::find()
        .filter(entity::user::Column::Email.eq(data.email))
        .one(&state.database)
        .await
        .or_internal_server_error("Failed to load user")?
        .or_not_found("User not found")?;

    let permission =
        server_functions::get_todo_list_permission(todo_list_id, user.id, &state.database)
            .await
            .inspect_err(|e| error!("{e}"))?
            .or_unauthorized("Unauthorized to update todo list")?;

    permission
        .can_admin()
        .or_unauthorized("Unauthorized to update todo list")?;

    let existing_invitation = TodoListInvitation::find()
        .filter(entity::todo_list_invitation::Column::ReceivingUserId.eq(to_user.id))
        .filter(entity::todo_list_invitation::Column::TodoListId.eq(todo_list_id))
        .exists(&state.database)
        .await
        .or_internal_server_error("Failed to load todo list invitation")?;

    (!existing_invitation).or_bad_request("User is already invited to this todo list")?;

    let invitation = entity::todo_list_invitation::ActiveModel {
        receiving_user_id: sea_orm::Set(to_user.id),
        sender_user_id: sea_orm::Set(user.id),
        todo_list_id: sea_orm::Set(todo_list_id),
        permission: sea_orm::Set(data.permission),
        is_accepted: sea_orm::Set(false),
        ..Default::default()
    };

    let _ = invitation
        .save(&state.database)
        .await
        .or_internal_server_error("Failed to invite user")?;

    Ok(NoContent)
}

#[post("/api/todolists/{todo_list_id}/invite/accept", state: Extension<server::AppState>, auth: Extension<server::AuthenticationState>)]
pub async fn accept_todo_list_invite(todo_list_id: i32) -> Result<NoContent, ServerFnError> {
    use entity::todo_list_invitation::Column as InviteColum;
    use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter};
    let user = auth.user.as_ref().or_unauthorized("Not authenticated")?;

    let invitation = TodoListInvitation::find()
        .filter(InviteColum::ReceivingUserId.eq(user.id))
        .filter(InviteColum::TodoListId.eq(todo_list_id))
        .one(&state.database)
        .await
        .inspect_err(|e| error!("{e}"))
        .or_internal_server_error("Failed to retrive user")?
        .or_not_found("Cannot accept invite")?;

    let mut invitation = invitation.into_active_model();
    invitation.is_accepted = sea_orm::Set(true);
    invitation
        .save(&state.database)
        .await
        .inspect_err(|e| error!("{e}"))
        .or_internal_server_error("Failed to accept Invite")?;

    Ok(NoContent)
}
