use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use dioxus::prelude::*;
use sea_orm::DatabaseConnection;

pub fn hash_password(user_password: String) -> Result<String, ServerFnError> {
    let salt = SaltString::generate(&mut OsRng);
    match Argon2::default().hash_password(user_password.as_bytes(), &salt) {
        Ok(password_hashed) => Ok(password_hashed.to_string()),
        Err(_) => Err(ServerFnError::ServerError {
            message: "Server Error".to_string(),
            code: 500,
            details: None,
        }),
    }
}

pub fn verify_password(user_password: &str, password_hashed: &str) -> bool {
    let parsed_hash = PasswordHash::new(password_hashed);
    if let Ok(hash) = parsed_hash {
        return Argon2::default()
            .verify_password(user_password.as_bytes(), &hash)
            .is_ok();
    }
    false
}

pub async fn verify_user(
    user_password: &str,
    user_email: &str,
    db: &DatabaseConnection,
) -> Result<entity::user::Model, ServerFnError> {
    use entity::user;
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

    let user = user::Entity::find()
        .filter(user::Column::Email.eq(user_email))
        .one(db)
        .await
        .or_unauthorized("Missing or incorrect Credentials")?
        .or_not_found("User not found")?;

    let validated_password = verify_password(user_password, &user.password);

    if !validated_password {
        return Err(ServerFnError::ServerError {
            message: "Unauthorized".to_string(),
            code: 401,
            details: None,
        });
    }

    Ok(user)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn hashing_and_validation_work() {
        let password = "hello world".to_string();
        let password_hashed = hash_password(password).expect("hashing failed");
        let validate = verify_password("hello world", &password_hashed);
        assert!(validate)
    }

    #[tokio::test]
    async fn hashing_and_validation_fail() {
        let password_hashed = hash_password("hello world".to_string()).expect("hashing failed");
        let validate = verify_password("not hello world", &password_hashed);
        assert!(!validate)
    }
}
