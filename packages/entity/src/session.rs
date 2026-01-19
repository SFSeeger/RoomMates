use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "session")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub token: String,
    pub created_at: ChronoDateTimeLocal,
    pub expires_at: ChronoDateTimeLocal,

    // Relation
    pub user_id: i32,
    #[sea_orm(
        belongs_to,
        from = "user_id",
        to = "id",
        on_update = "Restrict",
        on_delete = "Cascade"
    )]
    pub user: HasOne<super::user::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
