use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "footer sm:footer-horizontal bg-neutral text-neutral-content p-10",
            nav {
                h6 { class: "footer-title", "RoomMates" }
                Link { to: Route::Home {}, class: "link link-hover", "Home" }
            }
        }
    }
}
