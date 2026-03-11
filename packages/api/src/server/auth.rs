use crate::routes::users::EMAIL_REGEX;
use crate::routes::users::UserInfo;
use std::env;

use argon2::password_hash::rand_core::RngCore;
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use base64::Engine;
use dioxus::prelude::*;
use entity::prelude::*;

use openidconnect::core::{
    CoreAuthDisplay, CoreAuthPrompt, CoreAuthenticationFlow, CoreClient, CoreErrorResponseType,
    CoreGenderClaim, CoreJsonWebKey, CoreJweContentEncryptionAlgorithm, CoreProviderMetadata,
    CoreRevocableToken, CoreRevocationErrorResponse, CoreTokenIntrospectionResponse,
    CoreTokenResponse,
};
use openidconnect::url::Url;
use openidconnect::{
    AuthorizationCode, Client, ClientId, ClientSecret, CsrfToken, EmptyAdditionalClaims,
    EndpointMaybeSet, EndpointNotSet, EndpointSet, IssuerUrl, Nonce, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, Scope, StandardErrorResponse, reqwest,
};
use regex::Regex;
use sea_orm::sea_query::prelude::Local;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter, Set,
    TryIntoModel,
};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

/// Hashes a password using Argon2
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

/// Verifies a password against a hash
pub fn verify_password(user_password: &str, password_hashed: &str) -> bool {
    let parsed_hash = PasswordHash::new(password_hashed);
    if let Ok(hash) = parsed_hash {
        return Argon2::default()
            .verify_password(user_password.as_bytes(), &hash)
            .is_ok();
    }
    false
}

/// Verifies the user credentials
///
/// # Arguments
/// * `user_password`: Plain text password to verify
/// * `user_email`: Email of the user to verify
/// * `db`: Connection to the database
///
/// returns: Result<entity::user::Model, ServerFnError> - Returns the user model if the credentials are correct or an error
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

pub async fn create_user(
    email: String,
    password: String,
    first_name: String,
    last_name: String,
    database: &DatabaseConnection,
) -> Result<entity::user::Model, ServerFnError> {
    let email = email.trim().to_lowercase();
    let email_regex = Regex::new(EMAIL_REGEX).expect("EMAIL_REGEX must be valid");
    email_regex
        .is_match(&email)
        .or_bad_request("email is not a valid email")?;
    let hashed_password = hash_password(password)?;

    let user = entity::user::ActiveModel {
        email: sea_orm::Set(email),
        password: sea_orm::Set(hashed_password),
        first_name: sea_orm::Set(first_name),
        last_name: sea_orm::Set(last_name),
        ..Default::default()
    };
    let user = user
        .save(database)
        .await
        .or_internal_server_error("Error saving new user to database")?;
    Ok(user
        .try_into_model()
        .or_internal_server_error("Failed to convert active model to model")?)
}

pub(crate) fn build_http_client() -> Result<reqwest::Client, anyhow::Error> {
    let http_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;
    Ok(http_client)
}

pub(crate) type OidcClient = Client<
    EmptyAdditionalClaims,
    CoreAuthDisplay,
    CoreGenderClaim,
    CoreJweContentEncryptionAlgorithm,
    CoreJsonWebKey,
    CoreAuthPrompt,
    StandardErrorResponse<CoreErrorResponseType>,
    CoreTokenResponse,
    CoreTokenIntrospectionResponse,
    CoreRevocableToken,
    CoreRevocationErrorResponse,
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointMaybeSet,
    EndpointMaybeSet,
>;

pub(crate) async fn create_oidc_client() -> Result<OidcClient, anyhow::Error> {
    let http_client = build_http_client()?;

    let provider_metadata = CoreProviderMetadata::discover_async(
        IssuerUrl::new(env::var("OIDC_ISSUER_URL")?)?,
        &http_client,
    )
    .await?;

    let redirect_url = format!("{}/api/oidc/redirect", env::var("SERVER_URL")?);

    let client = CoreClient::from_provider_metadata(
        provider_metadata,
        ClientId::new(env::var("OIDC_CLIENT_ID")?),
        Some(ClientSecret::new(env::var("OIDC_CLIENT_SECRET")?)),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url)?);
    Ok(client)
}

pub struct OidcMetadata {
    pub url: Url,
    pub csrf_token: CsrfToken,
    pub pkce_code_verifier: PkceCodeVerifier,
    pub nonce: Nonce,
}

#[derive(Serialize, Deserialize)]
pub struct OidcSession {
    pub(crate) pkce_code_verifier: PkceCodeVerifier,
    pub(crate) csrf_token: CsrfToken,
    pub(crate) nonce: Nonce,
}

impl From<OidcMetadata> for OidcSession {
    fn from(value: OidcMetadata) -> Self {
        Self {
            pkce_code_verifier: value.pkce_code_verifier,
            nonce: value.nonce,
            csrf_token: value.csrf_token,
        }
    }
}

