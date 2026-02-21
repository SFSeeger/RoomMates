use crate::Route;
use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::{LdEye, LdEyeOff, LdTrash};
use dioxus_free_icons::{
    Icon,
    icons::ld_icons::{LdMapPin, LdRefreshCcw},
};
use time::macros::format_description;

use crate::components::ui::button::{Button, ButtonShape, ButtonVariant};
use crate::components::ui::dialog::{Dialog, DialogAction, DialogContent, DialogTrigger};
use crate::components::{
    tooltip::Tooltip,
    ui::list::{ComplexListDetails, ListRow},
};

#[component]
pub fn EventListEntry(
    event: ReadSignal<entity::event::Model>,
    #[props(default)] navigable: bool,
    ondelete: Option<EventHandler<i32>>,
) -> Element {
    let date_format = format_description!("[day].[month].[year]");
    let date_str = event().date.format(date_format).unwrap_or_default();

    let time_format = format_description!("[hour]:[minute]");
    let start_time_str = event().start_time.format(time_format).unwrap_or_default();
    let end_time_str = event().end_time.format(time_format).unwrap_or_default();

    let href = navigable.then(|| Route::EditEventView {
        event_id: event().id,
    });

    rsx! {
        ListRow {
            div { class: "font-thin opacity-90 tabular-nums text-center",
                div { "{start_time_str}" }
                div { "-" }
                div { "{end_time_str}" }
            }
            ComplexListDetails {
                link: href,
                title: rsx! {
                    h3 { class: "flex items-center gap-1",
                        if event().reoccurring {
                            "{event().weekday} "
                        } else {
                            "{date_str} "
                        }
                        span { class: "font-bold", "{event().title}" }
                        if event().reoccurring {
                            Tooltip { tooltip: "Reoccurring event",
                                Icon { class: "size-4", icon: LdRefreshCcw }
                            }
                        }
                        if event().private {
                            Tooltip { tooltip: "Private event",
                                Icon { class: "size-4", icon: LdEyeOff }
                            }
                        } else {
                            Tooltip { tooltip: "Public event",
                                Icon { class: "size-4", icon: LdEye }
                            }
                        }
                    }
                },
                div { class: "text-xs font-semibold",
                    div { class: "flex items-center font-bold my-2",
                        if let Some(location) = event().location {
                            Icon { class: "size-4", icon: LdMapPin }
                            "{location}"
                        }
                    }
                    div { class: "opacity-70",
                        if let Some(description) = event().description {
                            "{description}"
                        }
                    }
                }
            }
            if ondelete.is_some() {
                Dialog {
                    DialogTrigger {
                        variant: ButtonVariant::Error,
                        shape: ButtonShape::Square,
                        ghost: true,
                        Icon { icon: LdTrash }
                    }
                    DialogContent { title: "Do you really want to delete {event().title}?",
                        DialogAction {
                            form { method: "dialog",
                                Button { variant: ButtonVariant::Secondary, "Cancel" }
                                Button {
                                    variant: ButtonVariant::Error,
                                    onclick: move |_| {
                                        if let Some(ondelete) = ondelete {
                                            ondelete(event().id);
                                        }
                                    },
                                    "Delete"
                                }
                            }
                        }
                    }
                }
            } else {
                div {}
            }
        }
    }
}
