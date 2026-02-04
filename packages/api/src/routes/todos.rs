use crate::server;
use dioxus::fullstack::NoContent;
use dioxus::prelude::*;
#[cfg(feature = "server")]
use dioxus::server::axum::Extension;
use entity::prelude::TodoList;
use entity::todos::{CreateToDo, UpdateToDo};

#[get("/api/todolists/{todo_list_id}/tdos", state: Extension<server::AppState>, auth: Extension<server::AuthenticationState>)]
pub async fn list_todo(
    todo_list_id: i32,
) -> Result<Vec<entity::todo::Model>, ServerFnError> {
    use sea_orm::EntityTrait;
    use sea_orm::QueryFilter;
    use sea_orm::QueryOrder;

    let user = auth.user.as_ref().or_unauthorized("Not authenticated")?;

    let todo_list = TodoList::find_by_id(todo_list_id)
        .one(&state.database)
        .await
        .or_internal_server_error("Error loading To-Do List")?
        .or_not_found("To-Do List not found")?;

    (todo_list.owner_id == user.id)
    .or_unauthorized("Not authenticated")?;

    let todos = Todo::find()
    .filter(entity::todo::Column::TodoListId.eq(todo_list_id))
    .order_by_asc(entity::todo::Column::Completed)
    .order_by_asc(entity::todo::Column::Id)
    .all(&state.database)
    .await
    .or_internal_server_error("Error loading Tasks")?;

    Ok(todos)
}

#[post("/api/todolists/{todo_list_id}/todos", state: Extension<server::AppState>, auth: Extension<server::AuthenticationState>)]
pub async fn create_todo(
    todo_list_id: i32,
    data: CreateTodo,
) -> Result<entity::todo::Model, ServerFnError> {
    use sea_orm::{ActiveModelTrait, EntityTrait, Set};

    let user = auth.user.as_ref().or_not_found("Not authenticated")?;

    let todo_list = TodoList::find_by_id(todo_list_id)
    .one(&state.database)
    .await
    .or_internal_server_error("Failed to load To-Do List")?
    .or_not_found("To-Do List not founds")?;

(todo_list.owner_id == user.id)
    .or_unauthorized("Not authenticated")?;

    let todo = entity::todo::ActiveModel{
        title: Set(data.title),
        details: Set(data.details),
        completed: Set(data.completed),
        todo_list_id: Set(todo_list_id),
        ..Default::default()
    };

    let todo = todo
    .insert(&state.database)
    .await
    .or_internal_server_error("Failed to create Task")?;
}

#[patch("/api/todos/{todo_id}", state: Extension<server::AppState>, auth: Extension<server::AuthenticationState>)]
pub async fn update_todo(
    todo_id: i32,
    data: UpdateTodo,
) -> Result<entity::todo::Model, ServerFnError> {
    use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, TryIntoModel};
    let user = auth.user.as_ref().or_unauthorized("Not authenticated")?;
    let todo = Todo::find_by_id(todo_id)
        .one(&state.database)
        .await
        .or_internal_server_error("Failed to load To-do Task")?
        .or_not_found("To-do Task not found")?;

    (todo_list.owner_id == user.id)
    .or_unauthorized("Unauthorized to update Task")?;

    let mut todo = todo.into_active_model();
    if let Some(title) = data.title{
        todo.title = sea_orm::Set(title);
    }
     if let Some(details) = data.details{
        todo.details = sea_orm::Set(details);
    }
     if let Some(completed) = data.completed{
        todo.completed = sea_orm::Set(completed);
    }

    let todo = todo
        .save(&state.database)
        .await
        .or_internal_server_error("Failed to update Task")?;

   Ok(todo)
}

#[delete("/api/todos/{todo_id}", state: Extension<server:AppState>, auth: Extension<server::AuthenticationState>
)]
pub async fn delete_todo(
    todo_id: i32,
) -> Result<NoContent, ServerFnError>{
    use sea_orm::{EntityTrait, ModelTrait};

    let user = auth.user.as_ref().or_unauthorized("Not authenticated")?;

    let todo = Todo::find_by_id(todo_id)
    .one(&state.database)
    .await
    .or_internal_server_error("Failed to load To-do Task")?
    .or_not_found("To-do Task not found")?;

    let todo_list = TodoList::find_by_id(todo.todo_list_id)
    .one(&state.database)
    .await
    .or_internal_server_error("Failed to load To-do List")?
    .or_not_found("To-do List not found")?;

    (todo_list.owner_id == user.id)
    .or_unauthorized("Not autheticated")?;

    todo
    .delete(&state.database)
    .await
    .or_internal_server_error("Failed to delete Task")?;

    Ok(NoContent)
}
