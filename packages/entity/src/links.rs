use crate::shared_friend_event;
use sea_orm::RelationDef;
use sea_orm::RelationTrait;

//events shared with the user by other users
pub struct FriendEvents;

impl sea_orm::Linked for FriendEvents {
    type FromEntity = super::user::Entity;
    type ToEntity = super::event::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            shared_friend_event::Relation::User.def().rev(), // cake -> cake_filling
            shared_friend_event::Relation::Event.def(),      // cake_filling -> filling
        ]
    }
}

pub struct EventUserMembers;

impl sea_orm::Linked for EventUserMembers {
    type FromEntity = super::event::Entity;
    type ToEntity = super::user::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            shared_friend_event::Relation::Event.def().rev(), // cake -> cake_filling
            shared_friend_event::Relation::User.def(),        // cake_filling -> filling
        ]
    }
}
