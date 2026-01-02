use crate::components::ui::card::{Card, CardBody, CardTitle};
use dioxus::prelude::*;

/// Echo component that demonstrates fullstack server functions.
#[component]
pub fn Echo() -> Element {
    let mut response = use_signal(String::new);
    rsx! {
        Card {
            CardBody {
                CardTitle { "ServerFn Echo" }
                input {
                    placeholder: "Type here to echo...",
                    oninput: move |event| async move {
                        let data = api::echo(event.value()).await.unwrap();
                        response.set(data);
                    },
                }
                if !response().is_empty() {
                    p {
                        "Server echoed: "
                        i { "{response}" }
                    }
                }
            }
        }
    }
}
