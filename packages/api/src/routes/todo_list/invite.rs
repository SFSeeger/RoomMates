use crate::server;
use dioxus::fullstack::NoContent;
use dioxus::prelude::*;
#[cfg(feature = "server")]
use dioxus::server::axum::Extension;
use entity::prelude::*;
use entity::todo_list_invitation::{UpdateMyTodoListInvitation, UpdateTodoListInvitation};
use entity::user::UserWithTodoListInvitation;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InviteToTodoListData {
    email: String,
    #[serde(default)]
    permission: entity::todo_list_invitation::InvitationPermission,
}

#[post("/api/todolists/{todo_list_id}/invite", state: Extension<server::AppState>, auth: Extension<server::AuthenticationState> )]
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

    server::todo_lists::get_todo_list_permission(todo_list_id, user.id, &state.database)
        .await?
        .or_forbidden("Unauthorized to invite")?
        .can_admin()
        .or_forbidden("Unauthorized to invite")?;

    let _ = TodoList::find_by_id(todo_list_id)
        .one(&state.database)
        .await
        .or_internal_server_error("Failed to retrieve To-Do List")?
        .or_not_found(format!("Unable to find To-Do List with id {todo_list_id}"))?;

    let existing_invitation = TodoListInvitation::find()
        .filter(entity::todo_list_invitation::Column::ReceivingUserId.eq(to_user.id))
        .filter(entity::todo_list_invitation::Column::TodoListId.eq(todo_list_id))
        .exists(&state.database)
        .await
        .or_internal_server_error("Failed to load todo list invitation")?;

    (!existing_invitation).or_bad_request("User is already invited to this todo list")?;

    let invitation = entity::todo_list_invitation::ActiveModel {
        receiving_user_id: sea_orm::Set(to_user.id),
        sender_user_id: sea_orm::Set(Some(user.id)),
        todo_list_id: sea_orm::Set(todo_list_id),
        permission: sea_orm::Set(data.permission),
        is_accepted: sea_orm::Set(false),
        is_favorite: sea_orm::Set(false),
    };

    let _ = invitation
        .insert(&state.database)
        .await
        .inspect_err(|e| error!("{e}"))
        .or_internal_server_error("Failed to invite user")?;

    Ok(NoContent)
}

#[post("/api/todolists/{todo_list_id}/invite/accept", state: Extension<server::AppState>, auth: Extension<server::AuthenticationState> )]
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
        .or_internal_server_error("Failed to retrieve Invite")?
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

#[post("/api/todolists/{todo_list_id}/invite/leave", state: Extension<server::AppState>, auth: Extension<server::AuthenticationState> )]
pub async fn leave_todo_list(todo_list_id: i32) -> Result<NoContent, ServerFnError> {
    use entity::todo_list_invitation::Column as InviteColum;
    use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, PaginatorTrait, QueryFilter};

    let user = auth.user.as_ref().or_unauthorized("Not authenticated")?;

    let todo_list = TodoList::find_by_id(todo_list_id)
        .one(&state.database)
        .await
        .inspect_err(|e| error!("{e}"))
        .or_internal_server_error("Failed to load todo list")?
        .or_not_found("Todo list not found")?;

    let invitation = TodoListInvitation::find()
        .filter(InviteColum::ReceivingUserId.eq(user.id))
        .filter(InviteColum::TodoListId.eq(todo_list_id))
        .filter(InviteColum::IsAccepted.eq(true))
        .one(&state.database)
        .await
        .inspect_err(|e| error!("{e}"))
        .or_internal_server_error("Failed to retrieve Invite")?
        .or_not_found("Cannot leave todo list")?;

    let invitation_count = TodoListInvitation::find()
        .filter(InviteColum::TodoListId.eq(todo_list_id))
        .count(&state.database)
        .await
        .inspect_err(|e| error!("{e}"))
        .or_internal_server_error("Failed to load todo list invitations")?;

    if invitation_count <= 1 {
        // If there is only 1 invitation, then the user is the only member of the To-Do list
        // In this case, we can allow them to leave and delete the To-Do list as well to avoid orphaned To-Do lists
        todo_list
            .delete(&state.database)
            .await
            .inspect_err(|e| error!("{e}"))
            .or_internal_server_error("Failed to delete todo list")?;

        return Ok(NoContent);
    }

    // Ensure that the user is not the only admin of the To-Do list before allowing them to leave
    // This is to avoid deadlocking the To-Do list with no admins
    if invitation.permission.can_admin() {
        let admin_users = TodoListInvitation::find()
            .filter(InviteColum::TodoListId.eq(todo_list_id))
            .filter(
                InviteColum::Permission
                    .eq(entity::todo_list_invitation::InvitationPermission::Admin),
            )
            .filter(InviteColum::IsAccepted.eq(true))
            .count(&state.database)
            .await
            .inspect_err(|e| error!("{e}"))
            .or_internal_server_error("Failed to load todo list invitations")?;

        (admin_users > 1)
            .or_bad_request("Cannot leave todo list as you are the only active admin")?;
    }

    invitation
        .delete(&state.database)
        .await
        .or_internal_server_error("Failed to leave todo list")?;

    Ok(NoContent)
}

