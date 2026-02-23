use crate::server;
use dioxus::fullstack::NoContent;
use dioxus::prelude::*;
#[cfg(feature = "server")]
use dioxus::server::axum::Extension;
use entity::prelude::*;
use entity::todo_list::{CreateTodoList, UpdateTodoList};

pub mod invite;

#[get("/api/todolists", state: Extension<server::AppState>, auth: Extension<server::AuthenticationState>)]
pub async fn list_todo_lists()
-> Result<Vec<entity::todo_list::TodoListWithPermission>, ServerFnError> {
    use entity::todo_list::Column as TodoListColumn;
    use entity::todo_list_invitation::Column as InvitationColumn;
    use sea_orm::ColumnTrait;
    use sea_orm::Condition;
    use sea_orm::EntityTrait;
    use sea_orm::QueryFilter;
    use sea_orm::QueryOrder;

    let user = auth.user.as_ref().or_unauthorized("Not authenticated")?;

    let todo_lists = TodoList::find()
        .inner_join(TodoListInvitation)
        .filter(
            Condition::all()
                .add(InvitationColumn::ReceivingUserId.eq(user.id))
                .add(InvitationColumn::IsAccepted.eq(true)),
        )
        .order_by_asc(TodoListColumn::Title)
        .into_partial_model()
        .all(&state.database)
        .await
        .inspect_err(|e| error!("{e}"))
        .or_internal_server_error("Error loading todo lists")?;

    Ok(todo_lists)
}

#[get("/api/todolists/{todo_list_id}", state: Extension<server::AppState>, auth: Extension<server::AuthenticationState>)]
pub async fn retrieve_todo_list(
    todo_list_id: i32,
) -> Result<entity::todo_list::TodoListWithPermission, ServerFnError> {
    use entity::todo_list_invitation::Column as InvitationColumn;
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

    let user = auth.user.as_ref().or_unauthorized("Not authenticated")?;

    let todo_list = TodoList::find_by_id(todo_list_id)
        .inner_join(TodoListInvitation)
        .filter(InvitationColumn::ReceivingUserId.eq(user.id))
        .into_partial_model()
        .one(&state.database)
        .await
        .inspect_err(|e| error!("{e}"))
        .or_internal_server_error("Error loading To-Do List")?
        .or_not_found("To-Do List not found")?;

    Ok(todo_list)
}

#[get("/api/todolists/{todo_list_id}/members", state: Extension<server::AppState>, auth: Extension<server::AuthenticationState>)]
pub async fn list_todo_list_members(
    todo_list_id: i32,
) -> Result<Vec<entity::user::UserWithTodoListInvitation>, ServerFnError> {
    use sea_orm::EntityTrait;
    use sea_orm::JoinType;
    use sea_orm::{ColumnTrait, QueryFilter, QuerySelect, RelationTrait};

    let user = auth.user.as_ref().or_unauthorized("Not authenticated")?;

    let _ = server::todo_lists::get_todo_list_permission(todo_list_id, user.id, &state.database)
        .await?
        .or_forbidden("You are not permitted to view members of this To-Do List")?;

    let members = User::find()
        .filter(entity::todo_list_invitation::Column::TodoListId.eq(todo_list_id))
        .join(
            JoinType::InnerJoin,
            entity::todo_list_invitation::Relation::Receiver.def().rev(),
        )
        .into_partial_model()
        .all(&state.database)
        .await
        .inspect_err(|e| error!("Member Query Failed: {e}"))
        .or_internal_server_error("Failed to load members")?;

    Ok(members)
}

#[post("/api/todolists", state: Extension<server::AppState>, auth: Extension<server::AuthenticationState>)]
pub async fn create_todo_list(
    data: CreateTodoList,
) -> Result<entity::todo_list::Model, ServerFnError> {
    use sea_orm::TransactionError;
    use sea_orm::TransactionTrait;
    use sea_orm::{ActiveModelTrait, IntoActiveModel, TryIntoModel};
    let user_id = auth.user.as_ref().or_unauthorized("Not authenticated")?.id;

    state
        .database
        .transaction::<_, entity::todo_list::Model, ServerFnError>(move |txn| {
            Box::pin(async move {
                let mut todo_list = data.into_active_model();
                todo_list.created_by_id = sea_orm::Set(Some(user_id));

                let todo_list = todo_list
                    .save(txn)
                    .await
                    .inspect_err(|error| error!("{error}"))
                    .or_internal_server_error("Error creating todo list")?;

                let todo_list = todo_list
                    .try_into_model()
                    .or_internal_server_error("Error parsing todo list")?;

                let invitation = entity::todo_list_invitation::ActiveModel {
                    todo_list_id: sea_orm::Set(todo_list.id),
                    receiving_user_id: sea_orm::Set(user_id),
                    permission: sea_orm::Set(
                        entity::todo_list_invitation::InvitationPermission::Admin,
                    ),
                    is_accepted: sea_orm::Set(true),
                    is_favorite: sea_orm::Set(false),
                    ..Default::default()
                };
                invitation
                    .insert(txn)
                    .await
                    .inspect_err(|error| error!("{error}"))
                    .or_internal_server_error("Error creating todo list invitation")?;

                Ok(todo_list)
            })
        })
        .await
        .map_err(|error| {
            error!("Creating To-Do List: {error}");
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

#[patch("/api/todolists/{todo_list_id}", state: Extension<server::AppState>, auth: Extension<server::AuthenticationState>)]
pub async fn update_todo_list(
    todo_list_id: i32,
    data: UpdateTodoList,
) -> Result<entity::todo_list::Model, ServerFnError> {
    use sea_orm::{ActiveModelTrait, IntoActiveModel, TryIntoModel};
    let user = auth.user.as_ref().or_unauthorized("Not authenticated")?;

    server::todo_lists::get_todo_list_permission(todo_list_id, user.id, &state.database)
        .await?
        .or_forbidden("Unauthorized to update todo list")?
        .can_write()
        .or_forbidden("Unauthorized to update todo list")?;

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

    server::todo_lists::get_todo_list_permission(todo_list_id, user.id, &state.database)
        .await?
        .or_forbidden("Unauthorized to update todo list")?
        .can_admin()
        .or_forbidden("Unauthorized to update todo list")?;

    TodoList::delete_by_id(todo_list_id)
        .exec(&state.database)
        .await
        .inspect_err(|e| error!("Error deleting todo list: {e}"))
        .or_internal_server_error("Failed to delete todo list")?;

    Ok(NoContent)
}

/// Kicks a user from a `TodoList`. Can only be performed by the owner or an admin of the `TodoList`.
/// If the owner was kicked, the ownership will be transferred to the user which kicked the owner.
#[post("/api/todolists/{todo_list_id}/remove-user", state: Extension<server::AppState>, auth: Extension<server::AuthenticationState>)]
pub async fn remove_user_from_todo_list(
    todo_list_id: i32,
    user_id: i32,
) -> Result<NoContent, ServerFnError> {
    let request_user_id = auth.user.as_ref().or_unauthorized("Not authenticated")?.id;
    server::todo_lists::remove_user_from_todo_list(
        todo_list_id,
        user_id,
        request_user_id,
        &state.database,
    )
    .await?;
    Ok(NoContent)
}
