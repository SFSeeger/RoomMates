use crate::Route;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::LdCircleHelp;

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    rsx! {
        div { class: "flex flex-col items-center gap-4 justify-center h-full",
            Icon { class: "size-30", icon: LdCircleHelp }
            h1 { class: "text-2xl font-bold text-center", "404 - Page not found" }
            Link { class: "btn btn-lg btn-outline", to: Route::Home {}, "Return to start" }
        }
    }
}
