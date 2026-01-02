use crate::components::ui::sidebar::Sidebar;
use dioxus::prelude::*;

#[derive(Clone)]
pub struct SidebarState {
    pub visible: Signal<bool>,
}

impl SidebarState {}

#[component]
pub fn SidebarProvider(children: Element) -> Element {
    let sidebar_visible = use_signal(|| false);

    use_context_provider(|| SidebarState {
        visible: sidebar_visible,
    });

    rsx! {
        div { class: "drawer lg:drawer-open",
            input {
                id: "drawer-toggle",
                r#type: "checkbox",
                class: "drawer-toggle",
                checked: sidebar_visible,
            }
            div { class: "drawer-content", {children} }
            Sidebar {}
        }
    }
}
