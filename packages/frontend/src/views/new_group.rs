use dioxus::prelude::*;
//use crate::components::ui::forms::use_form_field;

#[component]
pub fn NewGroup() -> Element {
    rsx! {
        div {
            div {
                h1 { class: "text-2xl font-bold text-center relative ", "Edit your group " }
                div { class: "text-center", "group_name" }
            }
            div { class: "checkbox toggle display: none",
                aside { class: "aside-wrapper" }
            }
        }
    }
}
