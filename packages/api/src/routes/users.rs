use crate::dioxus_fullstack::NoContent;
use crate::server;
use dioxus::fullstack::{SetCookie, SetHeader};
use dioxus::prelude::*;

#[cfg(feature = "server")]
use dioxus::server::axum::Extension;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct UserInfo {
    pub id: i32,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

impl UserInfo {
    pub fn from_user_model(user: entity::user::Model) -> Self {
        UserInfo {
            id: user.id,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
        }
    }
}

#[get("/api/users/{user_id}", ext: Extension<server::AppState>)]
pub async fn retrieve_user(user_id: i32) -> dioxus::Result<UserInfo, ServerFnError> {
    use entity::user::Entity as User;
    use sea_orm::EntityTrait;

    let user = User::find_by_id(user_id)
        .one(&ext.database)
        .await
        .or_internal_server_error("Error loading user from database")?
        .or_not_found("User not found")?;
    Ok(UserInfo::from_user_model(user))
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

pub const EMAIL_REGEX: &str = r"^[\w+.-]*\w@[\w.-]+\.\w+$";

#[post("/api/users/signup", ext: Extension<server::AppState>)]
pub async fn sign_up(email: String, password: String) -> dioxus::Result<UserInfo, ServerFnError> {
    use entity::user::Entity as User;
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

    let email_regex = Regex::new(EMAIL_REGEX).expect("EMAIL_REGEX must be valid");
    email_regex
        .is_match(&email)
        .or_bad_request("email is not a valid email")?;

    let email = email.trim().to_lowercase();

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
        Ok(UserInfo::from_user_model(user))
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

    let email = email.trim().to_lowercase();

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

#[post("/api/logout", ext: Extension<server::AppState>, mut auth: Extension<server::AuthenticationState>)]
pub async fn logout() -> Result<NoContent, ServerFnError> {
    auth.logout(&ext.database).await?;
    Ok(NoContent)
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

#[get("/api/me", auth: Extension<server::AuthenticationState>)]
pub async fn get_me() -> Result<UserInfo, ServerFnError> {
    let auth_user = auth.user.clone().or_unauthorized("Not authenticated")?;
    Ok(UserInfo::from_user_model(auth_user))
}
