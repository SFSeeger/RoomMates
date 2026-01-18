use dioxus::prelude::*;

#[component]
pub fn Fieldset(children: Element) -> Element {
    rsx! {
        fieldset { class: "fieldset bg-base-200 border-base-300 rounded-box w-xs p-4",
            legend { class: "fieldset-legend", "Enter your name" }
            {children}
        }
    }
}
//TODO: Add title, placeholder and label as needed
