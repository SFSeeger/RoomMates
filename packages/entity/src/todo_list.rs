use sea_orm::DeriveIntoActiveModel;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "todo_list")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    #[sea_orm(nullable)]
    pub description: Option<String>,
    pub is_favorite: bool,

    //relations
    pub owner_id: i32,
    #[sea_orm(
        belongs_to,
        from = "owner_id",
        to = "id",
        on_update = "Restrict",
        on_delete = "Cascade"
    )]
    pub user: HasOne<super::user::Entity>,

    #[sea_orm(has_many)]
    pub todos: HasMany<super::todo::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Serialize, Deserialize, Default, DeriveIntoActiveModel)]
pub struct CreateTodoList {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Default, DeriveIntoActiveModel)]
pub struct UpdateTodoList {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub description: Option<Option<String>>,
    #[serde(default)]
    pub is_favorite: Option<bool>,
}
