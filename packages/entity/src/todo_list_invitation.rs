use form_hooks::prelude::*;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "todo_list_invitation")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub todo_list_id: i32,
    #[sea_orm(belongs_to, from = "todo_list_id", to = "id", on_delete = "Cascade")]
    pub todo_list: HasOne<super::todo_list::Entity>,

    #[sea_orm(primary_key)]
    pub receiving_user_id: i32,
    #[sea_orm(
        belongs_to,
        relation_enum = "Receiver",
        from = "receiving_user_id",
        to = "id",
        on_delete = "Cascade"
    )]
    pub receiver: HasOne<super::user::Entity>,

    pub sender_user_id: Option<i32>,
    #[sea_orm(
        belongs_to,
        relation_enum = "Sender",
        from = "sender_user_id",
        to = "id",
        on_delete = "SetNull"
    )]
    pub sender: HasOne<super::user::Entity>,

    pub permission: InvitationPermission,
    pub is_accepted: bool,
    pub is_favorite: bool,
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

impl InvitationPermission {
    #[must_use]
    pub fn can_write(self) -> bool {
        matches!(self, Self::Write | Self::Admin)
    }

    #[must_use]
    pub fn can_admin(self) -> bool {
        matches!(self, Self::Admin)
    }
}

impl Display for InvitationPermission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read => write!(f, "Read"),
            Self::Write => write!(f, "Write"),
            Self::Admin => write!(f, "Admin"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Default, Debug, DerivePartialModel)]
#[sea_orm(entity = "Entity")]
pub struct TodoListInvitationPartialModel {
    pub permission: InvitationPermission,
    pub is_accepted: bool,
    pub is_favorite: bool,
}

#[derive(Serialize, Deserialize, Default, Debug, DeriveIntoActiveModel)]
pub struct UpdateTodoListInvitation {
    #[serde(default)]
    pub permission: Option<InvitationPermission>,
}
#[derive(Serialize, Deserialize, Default, Debug, DeriveIntoActiveModel)]
pub struct UpdateMyTodoListInvitation {
    #[serde(default)]
    pub is_favorite: Option<bool>,
}
