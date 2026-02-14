use crate::Route;
use crate::components::ui::button::{Button, ButtonShape, ButtonVariant};
use crate::components::ui::dialog::{Dialog, DialogAction, DialogContent, DialogTrigger};
use crate::components::ui::form::input::Input;
use crate::components::ui::groupcard::GroupCard;
use crate::components::ui::toaster::{ToastOptions, use_toaster};
use api::routes::groups::create_group;
use api::routes::groups::list_groups;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{LdPlus, LdUsers};
use form_hooks::use_form::{use_form, use_on_submit};
use form_hooks::use_form_field::use_form_field;
use form_hooks::validators;

#[derive(serde::Deserialize)]
struct NewGroupName {
    group_name: String,
}

#[component]
pub fn GroupView() -> Element {
    let groups = use_server_future(move || async move { list_groups().await })?;

    rsx! {
        div {
            h1 { class: "text-2xl font-bold text-center relative", "Your groups" }
            Dialog {
                DialogTrigger {
                    variant: ButtonVariant::Primary,
                    shape: ButtonShape::Round,
                    ghost: false,
                    class: "fixed bottom-16 lg:bottom-4 right-4 btn btn-primary btn-circle lg:btn-lg",
                    Icon { icon: LdPlus }
                }
                DialogContent { title: "Enter the new group's name", NewGroupForm {} }
            }
            match &*groups.read() {
                Some(Ok(groups)) => rsx! {
                    div { class: "space-y-4 grid grid-cols-1 md:grid-cols-2 gap-6 overflow-y-auto",
                        for group in groups.iter() {
                            GroupCard { key: "{group.id}", group_id: group.id }
                        }
                    }
                },
                Some(Err(error)) => rsx! {
                    p { class: "text-red-500", "Loading groups failed: {error}" }
                },
                None => rsx! {
                    p { "Loading..." }
                },
            }
        }

    }
}

#[component]
pub fn NewGroupForm() -> Element {
    let mut toaster = use_toaster();
    let nav = navigator();

    let mut create_group = use_action(create_group);
    let mut new_group_name_form = use_form();

    let group_name_field = use_form_field("group_name", String::new())
        .with_validator(validators::required("Group name is required!"));

    new_group_name_form.register_field(&group_name_field);
    new_group_name_form.revalidate();

    let onsubmit = use_on_submit(&new_group_name_form, move |form| async move {
        let new_group_name: NewGroupName = form.parsed_values().unwrap();

        create_group.call(new_group_name.group_name).await;

        match create_group.value() {
            Some(Ok(group)) => {
                let group_id = group.read().id;
                nav.push(Route::EditGroup { group_id });
                debug!("Group name updated");
            }
            Some(Err(error)) => {
                toaster.error(
                    "Failed to change group name",
                    ToastOptions::new().description(rsx! {
                        span { "{error.to_string()}" }
                    }),
                );
            }
            None => {
                warn! {"No value present!"}
            }
        }
    });
    rsx! {
        form { onsubmit,
            Input {
                field: group_name_field,
                label: "New group name",
                r#type: "text",
                class: "h-12 text-lg px4",
                icon: {
                    rsx! {
                        Icon { icon: LdUsers }
                    }
                },
            }
            DialogAction {
                form { method: "dialog",
                    Button { r#type: "button", variant: ButtonVariant::Secondary, "Cancel" }
                }
                Button { r#type: "submit", variant: ButtonVariant::Primary, "Add" }
            }
        }
    }
}
