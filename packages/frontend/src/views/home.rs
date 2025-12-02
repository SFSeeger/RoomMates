use crate::components::Echo;
use api::list_events;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let events = use_loader(move || async move { list_events().await })?;

    rsx! {
        div {
            h1 { "Your Events" }
            ul {
                for event in events.read().cloned() {
                    li { key: "{event.id}", "{event.title}" }
                }
            }
        }
        Echo {}
    }
}
