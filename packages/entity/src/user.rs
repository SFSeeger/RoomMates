use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    #[serde(skip)]
    pub password: String,

    //events that belong to this user
    #[sea_orm(has_many)]
    pub my_events: HasMany<super::event::Entity>,

    //belongs to many groups
    #[sea_orm(has_many, via = "is_in_group")]
    pub groups: HasMany<super::group::Entity>,

    //has gotten many invitations
    #[sea_orm(has_many)]
    pub invitations: HasMany<super::invitation::Entity>,
    //events shared by other users found through linked

    // Sessions owned by this user
    #[sea_orm(has_many)]
    pub sessions: HasMany<super::session::Entity>,

    // TodoLists owned by this user
    #[sea_orm(has_many)]
    pub todo_lists: HasMany<super::todo_list::Entity>,

    //Todos owned by this user
    #[sea_orm(has_many)]
    pub todos: HasMany<super::todo::Entity>,

    // TodoLists Invitation Relations
    #[sea_orm(
        has_many,
        relation_enum = "TodoListInvitationSender",
        via_rel = "Sender"
    )]
    pub send_todo_list_invitations: HasMany<super::todo_list_invitation::Entity>,

    #[sea_orm(
        has_many,
        relation_enum = "TodoListInvitationReceiver",
        via_rel = "Receiver"
    )]
    pub received_todo_list_invitations: HasMany<super::todo_list_invitation::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
