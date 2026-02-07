use crate::server;
use dioxus::fullstack::NoContent;
use dioxus::prelude::*;
#[cfg(feature = "server")]
use dioxus::server::axum::Extension;
use entity::prelude::TodoList;
use entity::todo_list::{CreateTodoList, UpdateTodoList};

#[get("/api/todolists", state: Extension<server::AppState>, auth: Extension<server::AuthenticationState>)]
pub async fn list_todo_lists() -> Result<Vec<entity::todo_list::Model>, ServerFnError> {
    use sea_orm::ModelTrait;
    use sea_orm::QueryOrder;

    let user = auth.user.as_ref().or_unauthorized("Not authenticated")?;

    let todo_lists = user
        .find_related(TodoList)
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
    use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, TryIntoModel};
    let user = auth.user.as_ref().or_unauthorized("Not authenticated")?;
    let todo_list = TodoList::find_by_id(todo_list_id)
        .one(&state.database)
        .await
        .or_internal_server_error("Failed to load todo list")?
        .or_not_found("Todo list not found")?;
    (todo_list.owner_id == user.id).or_unauthorized("Unauthorized to update todo list")?;
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
    use sea_orm::ModelTrait;
    let user = auth.user.as_ref().or_unauthorized("Not authenticated")?;
    let todo_list = TodoList::find_by_id(todo_list_id)
        .one(&state.database)
        .await
        .or_internal_server_error("Failed to load todo list")?
        .or_not_found("Todo List not found")?;
    (todo_list.owner_id == user.id).or_unauthorized("Unauthorized to delete todo list")?;
    todo_list
        .delete(&state.database)
        .await
        .or_internal_server_error("Failed to delete todo list")?;

    Ok(NoContent)
}
