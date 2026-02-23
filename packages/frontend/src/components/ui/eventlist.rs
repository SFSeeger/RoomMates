use crate::Route;
use crate::components::tooltip::Tooltip;
use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::dialog::{Dialog, DialogContent, DialogTrigger, use_dialog};
use crate::components::ui::list::{List, ListDetails, ListRow};
use crate::components::ui::toaster::{ToastOptions, use_toaster};
use api::routes::events::{delete_event, event_has_groups, list_events};
use api::routes::groups::{remove_event_from_group, retrieve_group};
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{
    LdBadgeInfo, LdEye, LdEyeOff, LdFlag, LdNavigation, LdPencil, LdRefreshCcw, LdTrash, LdUsers,
};
use time::macros::format_description;

#[component]
pub fn EventList() -> Element {
    let mut events = use_loader(move || async move { list_events().await })?;
    let mut event_has_groups = use_action(event_has_groups);
    let mut delete_event = use_action(delete_event);
    let mut remove_event_from_group = use_action(remove_event_from_group);
    let mut toaster = use_toaster();

    let ondelete = move |event_id: i32| async move {
        {
            event_has_groups.call(event_id).await;
            if let Some(Ok(groups)) = event_has_groups.value() {
                if groups.read().is_empty() {
                    delete_event.call(event_id).await;
                    match delete_event.value() {
                        Some(Ok(_)) => {
                            toaster.success("Deleted event successfully!", ToastOptions::new());
                            events.write().retain(|event| event.id != event_id);
                        }
                        Some(Err(_)) => {
                            toaster.error("Failed to delete event!", ToastOptions::new());
                        }
                        None => {
                            warn!("Request did not finish!");
                        }
                    }
                } else {
                    for group in groups.read().iter() {
                        remove_event_from_group.call(group.id, event_id).await;
                        match remove_event_from_group.value() {
                            Some(Ok(_)) => {
                                toaster.success("Deleted event successfully!", ToastOptions::new());
                                events.write().retain(|event| event.id != event_id);
                            }
                            Some(Err(_)) => {
                                toaster.error("Failed to delete event!", ToastOptions::new());
                            }
                            None => {
                                warn!("Request did not finish!");
                            }
                        }
                    }
                }
            }
        }
    };

    rsx! {
        div { class: "w-full",
            List { header: "Your Events",
                for event in events.iter() {
                    EventListEntry { event: event.clone(), ondelete }
                }
            }
        }
    }
}

#[component]
pub fn EventListGroups(group_id: i32) -> Element {
    let mut group = use_loader(move || async move { retrieve_group(group_id).await })?;
    let mut remove_event_from_group = use_action(remove_event_from_group);
    let mut toaster = use_toaster();

    let ondelete = move |event_id: i32| async move {
        {
            remove_event_from_group.call(group_id, event_id).await;
            match remove_event_from_group.value() {
                Some(Ok(_)) => {
                    toaster.success("Deleted event successfully!", ToastOptions::new());
                    group.write().events.retain(|event| event.id != event_id);
                }
                Some(Err(_)) => {
                    toaster.error("Failed to delete event!", ToastOptions::new());
                }
                None => {
                    warn!("Request did not finish!");
                }
            }
        }
    };

    rsx! {
        div { class: "w-full",
            List { header: "Your Events",
                for event in group.read().events.iter() {
                    EventListEntry { event: event.clone(), ondelete }
                }
            }
        }
    }
}

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

    rsx! {
        div { class: "w-full",
            ListRow {
                ListDetails { title: title.clone(),
                    div { class: "flex w-full items-center gap-4 flex-wrap md:flex-nowrap",
                        div { class: "whitespace-nowrap",
                            if event.reoccurring {
                                h1 { "{event.weekday:?}" }
                            } else {
                                h1 { "{event.date}" }
                            }
                            p { "{start} - {end}" }
                        }
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
                            } else {
                                Tooltip { tooltip: "One time event",
                                    Icon { icon: LdFlag }
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
                            Icon { icon: LdBadgeInfo }
                            match &event.description {
                                Some(Text) => rsx! { "{Text}" },
                                None => rsx! { "no description" },
                            }
                        }
                        div { class: "flex items-center gap-2 whitespace-nowrap",
                            Icon { icon: LdNavigation }
                            match &event.location {
                                Some(Text) => rsx! { "{Text}" },
                                None => rsx! { "no location" },
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
                                    AddEventToGroup {}
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

#[component]
pub fn AddEventToGroup() -> Element {
    let _toaster = use_toaster();
    let _dialog = use_dialog();

    rsx! {
        div {}
    }
}
