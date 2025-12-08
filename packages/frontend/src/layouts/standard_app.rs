use dioxus::prelude::*;

use crate::{
    Route,
    components::{Footer, Navbar, sidebar::SidebarProvider},
};

#[component]
pub fn StandardAppLayout(children: Element) -> Element {
    rsx! {
        SidebarProvider {
            div { class: "min-h-screen flex flex-col",
                input {
                    id: "drawer-toggle",
                    r#type: "checkbox",
                    class: "drawer-toggle",
                }
                div {
                    Navbar {}
                    main { class: "grow mx-10 mt-5", Outlet::<Route> {} }
                }
            }
            Footer {}
        }
    }
}
