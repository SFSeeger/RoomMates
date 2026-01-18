use crate::dioxus_fullstack::NoContent;
use crate::server;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use dioxus::server::axum::Extension;

#[get("/api/users/{user_id}", ext: Extension<server::AppState>)]
pub async fn retrieve_user(user_id: i32) -> dioxus::Result<entity::user::Model, ServerFnError> {
    use entity::user::Entity as User;
    use sea_orm::EntityTrait;

    let user = User::find_by_id(user_id)
        .one(&ext.database)
        .await
        .or_internal_server_error("Error loading user from database")?
        .or_not_found("User not found")?;
    Ok(user)
}

#[post("/api/users", ext: Extension<server::AppState>)]
pub async fn create_user(
    email: String,
    password: String,
) -> Result<entity::user::Model, ServerFnError> {
    use sea_orm::{ActiveModelTrait, TryIntoModel};

    let user = entity::user::ActiveModel {
        email: sea_orm::Set(email),
        password: sea_orm::Set(password),
        ..Default::default()
    };

    let user = user
        .save(&ext.database)
        .await
        .or_internal_server_error("Error saving new user to database")?;
    Ok(user
        .try_into_model()
        .or_internal_server_error("Error converting user to model")?)
}

#[post("/api/users/sign_up", ext: Extension<server::AppState>)]
pub async fn sign_up(
    email: String,
    password: String,
) -> dioxus::Result<entity::user::Model, ServerFnError> {
    use entity::user::Entity as User;
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

    let user_check = User::find()
        .filter(entity::user::Column::Email.eq(&email))
        .one(&ext.database)
        .await
        .or_internal_server_error("Error loading user from database")?;

    if user_check.is_some() {
        Err(ServerFnError::ServerError {
            message: ("Email already registered".to_string()),
            code: (409),
            details: None,
        })
    } else {
        let user = create_user(email, password)
            .await
            .or_internal_server_error("Error creating user")?;
        Ok(user)
    }
}

#[post("/api/users/sign_in", ext: Extension<server::AppState>)]
pub async fn sign_in(
    email: String,
    password: String,
) -> dioxus::Result<entity::user::Model, ServerFnError> {
    let verified_user = crate::server::auth::verify_user(&password, &email, &ext.database)
        .await
        .or_unauthorized("Missing or incorrect Credentials")?;

    Ok(verified_user)
}

#[delete("/api/users/{user_id}", ext: Extension<server::AppState>)]
pub async fn delete_user(user_id: i32) -> dioxus::Result<NoContent, ServerFnError> {
    use entity::user::Entity as User;
    use sea_orm::EntityTrait;
    let delete_result = User::delete_by_id(user_id)
        .exec(&ext.database)
        .await
        .or_internal_server_error("Error deleting user")?;

    (delete_result.rows_affected == 1).or_not_found("User not found")?;
    Ok(NoContent)
}
