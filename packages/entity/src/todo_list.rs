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

    //relations
    pub created_by_id: Option<i32>,
    #[sea_orm(
        belongs_to,
        from = "created_by_id",
        to = "id",
        on_update = "Restrict",
        on_delete = "SetNull"
    )]
    pub created_by: HasOne<super::user::Entity>,

    #[sea_orm(has_many)]
    pub todos: HasMany<super::todo::Entity>,

    #[sea_orm(has_many)]
    pub invitations: HasMany<super::todo_list_invitation::Entity>,
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
}

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, DerivePartialModel)]
#[sea_orm(entity = "Entity")]
pub struct TodoListWithPermission {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    #[sea_orm(nested)]
    pub invitation: super::todo_list_invitation::TodoListInvitationPartialModel,
}
