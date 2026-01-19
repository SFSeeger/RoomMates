use crate::dioxus_fullstack::NoContent;
use crate::server;
use dioxus::fullstack::{SetCookie, SetHeader};
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
    use crate::server::auth::hash_password;
    use sea_orm::{ActiveModelTrait, TryIntoModel};

    let hashed_password = hash_password(password)?;

    let user = entity::user::ActiveModel {
        email: sea_orm::Set(email),
        password: sea_orm::Set(hashed_password),
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

#[post("/api/users/signup", ext: Extension<server::AppState>)]
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

#[post("/api/users/login", ext: Extension<server::AppState>, auth: Extension<server::AuthenticationState>)]
pub async fn login(email: String, password: String) -> Result<SetHeader<SetCookie>, ServerFnError> {
    use crate::server::auth::{create_session, verify_user};

    if auth.is_authenticated() {
        return Err(ServerFnError::ServerError {
            message: "Already logged in".to_string(),
            code: 409,
            details: None,
        });
    }

    let verified_user = verify_user(&password, &email, &ext.database)
        .await
        .or_unauthorized("Missing or incorrect Credentials")?;

    let (session_key, expires_at) = create_session(&verified_user.id, &ext.database)
        .await
        .or_internal_server_error("Error creating session")?;

    Ok(SetHeader::new(format!(
        "session={}; HttpOnly; Expires={}; Path=/",
        session_key,
        expires_at.to_rfc2822()
    ))
    .or_internal_server_error("Error setting session cookie")?)
}

#[delete("/api/users/{user_id}", ext: Extension<server::AppState>, auth: Extension<server::AuthenticationState>)]
pub async fn delete_user(user_id: i32) -> Result<NoContent, ServerFnError> {
    use entity::user::Entity as User;
    use sea_orm::EntityTrait;

    (auth.is_authenticated()
        && auth
            .user
            .as_ref()
            .expect("Impossible panic. User is authenticated")
            .id
            == user_id)
        .or_unauthorized("Invalid authentication")?;

    let delete_result = User::delete_by_id(user_id)
        .exec(&ext.database)
        .await
        .or_internal_server_error("Error deleting user")?;

    (delete_result.rows_affected == 1).or_not_found("User not found")?;
    Ok(NoContent)
}
