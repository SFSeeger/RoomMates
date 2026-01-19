use crate::dioxus_fullstack::NoContent;
use crate::server;
use dioxus::prelude::*;
use entity::prelude::*;

#[cfg(feature = "server")]
use dioxus::server::axum::Extension;

#[get("/api/groups", ext: Extension<server::AppState>)]
pub async fn list_groups() -> Result<Vec<entity::group::Model>, ServerFnError> {
    use sea_orm::EntityTrait;
    use sea_orm::ModelTrait;

    let user = User::find_by_id(1)
        .one(&ext.database)
        .await
        .or_internal_server_error("Error loading user from database")?
        .or_not_found("User not found")?;
    let groups = user
        .find_related(Group)
        .all(&ext.database)
        .await
        .or_internal_server_error("Error loading groups")?;

    Ok(groups)
}

#[post("/api/groups", ext: Extension<server::AppState>)]
pub async fn add_user_to_table(user_id: i32, group_id: i32) -> Result<NoContent, ServerFnError> {
    use entity::is_in_group;
    use sea_orm::{ActiveModelTrait, Set};

    let pair = is_in_group::ActiveModel {
        user_id: Set(user_id),
        group_id: Set(group_id),
    };

    let _pair: is_in_group::ActiveModel = pair
        .insert(&ext.database)
        .await
        .or_internal_server_error("Error inserting pair into database")?
        .into();

    Ok(NoContent)
}

#[post("/api/groups/{group_id}/add_user", ext: Extension<server::AppState>)]
pub async fn add_user_to_group(email: i32, group_id: i32) -> Result<NoContent, ServerFnError> {
    use entity::user::Entity as User;
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

    let user = User::find()
        .filter(entity::user::Column::Email.eq(email))
        .one(&ext.database)
        .await
        .or_internal_server_error("Error loading user from database")?
        .or_not_found("User not found")?;

    add_user_to_table(user.id, group_id).await?;

    Ok(NoContent)
}
