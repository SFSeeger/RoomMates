use crate::components::ui::button::{Button, ButtonShape, ButtonVariant};
use crate::components::ui::card::{Card, CardActions, CardBody, CardTitle};
use crate::components::ui::dialog::{Dialog, DialogAction, DialogContent, DialogTrigger};
use crate::components::ui::form::input::Input;
use crate::components::ui::form::submit_button::SubmitButton;
use crate::components::ui::list::{List, ListDetails, ListRow};
use crate::components::ui::toaster::{Toast, ToastVariant, ToasterState};
use api::routes::groups::retrieve_group;
use api::routes::groups::{add_user_to_group, change_group_name, remove_user_from_group};
use api::routes::users::{EMAIL_REGEX, UserInfo};
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{LdMail, LdMinus, LdPlus, LdUsers};
use form_hooks::use_form::{use_form, use_on_submit};
use form_hooks::use_form_field::use_form_field;
use form_hooks::validators;
use regex::Regex;

#[derive(serde::Deserialize)]
struct AddUserFormData {
    email: String,
}

#[derive(serde::Deserialize)]
struct GroupNameNew {
    group_name: String,
}

#[component]
pub fn EditGroup(group_id: i32) -> Element {
    let mut group = use_loader(move || async move { retrieve_group(group_id).await })?;

    let toaster = use_context::<ToasterState>();

    let mut change_group_name = use_action(change_group_name);
    let mut form_state_group_name = use_form();

    let group_name_field = use_form_field("group_name", group.read().name.clone())
        .with_validator(validators::required("Group name is required!"));

    form_state_group_name.register_field(&group_name_field);

    form_state_group_name.revalidate();

    let onsubmitgroupname = use_on_submit(&form_state_group_name, move |form| async move {
        let group_name_new: GroupNameNew = form.parsed_values().unwrap();
        change_group_name
            .call(group_id, group_name_new.group_name.clone())
            .await;

        match change_group_name.value() {
            Some(Ok(_)) => {
                debug!("Group name updated");
                group.restart();
            }
            Some(Err(error)) => {
                debug!("Failed to change group name with error {:?}", error)
            }
            None => {
                debug! {"No value present!"}
            }
        }
    });

    let mut form_state_add = use_form();
    let email = use_form_field("email", String::new())
        .with_validator(validators::required("Email is required"))
        .with_validator(validators::pattern(
            Regex::new(EMAIL_REGEX)?,
            "Email must be a valid email",
        ));
    form_state_add.register_field(&email);

    let mut add_user_to_group = use_action(add_user_to_group);
    let onsubmitadd = use_on_submit(&form_state_add, move |form| {
        let value = toaster.clone();
        async move {
            let add_user_form_data: AddUserFormData = form.parsed_values().unwrap();
            let mut toaster_clone = value.clone();
            add_user_to_group
                .call(group_id, add_user_form_data.email.clone())
                .await;
            match add_user_to_group.value() {
                Some(Ok(_)) => {
                    toaster_clone.toast(Toast::new(
                        format!("Added user {} to group", add_user_form_data.email),
                        None,
                        true,
                        ToastVariant::Success,
                    ));
                    group.restart();
                }
                Some(Err(error)) => {
                    toaster_clone.toast(Toast::new(
                        "Failed to add user".into(),
                        Some(rsx! {
                            span { "{error}" }
                        }),
                        true,
                        ToastVariant::Error,
                    ));
                }
                None => {
                    debug! {"Add user did not finish"}
                }
            }
        }
    });

    let onmemberremove = move |member_id: i32| {
        group
            .write()
            .members
            .retain(|member| member.id != member_id);
    };

    rsx! {
        div {
            h1 { class: "relative text-2xl font-bold text-center", "Edit your groups" }
            div { class: "flex w-full flex-col items-start md:flex-row",
                div { class: "Group-Events flex-1 items-center grow justify-center",
                    Card { class: "w-full",
                        CardBody {
                            CardTitle { class: "flex items-center justify-center",
                                div { class: "flex justify-center w-full",
                                    div { class: "w-3/4",
                                        form { onsubmit: onsubmitgroupname,
                                            Input {
                                                field: group_name_field,
                                                label: "Group name",
                                                r#type: "text",
                                                class: "h-12 text-lg px-4",
                                                icon: {
                                                    rsx! {
                                                        Icon { icon: LdUsers }
                                                    }
                                                },
                                            }
                                            CardActions {
                                                SubmitButton {
                                                    form: form_state_group_name.clone(),
                                                    class: "w-full",
                                                    label: "Change group name",
                                                    submitting_label: "Changing group name...",
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            p { "Events" }
                        }
                    }
                }
                div { class: "Member-Sidecard relative flex flex-col w-66 overflow-y-auto",
                    List { header: "Members",
                        div { class: "absolute top-3 right-4 z-10",
                            Dialog {
                                DialogTrigger {
                                    variant: ButtonVariant::Primary,
                                    shape: ButtonShape::Round,
                                    ghost: false,
                                    class: "btn-sm",
                                    Icon { icon: LdPlus }
                                }
                                DialogContent { title: "Enter the email of the person you want to add to {group.read().name}",
                                    form { onsubmit: onsubmitadd,
                                        Input {
                                            field: email,
                                            label: "User email",
                                            r#type: "email",
                                            icon: rsx! {
                                                Icon { icon: LdMail }
                                            },
                                        }
                                        DialogAction {
                                            form { method: "dialog",
                                                Button {
                                                    r#type: "button",
                                                    variant: ButtonVariant::Secondary,
                                                    "Cancel"
                                                }
                                            }
                                            Button {
                                                r#type: "submit",
                                                variant: ButtonVariant::Primary,
                                                "Add"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if group.read().members.is_empty() {
                            ListRow {
                                ListDetails { title: "No members yet" }
                            }
                        }
                        ListRow {
                            ListDetails { title: "",
                                for member in group.read().members.iter() {
                                    GroupListEntry {
                                        key: "{member.id}",
                                        member: member.clone(),
                                        group_id,
                                        onmemberremove,
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

#[component]
pub fn GroupListEntry(
    member: UserInfo,
    group_id: i32,
    onmemberremove: EventHandler<i32>,
) -> Element {
    let toaster = use_context::<ToasterState>();
    let name = format! {"{} {}", member.first_name, member.last_name};
    let mut remove_user_from_group = use_action(remove_user_from_group);
    rsx! {
        div { class: "flex justify-between items-center mb-2 w-full",
            span { class: "flex-1", "{name}" }
            Dialog {
                DialogTrigger {
                    variant: ButtonVariant::Primary,
                    shape: ButtonShape::Round,
                    ghost: false,
                    class: "btn-sm",
                    Icon { icon: LdMinus }
                }
                DialogContent { title: "Do you want to remove {name.clone()} from this group?",
                    form { method: "dialog",
                        DialogAction {
                            Button { variant: ButtonVariant::Secondary, "Cancel" }
                            Button {
                                onclick: move |_| {
                                    let member_id = member.id;
                                    let group_id = group_id;
                                    let mut toaster_clone = toaster.clone();
                                    let name_clone = name.clone();
                                    async move {
                                        remove_user_from_group.call(group_id, member_id).await;
                                        match remove_user_from_group.value() {
                                            Some(Ok(_)) => {
                                                toaster_clone
                                                    .toast(
                                                        Toast::new(
                                                            format!("Deleted {} successfully!", name_clone),
                                                            None,
                                                            true,
                                                            ToastVariant::Success,
                                                        ),
                                                    );
                                                onmemberremove.call(member_id);
                                            }
                                            Some(Err(error)) => {
                                                toaster_clone
                                                    .toast(
                                                        Toast::new(
                                                            format!("Failed to delete {}!", name_clone),
                                                            Some(rsx! {
                                                                span { "{error.to_string()}" }
                                                            }),
                                                            true,
                                                            ToastVariant::Error,
                                                        ),
                                                    );
                                            }
                                            None => warn!("Delete member did not finish!"),
                                        }
                                    }
                                },
                                variant: ButtonVariant::Error {},
                                "Delete"
                            }
                        }
                    }
                }
            }
        }
    }
}
