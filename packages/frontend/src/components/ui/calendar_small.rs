use crate::Route;
use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::loader::Loader;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::LdMapPin;
use roommates::is_full_event_on_day;
use time::macros::format_description;
use time::{Date, Duration, OffsetDateTime};

#[component]
pub fn CalendarDashview(
    selected_date: ReadSignal<Date>,
    on_date_change: EventHandler<Date>,
    events: ReadSignal<Vec<entity::event::FullEvent>>,
    #[props(default)] is_loading: ReadSignal<bool>,
) -> Element {
    let today = OffsetDateTime::now_utc().date();
    let days = use_memo(move || {
        (-3..=3)
            .map(|offset| {
                let date = *selected_date.read() + Duration::days(offset);
                (
                    date,
                    date.format(format_description!("[weekday repr:short]"))
                        .unwrap()
                        .to_uppercase(),
                    offset == 0,
                    offset.abs(),
                )
            })
            .collect::<Vec<_>>()
    });
    let selected_days_events = use_memo(move || {
        events
            .read()
            .iter()
            .filter(|&event| is_full_event_on_day(event, selected_date()))
            .cloned()
            .collect::<Vec<_>>()
    });

    rsx! {
        div {
            div { class: "flex gap-1 md:gap-4 items-center justify-center mb-4",
                for (date , day_label , is_selected , abs_index) in days() {
                    div {
                        key: "{date}",
                        class: "flex-1",
                        class: if abs_index > 1 { "hidden md:block" },
                        class: if abs_index > 2 { "hidden lg:block" },
                        Button {
                            key: "{date}",
                            onclick: move |_| on_date_change.call(date),
                            outline: !is_selected,
                            class: "flex-col gap-0 w-full",
                            variant: if is_selected { ButtonVariant::Primary } else if date == today { ButtonVariant::Accent } else { ButtonVariant::Primary },
                            strong { class: "font-bold", "{date.day()}" }
                            br {}
                            "{day_label}"
                        }
                    }
                }
            }
            div { class: "divider divider-neutral" }
            div { class: "flex flex-col rounded-lg gap-2",
                CalenderDaily { events: selected_days_events, is_loading }
            }
        }
    }
}

#[component]
pub fn CalenderDaily(
    events: ReadSignal<Vec<entity::event::FullEvent>>,
    #[props(default)] is_loading: ReadSignal<bool>,
) -> Element {
    rsx! {
        div { class: "flex flex-col relative bg-base-100 rounded-lg  w-full h-[600px] overflow-y-scroll border shadow-sm border-base-100",
            div { class: "relative",
                div {
                    class: "grid grid-cols-[4rem_1fr] grid-rows-[repeat(96,var(--grid-row-size))] w-full relative",
                    style: "--grid-row-size: 1.5rem;",
                    for hour in 0..24 {
                        div {
                            class: "text-xs opacity-50 font-mono self-start pt-0 -mt-1 text-right pr-3 col-start-1 row-start-(--row-start)",
                            style: "--row-start: {hour * 4 + 1};",
                            "{hour:02}:00"
                        }
                    }
                    div { class: "day-calendar-grid grid grid-rows-subgrid row-span-full auto-rows-fr col-start-2",
                        for event in events.read().iter() {
                            CalenderEvents { event: event.clone() }
                        }
                    }
                }
                if is_loading() {
                    div { class: "absolute inset-0 bg-black/50 flex justify-center z-10",
                        div { class: "sticky top-0 h-[600px] flex",
                            Loader { class: "text-base-200 dark:text-base-content" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CalenderEvents(event: entity::event::FullEvent) -> Element {
    let start_row =
        (event.start_time.hour() as i32 * 4) + (event.start_time.minute() as i32 / 15) + 1;
    let duration_mins = (event.end_time - event.start_time).whole_minutes();
    let duration_rows = ((duration_mins as f32 / 15.0).ceil() as i32).max(1);

    let color_classes = if event.is_shared_with_user {
        "bg-success text-success-content"
    } else if event.is_group_event {
        "bg-accent text-accent-content"
    } else if event.private {
        "bg-info text-info-content"
    } else {
        "bg-primary text-primary-content"
    };

    rsx! {
        div {
            key: "{event.id}",
            class: "group relative rounded-lg border border-accent/10 shadow-sm flex overflow-hidden m-px {color_classes} hover:opacity-95",
            class: "row-start-(--grid-row) row-span-(--grid-span)",
            style: "--grid-row: {start_row}; --grid-span: {duration_rows};",
            Link {
                class: "block flex-1 p-2 flex-col justify-start min-w-0",
                to: Route::EditEventView {
                    event_id: event.id,
                },
                div { class: "font-bold text-lg truncate", "{event.title}" }
                if let Some(location) = event.location && duration_rows >= 2 {
                    span { class: "flex items-center text-sm gap-1",
                        Icon { class: "size-4", icon: LdMapPin }
                        "{location}"
                    }
                }
                if let Some(description) = &event.description && duration_rows > 2 {
                    p {
                        class: "text-sm opacity-80 max-h-full overflow-hidden text-ellipsis line-clamp-(--max-content-lines)",
                        style: "--max-content-lines: {(duration_rows - 2)};",
                        "{description}"
                    }
                }
            }
        }

    }
}
