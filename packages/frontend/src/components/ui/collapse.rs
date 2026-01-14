use dioxus::prelude::*;

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq, Default)]
pub enum CollapseVariant {
    #[default]
    Arrow,
    Plus,
    None,
}

impl CollapseVariant {
    #[allow(dead_code)]
    fn class(&self) -> &'static str {
        match self {
            CollapseVariant::Arrow => "collapse-arrow",
            CollapseVariant::Plus => "collapse-plus",
            CollapseVariant::None => "",
        }
    }
}

#[component]
pub fn Collapse(
    #[props(default)] variant: CollapseVariant,
    title: ReadSignal<String>,
    children: Element,
) -> Element {
    let variant_class = variant.class();
    rsx! {
        div { class: "collapse bg-base-100 border-base-300 border {variant_class}",
            input { r#type: "checkbox" }
            div { class: "collapse-title font-semibold", {title} }
            div { class: "collapse-content text-sm", {children} }
        }

    }
}
