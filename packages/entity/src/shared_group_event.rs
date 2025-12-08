use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "shared_group_event")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub event_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub group_id: i32,

    #[sea_orm(belongs_to, from = "event_id", to = "id")]
    pub event: Option<super::event::Entity>,
    #[sea_orm(belongs_to, from = "group_id", to = "id")]
    pub group: Option<super::group::Entity>,
    //whether the contents of the event is visible to this group
    //pub is_private: bool,
}

impl ActiveModelBehavior for ActiveModel {}
