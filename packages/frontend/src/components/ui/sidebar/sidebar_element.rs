use crate::Route;
use crate::components::ui::sidebar::sidebar_provider::SidebarState;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::IconShape;
use dioxus_free_icons::icons::ld_icons::{LdHome, LdLibrary};

#[component]
pub fn Sidebar() -> Element {
    let mut sidebar_state = use_context::<SidebarState>();

    rsx! {
        aside { class: "drawer-side is-drawer-close:overflow-visible overflow-y-hidden",
            button {
                class: "drawer-overlay",
                onclick: move |_| sidebar_state.visible.toggle(),
            }
            div { class: "flex min-h-full flex-col items-start bg-base-200 is-drawer-close:w-14 is-drawer-open:w-64",
                ul { class: "menu w-full grow",
                    SidebarItem { title: "Homepage", icon: LdHome, to: Route::Home {} }
                    SidebarItem {
                        title: "Library",
                        icon: LdLibrary,
                        to: Route::Home {},
                    }
                }
            }
        }
    }
}

#[component]
pub fn SidebarItem<T: IconShape + Clone + PartialEq + 'static>(
    title: ReadSignal<String>,
    icon: T,
    to: Route,
) -> Element {
    rsx! {
        li {
            Link {
                to,
                class: "is-drawer-close:tooltip is-drawer-close:tooltip-right",
                "data-tip": title,
                Icon { class: "size-4", icon }
                span { class: "is-drawer-close:hidden", {title} }
            }
        }
    }
}
