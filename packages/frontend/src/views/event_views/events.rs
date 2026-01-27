use crate::Route;
use crate::components::tooltip::Tooltip;
use crate::components::ui::list::{List, ListDetails, ListRow};
use crate::components::ui::toaster::{Toast, ToastVariant, ToasterState};
use crate::components::ui::{
    button::{Button, ButtonShape, ButtonVariant},
    card::{Card, CardBody},
};
use api::routes::events::{delete_event, list_events};
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{
    LdBadgeInfo, LdEye, LdEyeOff, LdFlag, LdNavigation, LdPencil, LdPlus, LdRefreshCcw, LdTrash,
};

#[component]
pub fn ListEventView() -> Element {
    rsx! {
        div {
            Link { to: Route::AddEventView {},
                Button { variant: ButtonVariant::Primary, shape: ButtonShape::Wide,
                    Icon { icon: LdPlus }
                    "create new event"
                }
            }
        }
        div { class: "divider" }
        div { class: "flex w-full",
            Card {
                CardBody { EventList {} }
            }
        }
    }
}

#[component]
pub fn EventList() -> Element {
    let mut events = use_loader(move || async move { list_events().await })?;

    let ondelete = move |id: i32| {
        let mut lists_write = events.write();
        lists_write.retain(|list| list.id != id);
    };

    rsx! {
        List { header: "Your Events",
            for event in events.iter() {
                EventListEntry { event: event.clone(), ondelete }
            }
        }
    }
}

#[component]
pub fn EventListEntry(event: entity::event::Model, ondelete: EventHandler<i32>) -> Element {
    let mut delete_action: Action<(i32,), dioxus_fullstack::NoContent> = use_action(delete_event);
    let toaster = use_context::<ToasterState>();
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
                                let mut toaster_clone = toaster.clone();
                                let title_clone = title.clone();
                                async move {
                                    delete_action.call(event.id).await;
                                    match delete_action.value() {
                                        Some(Ok(_)) => {
                                            toaster_clone
                                                .toast(
                                                    Toast::new(
                                                        format!("Deleted {} successfully!", title_clone),
                                                        None,
                                                        true,
                                                        ToastVariant::Success,
                                                    ),
                                                );
                                            ondelete.call(event.id);

                                        }
                                        Some(Err(_)) => {
                                            toaster_clone
                                                .toast(
                                                    Toast::new(
                                                        "Failed to delete event!".to_owned(),
                                                        None,
                                                        true,
                                                        ToastVariant::Success,
                                                    ),
                                                );
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
