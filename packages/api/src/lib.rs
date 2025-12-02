use dioxus::prelude::*;
use std::sync::Arc;

#[cfg(feature = "server")]
use dioxus::server::axum::Extension;

#[cfg(feature = "server")]
pub mod server {
    use super::*;
    use crate::dioxus_fullstack::routing::Router;
    use sea_orm::DatabaseConnection;
    pub mod database;

    pub async fn setup_api(app: fn() -> Element) -> Result<Router, anyhow::Error> {
        let database: DatabaseConnection = database::establish_connection().await;
        database
            .get_schema_registry("entity::*")
            .sync(&database)
            .await?;

        let app_state = AppState { database };
        let router = dioxus::server::router(app).layer(Extension(Arc::new(app_state)));

        Ok(router)
    }
    #[derive(Clone)]
    pub struct AppState {
        pub database: DatabaseConnection,
    }
}

#[post("/api/echo")]
pub async fn echo(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}

#[get("/api/users/{user_id}", ext: Extension<Arc<server::AppState>>)]
pub async fn retrieve_user(user_id: i32) -> Result<entity::user::Model, ServerFnError> {
    use entity::user::Entity as User;
    use sea_orm::EntityTrait;

    let user = User::find_by_id(user_id)
        .one(&ext.database)
        .await
        .or_internal_server_error("Error loading user from database")?
        .or_not_found("User not found")?;

    Ok(user)
}

#[get("/api/events", ext: Extension<Arc<server::AppState>>)]
pub async fn list_events() -> Result<Vec<entity::event::Model>, ServerFnError> {
    use entity::event::Entity as Event;
    use entity::user::Entity as User;
    use sea_orm::EntityTrait;
    use sea_orm::ModelTrait;

    let user = User::find_by_id(1)
        .one(&ext.database)
        .await
        .or_internal_server_error("Error loading user from database")?
        .or_not_found("User not found")?;
    let events = user
        .find_related(Event)
        .all(&ext.database)
        .await
        .or_internal_server_error("Error loading events")?;

    Ok(events)
}

#[post("/api/users", ext: Extension<Arc<server::AppState>>)]
pub async fn create_event(
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