pub(crate) fn create_oidc_challenge(client: &OidcClient) -> Result<OidcMetadata, anyhow::Error> {
    let (pkce_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
    let mut authorization_request = client.authorize_url(
        CoreAuthenticationFlow::AuthorizationCode,
        CsrfToken::new_random,
        Nonce::new_random,
    );

    let env_scopes = env::var("OAUTH_SCOPES").unwrap_or("openid email profile".to_string());
    let scopes = env_scopes.split_whitespace();

    for scope in scopes {
        authorization_request = authorization_request.add_scope(Scope::new(scope.to_string()));
    }
    authorization_request = authorization_request.set_pkce_challenge(pkce_challenge);

    let (auth_url, csrf_token, nonce) = authorization_request.url();

    let metadata = OidcMetadata {
        url: auth_url,
        pkce_code_verifier,
        nonce,
        csrf_token,
    };
    Ok(metadata)
}

pub async fn verify_oidc_challenge(
    client: &OidcClient,
    authorization_code: String,
    pkce_code_verifier: PkceCodeVerifier,
) -> Result<CoreTokenResponse, anyhow::Error> {
    let http_client = build_http_client()?;
    let token = client
        .exchange_code(AuthorizationCode::new(authorization_code))?
        .set_pkce_verifier(pkce_code_verifier)
        .request_async(&http_client)
        .await?;
    Ok(token)
}

/// Hashes a session key using blake3
///
/// # Arguments
///
/// * `session_key`: Session key. Can be any string
///
/// returns: String
fn hash_session_key(session_key: &str) -> String {
    let hash = blake3::hash(session_key.as_bytes());
    hash.to_hex().to_string()
}

/// Creates a new random session key with 256 bits.
///
/// # Arguments
///
/// returns: String - A Base64 encoded session key
pub fn create_session_key() -> String {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    base64::prelude::BASE64_URL_SAFE.encode(bytes)
}

const SESSION_EXPIRATION_DURATION: i64 = 5;

/// Creates a new session in the database and links it to the supplied user
///
/// # Arguments
///
/// * `user_id`: Id of the user the new session will belong to
/// * `db`: Connection to the database
///
/// returns: Result<(String, DateTime<Local>), Error> - Tuple of the plain text session key and the expiration date.
///     Returns an error when saving into the database fails
pub async fn create_session(
    user_id: &i32,
    db: &DatabaseConnection,
) -> Result<(String, OffsetDateTime), anyhow::Error> {
    let session_key = create_session_key();
    let session_key_hash = hash_session_key(&session_key);

    let now = OffsetDateTime::now_local()?;

    let expires_at = now + Duration::hours(SESSION_EXPIRATION_DURATION);

    let session = entity::session::ActiveModel {
        token: Set(session_key_hash),
        created_at: Set(now),
        expires_at: Set(expires_at),
        user_id: Set(*user_id),
        ..Default::default()
    };
    session.save(db).await?;

    Ok((session_key, expires_at))
}

/// Searches the database for a user based on the unhashed session key. Returns `Ok(None)` if the session is expired
///
/// # Arguments
///
/// * `session_key`: plain session key
/// * `db`: Connection to the database
///
/// returns: Result<Option<(entity::user::Model, i32)>, Error> - (User, Session id) when the session is valid and has a lined user,
///     otherwise none. Returns an error, if the database operation fails.
pub async fn find_user_by_session(
    session_key: &str,
    db: &DatabaseConnection,
) -> Result<Option<(entity::user::Model, i32)>, anyhow::Error> {
    let hashed_session_key = hash_session_key(session_key);
    let session = Session::find()
        .filter(entity::session::Column::Token.eq(&hashed_session_key))
        .filter(entity::session::Column::ExpiresAt.gt(Local::now()))
        .one(db)
        .await?;
    if let Some(session) = session {
        return Ok(session
            .find_related(User)
            .one(db)
            .await?
            .map(|user| (user, session.id)));
    };
    Ok(None)
}

pub async fn find_user_by_email(
    email: String,
    db: &DatabaseConnection,
) -> dioxus::Result<UserInfo, ServerFnError> {
    use crate::routes::users::UserInfo;
    use entity::user::Entity as User;
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

    let user_option = User::find()
        .filter(entity::user::Column::Email.eq(&email))
        .one(db)
        .await
        .or_internal_server_error("Error loading user from database")?;

    let user = user_option.or_not_found("Cannot find user with this email")?;
    Ok(UserInfo::from_user_model(user))
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

    #[test]
    fn test_hash_session_key() {
        assert_eq!(
            "5865052a0e08e53ace9c8fc16261fb74aa85a2aabf56e1c00a36e4d7a9ac450d",
            hash_session_key("QzbJt8GqDJYlvuLSFiP9X144cqntnQXW5jgjHXHLyNY=")
        )
    }
}
