use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,

    // Relation
    #[sea_orm(has_many)]
    pub events: HasMany<super::event::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
