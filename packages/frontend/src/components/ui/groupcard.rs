use crate::components::ui::button;
use crate::components::ui::card::{Card, CardBody, CardTitle};
use crate::{Route, components::ui::button::Button};
use api::routes::groups::GroupCardData;
use dioxus::prelude::*;

#[component()]
pub fn GroupCard(data: GroupCardData) -> Element {
    let members = data.members.clone();
    let events = data.events.clone();
    let name = data.name.clone();
    rsx! {
        Card {
            CardBody {
                CardTitle { "{name}" }
                div { class: "absolute top-2 right-2",
                    Link { to: Route::NewGroup {},
                        Button {
                            variant: button::ButtonVariant::Primary,
                            ghost: false,
                            shape: button::ButtonShape::Round,
                            disabled: false,
                            "edit group"
                        }
                    }
                }
                div { class: "flex flex-row text-center",
                    div { class: " flex-1",
                        h3 { class: "font-bold", {"Members"} }
                        div { class: "h-2" }
                        {members.iter().take(10).map(|member| rsx! {
                            div { "{member.first_name} {member.last_name}" }
                        })}
                    }
                    div { class: " flex-1",
                        h3 { class: "font-bold", {"Events"} }
                        div { class: "h-2" }

                        {events.iter().take(10).map(|event| rsx! {
                            div { "{event.title}" }
                        })}
                    }
                }
            }
        }
    }
}
