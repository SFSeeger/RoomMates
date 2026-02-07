use sea_orm::DeriveIntoActiveModel;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "todo")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    pub completed: bool,
    #[sea_orm(nullable)]
    pub details: Option<String>,

    //Relations to others
    pub todo_list_id: i32,
    #[sea_orm(
        belongs_to,
        from = "todo_list_id",
        to = "id",
        on_update = "Restrict",
        on_delete = "Cascade"
    )]
    pub todo_list: HasOne<super::todo_list::Entity>,

    pub owner_id: i32,
    #[sea_orm(
        belongs_to,
        from = "owner_id",
        to = "id",
        on_update = "Restrict",
        on_delete = "Cascade"
    )]
    pub user: HasOne<super::user::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Serialize, Deserialize, Default, DeriveIntoActiveModel)]
pub struct CreateToDo {
    pub title: String,
    pub details: Option<String>,
    pub completed: bool,
}

#[derive(Serialize, Deserialize, Default, DeriveIntoActiveModel)]
pub struct UpdateToDo {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub details: Option<Option<String>>,
    #[serde(default)]
    pub completed: Option<bool>,
}
