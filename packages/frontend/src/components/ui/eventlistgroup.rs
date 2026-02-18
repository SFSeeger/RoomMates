use crate::Route;
use crate::components::tooltip::Tooltip;
use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::list::{List, ListDetails, ListRow};
use crate::components::ui::toaster::{ToastOptions, use_toaster};
use api::routes::groups::{remove_event_from_group, retrieve_group};
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{
    LdBadgeInfo, LdEye, LdEyeOff, LdFlag, LdNavigation, LdPencil, LdRefreshCcw, LdTrash,
};

#[component]
pub fn EventList(group_id: i32) -> Element {
    let mut group = use_loader(move || async move { retrieve_group(group_id).await })?;

    let oneventremove = move |event_id: i32| {
        group.write().events.retain(|event| event.id != event_id);
    };

    rsx! {
        List { header: "Your Events" }
        for event in group.read().events.iter() {
            EventListEntry { event: event.clone(), group_id, ondelete: oneventremove }
        }
    }
}

#[component]
pub fn EventListEntry(
    event: entity::event::Model,
    group_id: i32,
    ondelete: EventHandler<i32>,
) -> Element {
    let mut delete_action = use_action(remove_event_from_group);
    let mut toaster = use_toaster();
    let title = event.title.clone();

    rsx! {
        ListRow {
            div {
                p { "{event.start_time}" }
                p { class: "self-center", "-" }
                p { "{event.end_time}" }
            }
            ListDetails { title: title.clone(),
                div { class: "flex flex-row gap-4 justify-content full",
                    div {
                        if event.reoccurring {
                            h1 { class: "w-20", "{event.weekday:?}" }
                        } else {
                            h1 { class: "w-20", "{event.date}" }
                        }
                    }
                    div {
                        p {
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
                    }
                    div {
                        p {
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
                    }
                    div {
                        Icon { icon: LdBadgeInfo }
                        p { class: "w-100 ",
                            match &event.description {
                                Some(Text) => rsx! { "{Text}" },
                                None => rsx! { "no description" },
                            }
                        }
                    }
                    div {
                        Icon { icon: LdNavigation }
                        p { class: "w-50",
                            match &event.location {
                                Some(Text) => rsx! { "{Text}" },
                                None => rsx! { "no location" },
                            }
                        }
                    }
                    div {
                        Link {
                            to: Route::EditEventView {
                                event_id: event.id,
                            },
                            class: "btn btn-info",
                            Icon { icon: LdPencil }
                            "Edit"
                        }
                        Button {
                            onclick: move |_| {
                                let title_clone = title.clone();
                                async move {
                                    delete_action.call(group_id, event.id).await;
                                    match delete_action.value() {
                                        Some(Ok(_)) => {
                                            toaster
                                                .success(
                                                    &format!("Deleted {title_clone} successfully!"),
                                                    ToastOptions::new(),
                                                );
                                            ondelete.call(event.id);
                                        }
                                        Some(Err(_)) => {
                                            toaster.error("Failed to delete event!", ToastOptions::new());
                                        }
                                        None => {
                                            warn!("Request did not finish!");
                                        }
                                    }
                                }
                            },
                            variant: ButtonVariant::Error,
                            Icon { icon: LdTrash }
                        }
                    }
                }
            }
        }
    }
}
