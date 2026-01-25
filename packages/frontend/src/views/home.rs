use crate::components::{
    Echo,
    contexts::{AuthGuard, AuthState},
};
use api::routes::events::list_events;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let events = use_server_future(move || async move { list_events().await })?;

    let auth_state = use_context::<AuthState>();
    let user_name = use_memo(move || {
        auth_state
            .user
            .read()
            .as_ref()
            .map(|u| format!("{} {}", u.first_name, u.last_name))
            .unwrap_or("???".to_string())
    });

    rsx! {
        AuthGuard {
            div {
                div {
                    h1 { class: "text-xl font-bold mb-2", "Events owned by you ({user_name}):" }
                    match &*events.read() {
                        Some(Ok(data)) => rsx! {
                            ul { class: "list-disc pl-5",
                                if data.is_empty() {
                                    li { "No events found." }
                                } else {
                                    {data.iter().map(|event| rsx! {
                                        li { key: "{event.id}", "{event.title}" }
                                    })}
                                }
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
}
