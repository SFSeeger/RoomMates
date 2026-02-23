use crate::components::ui::events::eventlistentry::{EventListEntry, SharedEventRow};
use crate::components::ui::list::List;
use crate::components::ui::toaster::{ToastOptions, use_toaster};
use api::routes::events::invitations::list_shared_friend_events;
use api::routes::events::list_event_groups;
use api::routes::events::{
    delete_event, leave_event, list_events, remove_event_from_group, remove_shared_event_members,
};
use api::routes::groups::retrieve_group;
use dioxus::prelude::*;

#[component]
pub fn EventList() -> Element {
    let mut events = use_loader(move || async move { list_events().await })?;
    let mut event_has_groups = use_action(list_event_groups);
    let mut delete_event = use_action(delete_event);
    let mut remove_event_from_group = use_action(remove_event_from_group);
    let mut remove_event_shares = use_action(remove_shared_event_members);
    let mut toaster = use_toaster();

    let ondelete = move |event_id: i32| async move {
        event_has_groups.call(event_id).await;

        if let Some(Ok(groups)) = event_has_groups.value()
            && !groups.read().is_empty()
        {
            for group in groups.read().iter() {
                remove_event_from_group.call(event_id, group.id).await;
                match remove_event_from_group.value() {
                    Some(Ok(_)) => {}
                    Some(Err(_)) => {
                        toaster.error("Failed to remove event from groups!", ToastOptions::new());
                    }
                    None => {
                        warn!("Request did not finish!");
                    }
                }
            }
        }

        remove_event_shares.call(event_id).await;
        match remove_event_shares.value() {
            Some(Ok(_)) => {}
            Some(Err(_)) => {
                toaster.error(
                    "Failed to resolve shares between users!",
                    ToastOptions::new(),
                );
            }
            None => {
                warn!("Request did not finish!");
            }
        }

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
        remove_event_from_group.call(event_id, group_id).await;
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
            Some(Err(_)) => {
                toaster.error("Failed to leave event!", ToastOptions::new());
            }
            None => {
                warn!("Request did not finish!");
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
