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
pub fn EventListEntry(event: entity::event::Model, ondelete: EventHandler<i32>) -> Element {
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
                                Some(text) => rsx! { "{text}" },
                                None => rsx! { "no description" },
                            }
                        }
                        div { class: "flex items-center gap-2 whitespace-nowrap",
                            Icon { icon: LdNavigation }
                            match &event.location {
                                Some(text) => rsx! { "{text}" },
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
