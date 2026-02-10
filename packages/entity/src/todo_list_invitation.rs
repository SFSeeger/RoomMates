use form_hooks::prelude::*;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "todo_list_invitation")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    pub permissions: InvitationPermission,
    pub is_accepted: bool,

    pub receiving_user_id: i32,
    #[sea_orm(
        belongs_to,
        relation_enum = "Receiver",
        from = "receiving_user_id",
        to = "id"
    )]
    pub receiver: HasOne<super::user::Entity>,

    pub sender_user_id: i32,
    #[sea_orm(
        belongs_to,
        relation_enum = "Sender",
        from = "sender_user_id",
        to = "id"
    )]
    pub sender: HasOne<super::user::Entity>,

    pub todo_list_id: i32,
    #[sea_orm(belongs_to, from = "todo_list_id", to = "id")]
    pub todo_list: HasOne<super::todo_list::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(
    EnumIter,
    DeriveActiveEnum,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Default,
    Deserialize,
    Serialize,
    FieldValue,
    EnumSelect,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "InvitationStatus")]
pub enum InvitationPermission {
    #[default]
    #[sea_orm(string_value = "Read")]
    Read,
    #[sea_orm(string_value = "Write")]
    Write,
    #[sea_orm(string_value = "Admin")]
    Admin,
}
