use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "group")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,

    //relations

    //has many users as members
    #[sea_orm(has_many, via = "is_in_group")]
    pub members: HasMany<super::user::Entity>,

    //events shared with the group
    #[sea_orm(has_many, via = "shared_group_event")]
    pub shared_events: HasMany<super::event::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
