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
pub fn AddEventToGroup() -> Element {
    let _toaster = use_toaster();
    let _dialog = use_dialog();

    rsx! {
        div {}
    }
}
