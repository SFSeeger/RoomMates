use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "invitation")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    pub status: InvitationStatus,

    pub recieving_user: i32,
    #[sea_orm(belongs_to, from = "recieving_user", to = "id")]
    pub to_user: HasOne<super::user::Entity>,

    pub event_id: i32,
    #[sea_orm(belongs_to, from = "event_id", to = "id")]
    pub event: HasOne<super::event::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(EnumIter, DeriveActiveEnum, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "InvitationStatus")]
pub enum InvitationStatus {
    #[sea_orm(string_value = "Sent")]
    Sent,
    #[sea_orm(string_value = "Accepted")]
    Accepted,
    #[sea_orm(string_value = "Declined")]
    Declined,
}
