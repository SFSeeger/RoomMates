use crate::components::Echo;
use api::routes::events::list_events;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let events = use_loader(move || async move { list_events().await });

    rsx! {
        div {
            h1 { class: "text-xl font-bold mb-2", "Events owned by user 1" }
            if let Ok(events) = events {
                ul { class: "list-disc",
                    for event in events.read().cloned() {
                        li { key: "{event.id}", "{event.title}" }
                    }
                }
            } else {
                p { class: "text-red-500", "Getting User 1 caused an error" }
            }
        }
        Echo {}
    }
}
