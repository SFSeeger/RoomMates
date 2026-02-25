use crate::Route;
use crate::components::tooltip::Tooltip;
use crate::components::ui::form::submit_button::SubmitButton;
use crate::components::ui::{
    button::{Button, ButtonVariant},
    dialog::{Dialog, DialogAction, DialogContent, DialogTrigger, use_dialog},
    form::vectorselect::VectorSelect,
    list::{ComplexListDetails, ListRow},
    toaster::{ToastOptions, use_toaster},
};
use api::routes::events::{add_event_to_group, list_event_groups};
use api::routes::groups::list_groups;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{
    LdEye, LdEyeOff, LdMapPin, LdRefreshCcw, LdTrash, LdUsers,
};
use form_hooks::use_form::{use_form, use_on_submit};
use form_hooks::use_form_field::use_form_field;
use form_hooks::validators;
use roommates::message_from_captured_error;
use time::macros::format_description;

#[component]
pub fn EventListEntry(event: entity::event::Model, ondelete: EventHandler<i32>) -> Element {
    let event_has_groups = use_loader(move || async move { list_event_groups(event.id).await })?;
    let title = event.title.clone();
    let start = event
        .start_time
        .format(format_description!("[hour]:[minute]"))
        .unwrap();
    let end = event
        .end_time
        .format(format_description!("[hour]:[minute]"))
        .unwrap();
    let date = event
        .date
        .format(format_description!("[day].[month].[year]"))
        .unwrap();

    rsx! {
        div { class: "w-full",
            ListRow {
                div {}
                ComplexListDetails {
                    link: Route::EditEventView {
                        event_id: event.id,
                    },
                    title: rsx! {
                        "{title}"
                        div { class: "whitespace-nowrap",
                            if event.reoccurring {
                                h1 { "{event.weekday:?}" }
                            } else {
                                h1 { "{date}" }
                            }
                            p { "{start} - {end}" }
                        }
                        div { class: "flex flex-wrap items-center gap-2",
                            div {
                                if event.private {
                                    Tooltip { tooltip: "Event is private",
                                        Icon { icon: LdEyeOff }
                                    }
                                } else {
                                    Tooltip { tooltip: "Event is public",
                                        Icon { icon: LdEye }
                                    }
                                }
                            }
                            div {
                                if event.reoccurring {
                                    Tooltip { tooltip: "Reoccurring event",
                                        Icon { icon: LdRefreshCcw }
                                    }
                                }
                            }
                            div {
                                if event_has_groups.len() > 0 {
                                    Tooltip { tooltip: "Event is in a group",
                                        Icon { icon: LdUsers }
                                    }
                                }
                            }
                            div { class: "flex items-center gap-1 whitespace-nowrap",
                                if let Some(text) = &event.location {
                                    div {
                                        Icon { icon: LdMapPin }
                                    }
                                    br {}
                                    div { "{text}" }
                                }
                            }
                        }
                    },
                    div { class: "flex w-full items-center gap-4 flex-wrap md:flex-nowrap",

                        div { class: "flex gap-2 flex-1 items-center",
                            if let Some(text) = &event.description {
                                span { "{text}" }
                            }
                        }
                    }
                }
                div { class: "flex gap-2 ml-auto",
                    Dialog {
                        DialogTrigger {
                            variant: ButtonVariant::Primary,
                            ghost: false,
                            class: "btn",
                            Icon { icon: LdUsers }
                            "Add to group"
                        }
                        DialogContent { title: "Choose a group you want to add this event to",
                            AddEventToGroup { event_id: event.id }
                        }
                    }
                    Button {
                        onclick: move |_| { ondelete.call(event.id) },
                        variant: ButtonVariant::Error,
                        Icon { icon: LdTrash }
                    }
                }
            }
        }
    }
}

#[derive(serde::Deserialize)]
struct FormData {
    group_id: i32,
}

#[component]
pub fn AddEventToGroup(event_id: i32) -> Element {
    let groups = use_loader(move || async move { list_groups().await })?;

    let mut options = Vec::with_capacity(groups.len() + 1);
    options.push((None, "Select a group".into()));
    options.append(
        &mut groups()
            .iter()
            .map(|g| (Some(g.id), g.name.clone()))
            .collect(),
    );

    let mut toaster = use_toaster();
    let dialog = use_dialog();
    let mut form_state_group = use_form();
    let group_field = use_form_field("group_id", None::<i32>)
        .with_validator(validators::required("Group is required!"));
    form_state_group.register_field(&group_field);
    form_state_group.revalidate();
    let mut add_event = use_action(add_event_to_group);
    let mut form_state_group_clone = form_state_group.clone();

    let onsubmit = use_on_submit(&form_state_group, move |mut form| async move {
        let data: FormData = form.parsed_values().unwrap();
        add_event.call(event_id, data.group_id).await;
        match add_event.value() {
            Some(Ok(_)) => {
                toaster.success("Added event to group successfully!", ToastOptions::new());
                dialog.close();
                form.reset();
            }
            Some(Err(error)) => {
                toaster.error(
                    "Failed to add event to group!",
                    ToastOptions::new().description(rsx! {
                        span { {message_from_captured_error(&error)} }
                    }),
                );
            }
            None => warn!("Adding event to group did not finish yet!"),
        }
    });

    rsx! {
        form { onsubmit,
            VectorSelect {
                label: Some("Group".into()),
                field: group_field.clone(),
                options,
            }
            DialogAction {
                Button {
                    onclick: move |_| {
                        dialog.close();
                        form_state_group_clone.reset();
                    },
                    r#type: "button",
                    variant: ButtonVariant::Secondary,
                    "Cancel"
                }
                SubmitButton {
                    form: form_state_group.clone(),
                    label: "Add",
                    submitting_label: "Adding...",
                }
            }
        }
    }
}
