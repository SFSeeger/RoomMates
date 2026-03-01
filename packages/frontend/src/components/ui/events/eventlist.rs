use crate::components::ui::events::eventlistentry::{EventListEntry, SharedEventRow};
use crate::components::ui::list::List;
use crate::components::ui::toaster::{ToastOptions, use_toaster};
use api::routes::events::invitations::list_shared_friend_events;
use api::routes::events::{delete_event, leave_event, list_events, remove_event_from_group};
use api::routes::groups::retrieve_group;
use dioxus::prelude::*;
use roommates::message_from_captured_error;
use time::Date;

#[component]
pub fn EventList(date: Option<Date>) -> Element {
    let mut events = use_loader(move || async move { list_events(date, date).await })?;
    let mut delete_event = use_action(delete_event);
    let mut toaster = use_toaster();
    let ondelete = move |event_id: i32| async move {
        delete_event.call(event_id).await;
        match delete_event.value() {
            Some(Ok(_)) => {
                toaster.success("Deleted event successfully!", ToastOptions::new());
                events.write().retain(|event| event.id != event_id);
            }
            Some(Err(error)) => {
                toaster.error(
                    "Failed to delete event!",
                    ToastOptions::new().description(rsx! {
                        span { {message_from_captured_error(&error)} }
                    }),
                );
            }
            None => {
                warn!("Deleting event did not finish yet!");
            }
        }
    };

    rsx! {
        div { class: "w-full",
            List { header: "Your Events",
                for event in events.iter() {
                    EventListEntry { event: event.clone().into(), ondelete }
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
        remove_event_from_group.call(event_id, group_id).await;
        match remove_event_from_group.value() {
            Some(Ok(_)) => {
                toaster.success("Removed event successfully!", ToastOptions::new());
                group.write().events.retain(|event| event.id != event_id);
            }
            Some(Err(error)) => {
                toaster.error(
                    "Failed to remove event from group!",
                    ToastOptions::new().description(rsx! {
                        span { {message_from_captured_error(&error)} }
                    }),
                );
            }
            None => {
                warn!("Removing event from group did not finish yet!");
            }
        }
    };

    rsx! {
        div { class: "w-full",
            List { header: "Your Events",
                for event in group.read().events.iter() {
                    EventListEntry {
                        event: event.clone(),
                        ondelete,
                        group_id: Some(group_id),
                    }
                }
            }
        }
    }
}

#[component]
pub fn SharedEventList() -> Element {
    let mut shared_events = use_loader(move || async move { list_shared_friend_events().await })?;
    let mut leave_event = use_action(leave_event);

    let mut toaster = use_toaster();

    let onleave = move |event_id: i32| async move {
        leave_event.call(event_id).await;
        match leave_event.value() {
            Some(Ok(_)) => {
                toaster.success("Left Event!", ToastOptions::new());
                shared_events.write().retain(|event| event.id != event_id);
            }
            Some(Err(error)) => {
                toaster.error(
                    "Failed to leave event!",
                    ToastOptions::new().description(rsx! {
                        span { {message_from_captured_error(&error)} }
                    }),
                );
            }
            None => {
                warn!("Leaving the event did not finish yet!");
            }
        }
    };

    rsx! {
        div { class: "w-full",
            List { header: "Your friend's events ",
                for event in shared_events.read().iter() {
                    SharedEventRow { event: event.clone(), onleave }
                }
            }
        }
    }
}
