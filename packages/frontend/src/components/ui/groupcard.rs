use crate::Route;
use crate::components::ui::card::{Card, CardBody, CardTitle};
use api::routes::groups::retrieve_group;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::LdSquarePen;

#[component]
pub fn GroupCard(group_id: i32) -> Element {
    let group_data = use_server_future(move || async move { retrieve_group(group_id).await })?;

    rsx! {
        div {
            Card {
                CardBody {
                    match &*group_data.read() {
                        Some(Ok(group)) => rsx! {
                            CardTitle { "{group.name}" }
                            Link {
                                to: Route::EditGroup { group_id },
                                class: "absolute top-2 right-2 btn btn-primary btn-circle lg:btn-lg",
                                Icon { icon: LdSquarePen }
                            }
                            div { class: "flex flex-row text-center",
                                div { class: " flex-1",
                                    h3 { class: "font-bold", {"Members"} }
                                    div { class: "h-2" }
                                    {group.members.iter().take(10).map(|member| rsx! {
                                        div { "{member.first_name} {member.last_name}" }
                                    })}
                                }
                                div { class: " flex-1",
                                    h3 { class: "font-bold", {"Events"} }
                                    div { class: "h-2" }


                                    {group.events.iter().take(10).map(|event| rsx! {
                                        div { "{event.title}" }
                                    })}
                                }
                            }
                        },
                        Some(Err(error)) => rsx! {
                            p { class: "text-red-500", "Loading group failed with {error}" }
                        },
                        None => rsx! {
                            p { "Loading..." }
                        },
                    }
                }
            }
        }
    }
}