#[patch("/api/todolists/{todo_list_id}/invite/{user_id}", state: Extension<server::AppState>, auth: Extension<server::AuthenticationState>)]
pub async fn update_todo_list_invitation(
    todo_list_id: i32,
    user_id: i32,
    data: UpdateTodoListInvitation,
) -> Result<UserWithTodoListInvitation, ServerFnError> {
    use entity::todo_list_invitation::Column as InviteColum;
    use sea_orm::{
        ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, PaginatorTrait, QueryFilter,
        QuerySelect, RelationTrait,
    };

    let user = auth.user.as_ref().or_unauthorized("Not authenticated")?;

    server::todo_lists::get_todo_list_permission(todo_list_id, user.id, &state.database)
        .await?
        .or_forbidden("Unauthorized to update invite")?
        .can_admin()
        .or_forbidden("Unauthorized to update invite")?;

    let admin_users = TodoListInvitation::find()
        .filter(InviteColum::TodoListId.eq(todo_list_id))
        .filter(
            InviteColum::Permission.eq(entity::todo_list_invitation::InvitationPermission::Admin),
        )
        .filter(InviteColum::IsAccepted.eq(true))
        .count(&state.database)
        .await
        .inspect_err(|e| error!("{e}"))
        .or_internal_server_error("Failed to load todo list invitations")?;

    (admin_users > 1)
        .or_bad_request("Cannot update your permission as you are the only active admin")?;

    let invitation = TodoListInvitation::find()
        .filter(InviteColum::ReceivingUserId.eq(user_id))
        .filter(InviteColum::TodoListId.eq(todo_list_id))
        .one(&state.database)
        .await
        .inspect_err(|e| error!("{e}"))
        .or_internal_server_error("Failed to retrieve Invite")?
        .or_not_found("Invite not found")?;

    let permission = data.permission.unwrap_or(invitation.permission);
    let mut invitation = invitation.into_active_model();
    invitation.permission.set_if_not_equals(permission);
    invitation
        .save(&state.database)
        .await
        .inspect_err(|e| error!("{e}"))
        .or_internal_server_error("Failed to update Invite")?;

    let user = User::find_by_id(user_id)
        .join(
            sea_orm::JoinType::InnerJoin,
            entity::todo_list_invitation::Relation::Receiver.def().rev(),
        )
        .into_partial_model()
        .one(&state.database)
        .await
        .inspect_err(|e| error!("{e}"))
        .or_internal_server_error("Failed to load user")?
        .or_not_found("User not found")?;

    Ok(user)
}

#[patch("/api/todolists/{todo_list_id}/invite", state: Extension<server::AppState>, auth: Extension<server::AuthenticationState>)]
pub async fn update_my_todo_list_invitation(
    todo_list_id: i32,
    data: UpdateMyTodoListInvitation,
) -> Result<UserWithTodoListInvitation, ServerFnError> {
    use entity::todo_list_invitation::Column as InviteColum;
    use sea_orm::{
        ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, QuerySelect,
        RelationTrait,
    };

    let user = auth.user.as_ref().or_unauthorized("Not authenticated")?;

    server::todo_lists::get_todo_list_permission(todo_list_id, user.id, &state.database)
        .await?
        .or_forbidden("Unauthorized to update invite")?;

    let invitation = TodoListInvitation::find()
        .filter(InviteColum::ReceivingUserId.eq(user.id))
        .filter(InviteColum::TodoListId.eq(todo_list_id))
        .one(&state.database)
        .await
        .inspect_err(|e| error!("{e}"))
        .or_internal_server_error("Failed to retrieve Invite")?
        .or_not_found("Invite not found")?;

    let is_favorite = data.is_favorite.unwrap_or(invitation.is_favorite);
    let mut invitation = invitation.into_active_model();
    invitation.is_favorite.set_if_not_equals(is_favorite);
    invitation
        .save(&state.database)
        .await
        .inspect_err(|e| error!("{e}"))
        .or_internal_server_error("Failed to update Invite")?;

    let user = User::find_by_id(user.id)
        .join(
            sea_orm::JoinType::InnerJoin,
            entity::todo_list_invitation::Relation::Receiver.def().rev(),
        )
        .into_partial_model()
        .one(&state.database)
        .await
        .inspect_err(|e| error!("{e}"))
        .or_internal_server_error("Failed to load user")?
        .or_not_found("User not found")?;

    Ok(user)
}
