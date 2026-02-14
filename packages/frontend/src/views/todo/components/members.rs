use crate::Route;
use crate::components::contexts::use_auth;
use crate::components::tooltip::Tooltip;
use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::card::{Card, CardBody, CardTitle};
use crate::components::ui::dialog::{
    Dialog, DialogAction, DialogContent, DialogTrigger, use_dialog,
};
use crate::components::ui::form::input::Input;
use crate::components::ui::form::select::Select;
use crate::components::ui::form::submit_button::SubmitButton;
use crate::components::ui::list::{List, ListDetails, ListRow};
use crate::components::ui::toaster::{ToastOptions, use_toaster};
use crate::views::todo::todos_group::use_todo_list;
use api::routes::todo_list::{
    InviteToTodoListData, invite_to_todo_list, leave_todo_list, list_todo_list_members,
    remove_user_from_todo_list,
};
use api::routes::users::EMAIL_REGEX;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{LdLogOut, LdMail, LdUserRoundMinus, LdUserRoundPlus};
use dioxus_sdk::time::use_timeout;
use entity::todo_list_invitation::InvitationPermission;
use form_hooks::prelude::{use_form, use_form_field, use_on_submit};
use form_hooks::validators;
use regex::Regex;
use std::time::Duration;

#[component]
pub fn MemberList(todo_list_id: i32, user_permission: InvitationPermission) -> Element {
    let mut members = use_resource(move || list_todo_list_members(todo_list_id));
    let mut show_skeleton = use_signal(|| false);
    let timeout = use_timeout(Duration::from_millis(100), move |()| {
        show_skeleton.set(true);
    });

    use_effect(move || {
        timeout.action(());
    });

    let onmemberkick = move || {
        members.restart();
        show_skeleton.set(false);
        timeout.action(());
    };

    rsx! {
        Card { class: "w-full md:w-1/3",
            CardBody {
                CardTitle { class: "flex justify-between",
                    "Members"
                    Dialog {
                        DialogTrigger {
                            Icon { icon: LdUserRoundPlus }
                        }
                        DialogContent { title: "Invite a User to this To-Do List", InviteMemberForm {} }
                    }
                }
                List { header: "",
                    match members.read().as_ref() {
                        Some(Ok(members)) => rsx! {
                            for member in members.iter() {
                                MemberEntry { key: "{member.id}", member: member.clone(), onmemberkick }
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

#[component]
fn InviteMemberForm() -> Element {
    let dialog = use_dialog();
    let mut toaster = use_toaster();
    let todo_list_context = use_todo_list();
    let todo_list_id = todo_list_context.todo_list().id;

    let mut invite_user =
        use_action(move |data| async move { invite_to_todo_list(todo_list_id, data).await });

    let mut form_state = use_form();
    let email = use_form_field("email", String::new()).with_validator(validators::pattern(
        Regex::new(EMAIL_REGEX)?,
        "Email must be valid!",
    ));
    let permission = use_form_field("permission", InvitationPermission::Read);
    form_state.register_field(&email);
    form_state.register_field(&permission);

    form_state.revalidate();

    let mut form_state_clone = form_state.clone();

    let onsubmit = use_on_submit(&form_state, move |mut form_state| async move {
        let data: InviteToTodoListData = form_state.parsed_values().unwrap();
        invite_user.call(data).await;
        match invite_user.value().as_ref() {
            Some(Ok(_)) => {
                toaster.success("Invited User successfully", ToastOptions::new());
                dialog.close();
                form_state.reset();
            }
            Some(Err(error)) => {
                toaster.error(
                    "Inviting user failed!",
                    ToastOptions::new().description(rsx! { "{error}" }),
                );
            }
            None => {
                warn!("Invite user request did not finish!");
            }
        }
    });

    rsx! {
        form { onsubmit,
            Input {
                label: "Email",
                icon: rsx! {
                    Icon { icon: LdMail }
                },
                field: email,
            }
            Select { label: "Permission", field: permission }
            DialogAction {
                Button {
                    onclick: move |_| {
                        dialog.close();
                        form_state_clone.reset();
                    },
                    variant: ButtonVariant::Secondary,
                    r#type: "button",
                    "Cancel"
                }
                SubmitButton { form: form_state.clone() }
            }
        }
    }
}

#[component]
fn MemberEntry(
    member: entity::user::UserWithTodoListPermission,
    onmemberkick: EventHandler<()>,
) -> Element {
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
            ListDetails { title: "{member.first_name} {member.last_name}" }
            if let Some(user) = auth.user.peek().as_ref() && user.id == member.id {
                Dialog {
                    Tooltip { tooltip: "Leave To-Do List",
                        DialogTrigger { variant: ButtonVariant::Error,
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
                                                    ToastOptions::new().description(rsx! { "{error}" }),
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
                            DialogTrigger { variant: ButtonVariant::Error,
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
                                                            ToastOptions::new().description(rsx! { "User has been kicked from the To-Do List." }),
                                                        );
                                                    onmemberkick.call(());
                                                }
                                                Some(Err(error)) => {
                                                    toaster
                                                        .error(
                                                            "Failed to kick user",
                                                            ToastOptions::new().description(rsx! { "{error}" }),
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
