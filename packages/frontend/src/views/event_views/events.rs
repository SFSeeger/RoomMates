use crate::Route;
use crate::components::ui::events::eventlist::EventList;
use crate::components::ui::{
    button::{Button, ButtonShape, ButtonVariant},
    card::{Card, CardBody},
};
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
