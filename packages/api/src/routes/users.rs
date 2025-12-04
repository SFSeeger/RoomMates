use dioxus::prelude::*;

use crate::server;
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
