use crate::components::Echo;
use api::routes::events::list_events;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let events = use_server_future(move || async move { list_events().await })?;

    rsx! {
        div {
            div {
                h1 { class: "text-xl font-bold mb-2", "Events owned by user 1" }
                match &*events.read() {
                    Some(Ok(data)) => rsx! {
                        ul { class: "list-disc pl-5",
                            {data.iter().map(|event| rsx! {
                                li { key: "{event.id}", "{event.title}" }
                            })}
                        }
                    },
                    Some(Err(error)) => rsx! {
                        p { class: "text-red-500", "Loading Events failed with {error}" }
                    },
                    None => rsx! {
                        p { "Loading..." }
                    },
                }
            }
        }
        Echo {}
    }
}
