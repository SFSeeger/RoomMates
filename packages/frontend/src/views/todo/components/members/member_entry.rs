use super::MemberPermissionEditButton;
use crate::Route;
use crate::components::contexts::use_auth;
use crate::components::tooltip::Tooltip;
use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::dialog::{Dialog, DialogAction, DialogContent, DialogTrigger};
use crate::components::ui::list::{ComplexListDetails, ListRow};
use crate::components::ui::toaster::{ToastOptions, use_toaster};
use crate::views::todo::todos_group::use_todo_list;
use api::routes::todo_list::{invite::leave_todo_list, remove_user_from_todo_list};
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{LdLogOut, LdUserRoundMinus};
use entity::user::UserWithTodoListInvitation;
use roommates::message_from_captured_error;

#[component]
pub fn MemberEntry(member: UserWithTodoListInvitation, onmemberkick: EventHandler<()>) -> Element {
    let auth = use_auth();
    let nav = use_navigator();
    let mut toaster = use_toaster();

    let todo_list_context = use_todo_list();
    let todo_list = todo_list_context.todo_list();
    let todo_list_id = todo_list.id;

    let mut kick_user = use_action(move |user_id| async move {
        remove_user_from_todo_list(todo_list_id, user_id).await
    });
    let mut leave_list = use_action(move || async move { leave_todo_list(todo_list_id).await });

    rsx! {
        ListRow {
            ComplexListDetails {
                title: rsx! {
                    h3 { class: "flex flex-wrap items-center gap-2",
                        "{member.first_name} {member.last_name}"
                        MemberPermissionEditButton { member: member.clone() }
                        if !member.invitation.is_accepted {
                            span { class: "badge badge-outline badge-info badge-md", "Invited" }
                        }
                    }
                },
            }
            if let Some(user) = auth.user.peek().as_ref() && user.id == member.id {
                Dialog {
                    Tooltip { tooltip: "Leave To-Do List",
                        DialogTrigger { variant: ButtonVariant::Error, ghost: true,
                            Icon { icon: LdLogOut }
                        }
                    }
                    DialogContent { title: "Are you sure you want to leave this To-Do List?",
                        DialogAction {
                            form { method: "dialog",
                                Button { variant: ButtonVariant::Secondary, "Cancel" }
                                Button {
                                    onclick: move |_| async move {
                                        leave_list.call().await;
                                        if let Some(Err(error)) = leave_list.value().as_ref() {
                                            toaster
                                                .error(
                                                    "Failed to leave To-Do List",
                                                    ToastOptions::new().description(rsx! { "{message_from_captured_error(error)}" }),
                                                );
                                        } else {
                                            nav.push(Route::TodoListListView {});
                                        }
                                    },
                                    variant: ButtonVariant::Error,
                                    "Leave"
                                }
                            }
                        }
                    }
                }
            } else {
                if todo_list_context.permission().can_admin() {
                    Dialog {
                        Tooltip { tooltip: "Kick User",
                            DialogTrigger { variant: ButtonVariant::Error, ghost: true,
                                Icon { icon: LdUserRoundMinus }
                            }
                        }
                        DialogContent { title: "Are you sure you want to Kick {member.first_name} {member.last_name}?",
                            DialogAction {
                                form { method: "dialog",
                                    Button { variant: ButtonVariant::Secondary, "Cancel" }
                                    Button {
                                        onclick: move |_| async move {
                                            kick_user.call(member.id).await;
                                            match kick_user.value().as_ref() {
                                                Some(Ok(_)) => {
                                                    toaster
                                                        .success(
                                                            "User Kicked",
                                                            ToastOptions::new().description(rsx! { "User has been kicked from the To-Do List" }),
                                                        );
                                                    onmemberkick.call(());
                                                }
                                                Some(Err(error)) => {
                                                    toaster
                                                        .error(
                                                            "Failed to kick user",
                                                            ToastOptions::new().description(rsx! { "{message_from_captured_error(error)}" }),
                                                        );
                                                }
                                                None => {
                                                    warn!("Request still pending...");
                                                }
                                            }
                                        },
                                        variant: ButtonVariant::Error,
                                        "Kick"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
