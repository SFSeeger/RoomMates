use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "is_in_group")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub group_id: i32,

    #[sea_orm(belongs_to, from = "user_id", to = "id", on_delete = "Cascade")]
    pub user: Option<super::user::Entity>,
    #[sea_orm(belongs_to, from = "group_id", to = "id", on_delete = "Cascade")]
    pub group: Option<super::group::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
