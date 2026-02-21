use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::ld_icons::{LdChevronLeft, LdChevronRight},
};
use dioxus_primitives::calendar::{
    self, CalendarDayProps, CalendarGridProps, CalendarHeaderProps, CalendarMonthTitleProps,
    CalendarNavigationProps, CalendarProps, CalendarSelectMonthProps, CalendarSelectYearProps,
    RangeCalendarProps, use_calendar_day_attributes,
};
use time::Date;

#[component]
pub fn Calendar(props: CalendarProps) -> Element {
    rsx! {
        calendar::Calendar {
            class: "calendar",
            selected_date: props.selected_date,
            on_date_change: props.on_date_change,
            on_format_weekday: props.on_format_weekday,
            on_format_month: props.on_format_month,
            view_date: props.view_date,
            today: props.today,
            on_view_change: props.on_view_change,
            disabled: props.disabled,
            first_day_of_week: props.first_day_of_week,
            min_date: props.min_date,
            max_date: props.max_date,
            month_count: props.month_count,
            disabled_ranges: props.disabled_ranges,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn RangeCalendar(props: RangeCalendarProps) -> Element {
    rsx! {
        calendar::RangeCalendar {
            class: "calendar",
            selected_range: props.selected_range,
            on_range_change: props.on_range_change,
            on_format_weekday: props.on_format_weekday,
            on_format_month: props.on_format_month,
            view_date: props.view_date,
            today: props.today,
            on_view_change: props.on_view_change,
            disabled: props.disabled,
            first_day_of_week: props.first_day_of_week,
            min_date: props.min_date,
            max_date: props.max_date,
            month_count: props.month_count,
            disabled_ranges: props.disabled_ranges,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn CalendarView(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div { class: "calendar-view", ..attributes, {children} }
    }
}

#[component]
pub fn CalendarHeader(props: CalendarHeaderProps) -> Element {
    rsx! {
        calendar::CalendarHeader { id: props.id, attributes: props.attributes, {props.children} }
    }
}

#[component]
pub fn CalendarNavigation(props: CalendarNavigationProps) -> Element {
    rsx! {
        calendar::CalendarNavigation { attributes: props.attributes, {props.children} }
    }
}

#[component]
pub fn CalendarPreviousMonthButton(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        calendar::CalendarPreviousMonthButton { attributes,
            Icon { icon: LdChevronLeft }
        }
    }
}

#[component]
pub fn CalendarNextMonthButton(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        calendar::CalendarNextMonthButton { attributes,
            Icon { icon: LdChevronRight }
        }
    }
}

#[component]
pub fn CalendarSelectMonth(props: CalendarSelectMonthProps) -> Element {
    rsx! {
        calendar::CalendarSelectMonth { class: "calendar-month-select", attributes: props.attributes }
    }
}

#[component]
pub fn CalendarSelectYear(props: CalendarSelectYearProps) -> Element {
    rsx! {
        calendar::CalendarSelectYear { class: "calendar-year-select", attributes: props.attributes }
    }
}

#[component]
pub fn CalendarGrid(props: CalendarGridProps) -> Element {
    rsx! {
        calendar::CalendarGrid {
            id: props.id,
            show_week_numbers: props.show_week_numbers,
            render_day: props.render_day,
            attributes: props.attributes,
        }
    }
}

#[component]
pub fn CalendarMonthTitle(props: CalendarMonthTitleProps) -> Element {
    calendar::CalendarMonthTitle(props)
}

#[derive(Props, Clone, Debug, PartialEq)]
pub struct CustomCalendarDayProps {
    date: Date,
    class: Option<String>,
    children: Element,
    /// Additional attributes to extend the calendar day element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn CustomCalendarDay(props: CustomCalendarDayProps) -> Element {
    let CustomCalendarDayProps {
        date,
        class,
        children,
        attributes,
    } = props;
    let mut attributes = attributes;
    if let Some(class) = &class {
        attributes.push(Attribute::new("class", class.clone(), None, true));
    }
    let calendar_props = CalendarDayProps::builder()
        .date(date)
        .attributes(attributes)
        .build();
    let class = class.unwrap_or_default();
    let day = date.day();
    let base_attributes = use_calendar_day_attributes(calendar_props);
    rsx! {
        button { class, ..base_attributes,
            {day.to_string()}
            {children}
        }
    }
}

#[component]
pub fn CalendarDay(props: CalendarDayProps) -> Element {
    calendar::CalendarDay(props)
}
