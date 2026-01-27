use chrono::{NaiveDate, NaiveTime};
use form_hooks::EnumSelectDefault;
use form_hooks::prelude::{EnumSelect, FieldValue};
use sea_orm::DeriveIntoActiveModel;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use std::string::ToString;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "event")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    pub reoccurring: bool,
    pub private: bool,

    #[sea_orm(nullable)]
    pub description: Option<String>,
    #[sea_orm(nullable)]
    pub location: Option<String>,

    pub date: ChronoDate,
    pub start_time: ChronoTime,
    pub end_time: ChronoTime,
    pub weekday: Weekday,

    // Relation
    //belongs to this user
    pub owner_id: i32,
    #[sea_orm(
        belongs_to,
        from = "owner_id",
        to = "id",
        on_update = "Restrict",
        on_delete = "Cascade"
    )]
    pub user: HasOne<super::user::Entity>,

    //groups that this event is shared with
    #[sea_orm(has_many, via = "shared_group_event")]
    pub groups: HasMany<super::group::Entity>,
    //individual users that the event is shared with found through linked

    //invitations that were sent for this event
    #[sea_orm(has_many)]
    pub invitations: HasMany<super::invitation::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(
    EnumIter,
    DeriveActiveEnum,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Deserialize,
    Serialize,
    FieldValue,
    EnumSelect,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "Weekday")]
pub enum Weekday {
    #[sea_orm(string_value = "Monday")]
    Monday,
    #[sea_orm(string_value = "Tuesday")]
    Tuesday,
    #[sea_orm(string_value = "Wednesday")]
    Wednesday,
    #[sea_orm(string_value = "Thursday")]
    Thursday,
    #[sea_orm(string_value = "Friday")]
    Friday,
    #[sea_orm(string_value = "Saturday")]
    Saturday,
    #[sea_orm(string_value = "Sunday")]
    Sunday,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug, DeriveIntoActiveModel)]
pub struct PartialEventModel {
    pub title: String,
    pub reoccurring: bool,
    pub private: bool,
    pub description: Option<String>,
    pub location: Option<String>,
    pub date: NaiveDate,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub weekday: Weekday,
}
