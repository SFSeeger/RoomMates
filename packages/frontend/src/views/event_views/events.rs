use crate::Route;
use crate::components::ui::events::eventlist::{EventList, SharedEventList};
use crate::components::ui::{
    button::{Button, ButtonShape, ButtonVariant},
    card::{Card, CardBody},
};

use dioxus::prelude::*;
use dioxus::router::FromQueryArgument;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::LdPlus;
use std::fmt::{Display, Formatter};
use time::Date;
use time::macros::format_description;

#[derive(Default, Clone, PartialEq, Debug)]
pub struct DateQueryParam(Option<Date>);

impl DateQueryParam {
    pub fn value(&self) -> Option<Date> {
        self.0
    }
}

impl FromQueryArgument for DateQueryParam {
    type Err = ();

    fn from_query_argument(argument: &str) -> std::result::Result<Self, Self::Err> {
        Ok(argument.into())
    }
}

impl From<&str> for DateQueryParam {
    fn from(value: &str) -> Self {
        if value.is_empty() {
            DateQueryParam(None)
        } else {
            match Date::parse(value, format_description!("[year]-[month]-[day]")) {
                Ok(date) => date.into(),
                Err(_) => DateQueryParam(None),
            }
        }
    }
}
impl From<Option<Date>> for DateQueryParam {
    fn from(value: Option<Date>) -> Self {
        DateQueryParam(value)
    }
}

impl From<Date> for DateQueryParam {
    fn from(value: Date) -> Self {
        DateQueryParam(Some(value))
    }
}

impl Display for DateQueryParam {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.value() {
            Some(date) => write!(
                f,
                "{}",
                date.format(format_description!("[year]-[month]-[day]"))
                    .unwrap()
            ),
            None => write!(f, ""),
        }
    }
}

#[component]
pub fn ListEventView(date: DateQueryParam) -> Element {
    rsx! {
        div {
            Link {
                to: Route::AddEventView {
                    group_id: None.into(),
                },
                Button { variant: ButtonVariant::Primary, shape: ButtonShape::Wide,
                    Icon { icon: LdPlus }
                    "create new event"
                }
            }
        }
        div { class: "divider" }
        div { class: "w-full",
            Card {
                CardBody {
                    EventList { date: date.value() }
                }
            }
        }

        div { class: "divider" }
        div { class: "w-full",
            Card {
                CardBody { SharedEventList {} }
            }
        }
    }
}
