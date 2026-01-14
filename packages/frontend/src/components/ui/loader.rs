use dioxus::prelude::*;

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq, Default)]
pub enum LoaderSize {
    ExtraSmall,
    Small,
    #[default]
    Medium,
    Large,
    VeryLarge,
}

impl LoaderSize {
    #[allow(dead_code)]
    fn class(&self) -> &'static str {
        match self {
            LoaderSize::ExtraSmall => "loading-xs",
            LoaderSize::Small => "loading-sm",
            LoaderSize::Medium => "loading-md",
            LoaderSize::Large => "loading-lg",
            LoaderSize::VeryLarge => "loading-xl",
        }
    }
}

/// Simple Loading Spinner
#[component]
pub fn Loader(
    #[props(default)] size: LoaderSize,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let size_class = size.class();

    rsx! {
        span { class: "loading loading-spinner {size_class}", ..attributes }
    }
}
