use crate::Route;
use crate::components::ui::button::{Button, ButtonShape};
use crate::components::ui::calendar::{
    Calendar, CalendarGrid, CalendarHeader, CalendarNavigation, CalendarNextMonthButton,
    CalendarPreviousMonthButton, CalendarSelectMonth, CalendarSelectYear, CalendarView,
    CustomCalendarDay,
};
use crate::components::ui::calendar_small::CalenderDaily;
use crate::components::ui::card::{Card, CardBody, CardTitle};
use api::routes::events::list_events;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{LdCalendarDays, LdExternalLink};
use roommates::{days_since, is_full_event_on_day};
use time::ext::NumericalDuration;
use time::macros::format_description;
use time::{Date, UtcDateTime};

#[component]
pub fn EventCalendarView() -> Element {
    let mut selected_date = use_signal(|| Some(UtcDateTime::now().date()));
    let mut view_date = use_signal(|| UtcDateTime::now().date());

    let date_range = use_memo(move || {
        let previous_month = view_date
            .read()
            .replace_day(1)
            .expect("invalid or out-of-range date");
        let num_days = days_since(view_date(), time::Weekday::Monday);
        let start_date = previous_month.saturating_sub(num_days.days());

        let num_days_in_month = view_date.read().month().length(view_date.read().year());
        let total_cells = num_days + i64::from(num_days_in_month);
        let remainder = total_cells % 7;
        let end_date = view_date
            .read()
            .replace_day(num_days_in_month)
            .expect("invalid or out-of-range date")
            .saturating_add((7 - remainder).rem_euclid(7).days());
        (start_date, end_date)
    });

    let mut events = use_loader(move || async move {
        let date_range = date_range();
        list_events(Some(date_range.0), Some(date_range.1)).await
    })?;

    use_effect(move || {
        date_range.read();
        events.restart();
    });

    let selected_days_events = use_memo(move || {
        if let Some(selected_date) = selected_date() {
            events
                .read()
                .iter()
                .filter(|&event| is_full_event_on_day(event, selected_date))
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    });

    let render_day = move |date: Date| {
        let events_on_day: Vec<_> = events
            .iter()
            .filter(|event| is_full_event_on_day(event, date))
            .collect();

        rsx! {
            CustomCalendarDay {
                date,
                class: "calendar-grid-cell flex flex-col justify-start items-center gap-1 group overflow-hidden not-data-[today=true]:py-1 data-[today=true]:data-[selected=true]:py-1",
                div { class: "flex sm:hidden gap-1 items-center justify-center w-full",
                    for event in events_on_day.iter().take(3) {
                        span {
                            key: "status-{event.id}",
                            class: "status {get_event_colors_status(event)} status-sm group-data-[selected=true]:bg-primary-content group-data-[month=current]:group-not-data-[disabled=true]:group-hover:bg-primary-content",
                        }
                    }
                }
                div { class: "hidden sm:flex flex-col gap-0.5 md:gap-1 overflow-hidden text-ellipsis w-full px-0.5",
                    for event in events_on_day.iter().take(6) {
                        span {
                            key: "badge-{event.id}",
                            class: "badge {get_event_colors_badge(event)} badge-xs md:badge-sm lg:badge-md badge-outline w-full text-nowrap overflow-hidden text-ellipsis",
                            class: "group-data-[selected=true]:text-primary-content group-data-[month=current]:group-not-data-[disabled=true]:group-hover:text-primary-content",
                            class: "nth-[n+4]:hidden md:nth-[n+4]:block md:nth-[n+6]:hidden xl:nth-[n+6]:block xl:nth-[n+7]:hidden",
                            "{event.title}"
                        }
                    }
                }
            }
        }
    };

    let date_text = selected_date().map_or("Select a date".to_string(), |date| {
        date.format(format_description!("[day].[month].[year]"))
            .unwrap()
    });

    rsx! {
        div { class: "flex flex-col md:flex-row flex-wrap gap-4",
            div { class: "flex-3",
                Calendar {
                    selected_date: selected_date(),
                    on_date_change: move |date| {
                        selected_date.set(date);
                    },
                    view_date: view_date(),
                    on_view_change: move |new_view: Date| {
                        view_date.set(new_view);
                    },
                    first_day_of_week: time::Weekday::Monday,
                    CalendarView {
                        CalendarHeader {
                            CalendarNavigation {
                                CalendarPreviousMonthButton {}
                                CalendarSelectMonth {}
                                CalendarSelectYear {}
                                Button {
                                    shape: ButtonShape::Square,
                                    ghost: true,
                                    class: "btn-sm",
                                    onclick: move |_| {
                                        let now = UtcDateTime::now().date();
                                        view_date.set(now);
                                        selected_date.set(Some(now));
                                    },
                                    Icon { icon: LdCalendarDays }
                                }
                                CalendarNextMonthButton {}
                            }
                        }
                        CalendarGrid { render_day }
                    }
                }
            }
            Card { class: "w-full grow flex-1 md:min-w-64 lg:min-w-96",
                CardBody {
                    CardTitle { class: "flex justify-between items-center",
                        "{date_text}"
                        if selected_date().is_none() {
                            Button { outline: true, disabled: true,
                                Icon { icon: LdExternalLink }
                            }
                        } else {
                            Link {
                                class: "btn btn-primary btn-outline",
                                to: Route::ListEventView {
                                    date: selected_date().into(),
                                },
                                Icon { icon: LdExternalLink }
                            }
                        }
                    }
                    CalenderDaily { events: selected_days_events }
                }
            }
        }
    }
}

fn get_event_colors_badge(event: &entity::event::FullEvent) -> &'static str {
    if event.is_shared_with_user {
        "badge-success"
    } else if event.is_group_event {
        "badge-accent"
    } else if event.private {
        "badge-info"
    } else {
        "badge-primary"
    }
}

fn get_event_colors_status(event: &entity::event::FullEvent) -> &'static str {
    if event.is_shared_with_user {
        "status-success"
    } else if event.is_group_event {
        "status-accent"
    } else if event.private {
        "status-info"
    } else {
        "status-primary"
    }
}
