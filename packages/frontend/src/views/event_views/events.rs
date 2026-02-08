use crate::Route;
use crate::components::ui::events::eventlist::EventList;
use crate::components::ui::{
    button::{Button, ButtonShape, ButtonVariant},
    card::{Card, CardBody},
};
use api::routes::invitations::list_shared_friend_events;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::LdPlus;

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
        div { class: "w-full",
            Card {
                CardBody { EventList {} }
            }
        }
    }
}

/* #[component]
pub fn EventListEntry(event: entity::event::Model, ondelete: EventHandler<i32>) -> Element {
    let mut delete_action: Action<(i32,), dioxus_fullstack::NoContent> = use_action(delete_event);
    let mut toaster = use_toaster();
    let title = event.title.clone();

    rsx! {
        ListRow {
            div {
                p { "{event.start_time}" }
                p { class: "self-center", "-" }
                p { "{event.end_time}" }
            }
            ComplexListDetails { title: rsx! { "{title}" },
                div { class: "flex justify-content full",
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
                            Button { variant: ButtonVariant::Accent,
                                Icon { icon: LdPencil }
                            }
                        }
                        Link {
                            to: Route::SendInvite {
                                invite_id: event.id,
                            },
                            Button { variant: ButtonVariant::Info,
                                Icon { icon: LdUserPlus }
                            }
                        }
                        Button {
                            onclick: move |_| {
                                let title_clone = title.clone();
                                async move {
                                    delete_action.call(event.id).await;
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

#[component]
pub fn SharedEventRow(event: entity::event::Model) -> Element {
    rsx! {
        ListRow {
            ListDetails { title: event.title,
                div { class: "flex-row gap-w justify-content full",
                    p { "{event.start_time} - {event.end_time}" }
                    div {
                        if event.reoccurring {
                            h1 { class: "w-20", "Every {event.weekday:?}" }
                        } else {
                            h1 { class: "w-20", "{event.date}" }
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
                    }
                }
            }
        }
    }
} */
