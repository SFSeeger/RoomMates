use crate::server;
use dioxus::fullstack::NoContent;
use dioxus::prelude::*;
#[cfg(feature = "server")]
use dioxus::server::axum::Extension;
use entity::prelude::TodoList;
use entity::prelude::*;
use entity::todo::{CreateToDo, UpdateToDo};

#[get("/api/todolists/{todo_list_id}/todos", state: Extension<server::AppState>, auth: Extension<server::AuthenticationState>)]
pub async fn list_todo(todo_list_id: i32) -> Result<Vec<entity::todo::Model>, ServerFnError> {
    use sea_orm::ColumnTrait;
    use sea_orm::EntityTrait;
    use sea_orm::QueryFilter;
    use sea_orm::QueryOrder;

    let user = auth.user.as_ref().or_unauthorized("Not authenticated")?;

    let todo_list = TodoList::find_by_id(todo_list_id)
        .one(&state.database)
        .await
        .or_internal_server_error("Error loading To-Do List")?
        .or_not_found("To-Do List not found")?;

    let _ = server::todo_lists::get_todo_list_permission(todo_list.id, user.id, &state.database)
        .await?
        .or_forbidden("You are not permitted to view Tasks in this To-Do List")?;

    let todos = Todo::find()
        .filter(entity::todo::Column::TodoListId.eq(todo_list_id))
        .order_by_asc(entity::todo::Column::Completed)
        .order_by_asc(entity::todo::Column::Title)
        .all(&state.database)
        .await
        .or_internal_server_error("Error loading Tasks")?;

    Ok(todos)
}

#[get("/api/todos/?completed&favorite", state: Extension<server::AppState>, auth: Extension<server::AuthenticationState>)]
pub async fn list_todos(
    completed: Option<bool>,
    favorite: Option<bool>,
) -> Result<Vec<entity::todo::TodoWithPermission>, ServerFnError> {
    use sea_orm::ColumnTrait;
    use sea_orm::EntityTrait;
    use sea_orm::JoinType;
    use sea_orm::QueryFilter;
    use sea_orm::QueryOrder;
    use sea_orm::QuerySelect;
    use sea_orm::QueryTrait;
    use sea_orm::RelationTrait;

    let user = auth.user.as_ref().or_unauthorized("Not authenticated")?;

    let todos = Todo::find()
        .join(JoinType::InnerJoin, entity::todo::Relation::TodoList.def())
        .join(
            JoinType::InnerJoin,
            entity::todo_list::Relation::TodoListInvitation.def(),
        )
        .filter(entity::todo_list_invitation::Column::ReceivingUserId.eq(user.id))
        .filter(entity::todo_list_invitation::Column::IsAccepted.eq(true))
        .apply_if(completed, |query, v| {
            query.filter(entity::todo::Column::Completed.eq(v))
        })
        .apply_if(favorite, |query, v| {
            query.filter(entity::todo_list_invitation::Column::IsFavorite.eq(v))
        })
        .order_by_asc(entity::todo::Column::Completed)
        .order_by_asc(entity::todo::Column::Title)
        .into_partial_model()
        .all(&state.database)
        .await
        .inspect_err(|e| error!("{e}"))
        .or_internal_server_error("Error loading Tasks")?;

    Ok(todos)
}

#[post("/api/todolists/{todo_list_id}/todos", state: Extension<server::AppState>, auth: Extension<server::AuthenticationState>)]
pub async fn create_todo(
    todo_list_id: i32,
    data: CreateToDo,
) -> Result<entity::todo::Model, ServerFnError> {
    use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, Set};

    let user = auth.user.as_ref().or_unauthorized("Not authenticated")?;

    let todo_list = TodoList::find_by_id(todo_list_id)
        .one(&state.database)
        .await
        .or_internal_server_error("Failed to load To-Do List")?
        .or_not_found("To-Do List not founds")?;

    server::todo_lists::get_todo_list_permission(todo_list.id, user.id, &state.database)
        .await?
        .or_forbidden("You are not permitted to add Tasks in this To-Do List")?
        .can_write()
        .or_forbidden("You are not permitted to add Tasks to this To-Do List")?;

    let mut todo = data.into_active_model();
    todo.completed = Set(false);
    todo.owner_id = Set(user.id);
    todo.todo_list_id = Set(todo_list_id);

    let todo = todo
        .insert(&state.database)
        .await
        .inspect_err(|error| error!("{error:?}"))
        .or_internal_server_error("Failed to create Task")?;

    Ok(todo)
}

#[patch("/api/todos/{todo_id}", state: Extension<server::AppState>, auth: Extension<server::AuthenticationState>)]
pub async fn update_todo(
    todo_id: i32,
    data: UpdateToDo,
) -> Result<entity::todo::Model, ServerFnError> {
    use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, TryIntoModel};
    let user = auth.user.as_ref().or_unauthorized("Not authenticated")?;
    let todo = Todo::find_by_id(todo_id)
        .one(&state.database)
        .await
        .or_internal_server_error("Failed to load To-do Task")?
        .or_not_found("To-do Task not found")?;

    server::todo_lists::get_todo_list_permission(todo.todo_list_id, user.id, &state.database)
        .await?
        .or_forbidden("You are not permitted to update Tasks in this To-Do List")?
        .can_write()
        .or_forbidden("You are not permitted to update Tasks in this To-Do List")?;

    let mut todo = todo.into_active_model();
    if let Some(title) = data.title {
        todo.title = sea_orm::Set(title);
    }
    if let Some(details) = data.details {
        todo.details = sea_orm::Set(details);
    }
    if let Some(completed) = data.completed {
        todo.completed = sea_orm::Set(completed);
    }

    let todo = todo
        .save(&state.database)
        .await
        .or_internal_server_error("Failed to update Task")?;

    Ok(todo
        .try_into_model()
        .or_internal_server_error("Failed to convert to Active Model")?)
}

#[delete("/api/todos/{todo_id}", state: Extension<server::AppState>, auth: Extension<server::AuthenticationState>)]
pub async fn delete_todo(todo_id: i32) -> Result<NoContent, ServerFnError> {
    use sea_orm::{EntityTrait, ModelTrait};

    let user = auth.user.as_ref().or_unauthorized("Not authenticated")?;

    let todo = Todo::find_by_id(todo_id)
        .one(&state.database)
        .await
        .or_internal_server_error("Failed to load To-do Task")?
        .or_not_found("To-do Task not found")?;

    server::todo_lists::get_todo_list_permission(todo.todo_list_id, user.id, &state.database)
        .await?
        .or_forbidden("You are not permitted to delete Tasks in this To-Do List")?
        .can_write()
        .or_forbidden("You are not permitted to delete Tasks in this To-Do List")?;

    todo.delete(&state.database)
        .await
        .or_internal_server_error("Failed to delete Task")?;

    Ok(NoContent)
}
