use crate::Route;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::IconShape;
use dioxus_free_icons::icons::ld_icons::{LdCalendar, LdHome, LdListTodo};

#[component]
pub fn Dock() -> Element {
    rsx! {
        div { class: "dock dock-md lg:hidden",
            DockItem {
                label: "Todos",
                icon: LdListTodo,
                to: Route::TodoListListView {},
            }
            DockItem { label: "Home", icon: LdHome, to: Route::Home {} }
            DockItem {
                label: "Calendar",
                icon: LdCalendar,
                to: Route::NotFound {
                    segments: { vec![] },
                }, //TODO:put in right to:Route(...) once created
            }
        }
    }
}
#[component]
pub fn DockItem<T: IconShape + Clone + PartialEq + 'static>(
    label: ReadSignal<String>,
    icon: T,
    to: Route,
) -> Element {
    rsx! {
        Link { to, active_class: "dock-active",
            Icon { class: "size-4", icon }
            span { class: "dock-label", {label} }
        }
    }
}
