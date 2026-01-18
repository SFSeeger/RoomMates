use dioxus::prelude::*;

#[component]
pub fn Fieldset(
    title: Option<String>,
    children: Element,
    #[props(default)] has_background: bool,
) -> Element {
    rsx! {
        fieldset {
            class: "fieldset rounded-box p-4",
            class: if has_background { "bg-base-200 border-base-300" },
            if title.is_some() {
                legend { class: "fieldset-legend", {title} }
            }
            {children}
        }
    }
}
