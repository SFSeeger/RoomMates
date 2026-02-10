use super::{InviteMemberForm, MemberEntry};
use crate::components::ui::card::{Card, CardBody, CardTitle};
use crate::components::ui::dialog::{Dialog, DialogContent, DialogTrigger};
use crate::components::ui::list::{List, ListDetails, ListRow};
use crate::views::todo::todos_group::use_todo_list;
use api::routes::todo_list::list_todo_list_members;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::LdUserRoundPlus;
use dioxus_sdk::time::use_timeout;
use entity::user::UserWithTodoListInvitation;
use std::time::Duration;

#[derive(Clone, Copy, PartialEq)]
pub struct MembersContext {
    update_member: EventHandler<UserWithTodoListInvitation>,
}

impl MembersContext {
    pub fn new(update_member: EventHandler<UserWithTodoListInvitation>) -> Self {
        Self { update_member }
    }

    pub fn update_member(&self, member: UserWithTodoListInvitation) {
        self.update_member.call(member);
    }
}

#[component]
pub fn MemberList() -> Element {
    let todo_list_context = use_todo_list();
    let todo_list_id = todo_list_context.todo_list().id;

    let mut members = use_resource(move || list_todo_list_members(todo_list_id));
    let update_member = EventHandler::new(move |member: UserWithTodoListInvitation| {
        if let Some(Ok(members_mut)) = members.write().as_mut()
            && let Some(m) = members_mut.iter_mut().find(|m| m.id == member.id)
        {
            *m = member;
        }
    });

    use_context_provider(|| MembersContext::new(update_member));

    let mut show_skeleton = use_signal(|| false);
    let timeout = use_timeout(Duration::from_millis(250), move |()| {
        show_skeleton.set(true);
    });

    use_effect(move || {
        timeout.action(());
    });

    let onmemberchange = move || {
        members.restart();
        show_skeleton.set(false);
        timeout.action(());
    };

    rsx! {
        Card { class: "shrink-0 w-full lg:w-1/2 xl:w-1/3",
            CardBody {
                CardTitle { class: "flex items-center justify-between",
                    "Members"
                    if todo_list_context.permission().can_admin() {
                        Dialog {
                            DialogTrigger {
                                Icon { icon: LdUserRoundPlus }
                            }
                            DialogContent {
                                title: "Invite a User to this To-Do List",
                                dismissible: false,
                                close_button: false,
                                InviteMemberForm { onmemberinvited: onmemberchange }
                            }
                        }
                    }
                }
                List { header: "",
                    match members.read().as_ref() {
                        Some(Ok(members)) => rsx! {
                            for member in members.iter() {
                                MemberEntry {
                                    key: "{member.id}",
                                    member: member.clone(),
                                    onmemberkick: onmemberchange,
                                }
                            }
                        },
                        Some(Err(_)) => rsx! {
                            ListRow {
                                ListDetails { title: "Failed to load members" }
                            }
                        },
                        None => rsx! {
                            for i in 0..5 {
                                ListRow { key: "{i}", class: if *show_skeleton.read() { "skeleton mb-1" } else { "mb-1" },
                                    div { class: " h-4 w-full" }
                                }
                            }
                        },
                    }
                }
            }
        }
    }
}
