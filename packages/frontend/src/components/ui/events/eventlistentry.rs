use crate::Route;
use crate::components::tooltip::Tooltip;
use crate::components::ui::{
    button::{Button, ButtonVariant},
    dialog::{Dialog, DialogAction, DialogContent, DialogTrigger, use_dialog},
    form::vectorselect::VectorSelect,
    list::{ListDetails, ListRow},
    toaster::{ToastOptions, use_toaster},
};
use api::routes::events::{add_event_to_group, event_has_groups};
use api::routes::groups::list_groups;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{
    LdEyeOff, LdNavigation, LdPencil, LdRefreshCcw, LdTrash, LdUsers,
};
use form_hooks::use_form::{use_form, use_on_submit};
use form_hooks::use_form_field::use_form_field;
use time::macros::format_description;

#[component]
pub fn EventListEntry(event: entity::event::Model, ondelete: EventHandler<i32>) -> Element {
    let event_has_groups = use_loader(move || async move { event_has_groups(event.id).await })?;
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
                ListDetails { title: title.clone(),
                    div { class: "flex w-full items-center gap-4 flex-wrap md:flex-nowrap",
                        div { class: "whitespace-nowrap",
                            if event.reoccurring {
                                h1 { "{event.weekday:?}" }
                            } else {
                                h1 { "{date}" }
                            }
                            p { "{start} - {end}" }
                        }
                        div {
                            if event.private {
                                Tooltip { tooltip: "Event is private",
                                    Icon { icon: LdEyeOff }
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
                        div { class: "flex gap-2 flex-1 items-center",
                            if let Some(text) = &event.description {
                                "{text}"
                            }
                        }
                        div { class: "flex items-center gap-1 whitespace-nowrap",
                            if let Some(text) = &event.location {
                                div {
                                    Icon { icon: LdNavigation }
                                }
                                br {}
                                div { "{text}" }
                            }
                        }
                        div { class: "flex gap-2 ml-auto",
                            Link {
                                to: Route::EditEventView {
                                    event_id: event.id,
                                },
                                class: "btn btn-info",
                                Icon { icon: LdPencil }
                                "Edit"
                            }
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
    }
}

#[derive(serde::Deserialize)]
struct FormData {
    group_id: i32,
}

#[component]
pub fn AddEventToGroup(event_id: i32) -> Element {
    let groups = use_loader(move || async move { list_groups().await })?;
    let options = groups()
        .iter()
        .map(|g| (Some(g.id), g.name.clone()))
        .collect::<Vec<_>>();

    let mut toaster = use_toaster();
    let dialog = use_dialog();
    let mut form_state_group = use_form();
    let group_field = use_form_field("group_id", None::<i32>);
    form_state_group.register_field(&group_field);
    form_state_group.revalidate();
    let mut add_event = use_action(add_event_to_group);

    let onsubmit = use_on_submit(&form_state_group, move |form| async move {
        let data: FormData = form.parsed_values().unwrap();
        add_event.call(event_id, data.group_id).await;
        match add_event.value() {
            Some(Ok(_)) => {
                toaster.success("Added event to group successfully!", ToastOptions::new());
                dialog.close();
            }
            Some(Err(error)) => {
                toaster.error(
                    "Failed to add event to group!",
                    ToastOptions::new().description(rsx! {
                        span { "{error.to_string()}" }
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
                    },
                    r#type: "button",
                    variant: ButtonVariant::Secondary,
                    "Cancel"
                }
                Button { r#type: "submit", variant: ButtonVariant::Primary, "Add" }
            }
        }
    }
}
