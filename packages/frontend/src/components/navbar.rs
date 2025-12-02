use crate::Route;
use dioxus::prelude::*;
#[component]
pub fn Navbar() -> Element {
    rsx! {
        div { class: "bg-gray-800 text-white p-4 flex space-x-4",
            Link { to: Route::Home {}, "Home" }
        }
        main { class: "px-10 pt-4", Outlet::<Route> {} }
    }
}
