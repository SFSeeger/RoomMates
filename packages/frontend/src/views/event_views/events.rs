use crate::Route;
use crate::components::ui::list::{List, ListRow};
use crate::components::ui::{
    button::{Button, ButtonShape, ButtonVariant},
    card::{Card, CardBody, CardTitle},
};
use api::routes::events::list_events;
use dioxus::prelude::*;

//use dioxus_free_icons::Icon;
//use dioxus_free_icons::icons::ld_icons::LdCircleHelp;
// div { class: "flex flex-col items-center gap-4 justify-center h-full" }

#[component]
pub fn Events() -> Element {
    rsx! {
        div {
            Link { to: Route::EventCreator {},
                Button { variant: ButtonVariant::Primary, shape: ButtonShape::Wide, "+ create new event" }
            }
        }
        div { class: "divider" }
        div { class: "flex w-full",

            Card {
                CardBody { Event_List {} }
            }
        }
    }
}

#[component]
pub fn Event_List() -> Element {
    let events = use_server_future(move || async move { list_events().await })?;

    rsx! {

        match &*events.read() {
            Some(Ok(data)) => rsx! {
                List { header: "Your Events",
                    {data.iter().map(|event| rsx! {
                        ListRow { key: "{event.id}",
                            Card {
                                CardTitle { "{event.title}" }
                                CardBody {
                                    div { class: "flex flex-row gap-4 justify-content h-full",
                                        div {
                                            h1 { "Date: {event.date}" }
                                            p {
                                                "begins at:"
                                                "{event.start_time}"
                                            }
                                            p {
                                                "ends at:"
                                                "{event.end_time}"
                                            }
                                        }
                                        div {
                                            Link {
                                                to: Route::EventEditor {
                                                    event_id: event.id,
                                                },
                                                Button { variant: ButtonVariant::Info, "Edit" }
                                            }
                                            Button { variant: ButtonVariant::Error, "Delete" }
                                        }
                                    }
                                }
                            }
                        }
                    })}
                }
            },
            Some(Err(err)) => rsx! {
                p { class: "text-red-500", "Loading Events failed with {err}" }
            },
            None => rsx! {
                p { "cant connect to db" }
            },
        }
    }
}
