use dioxus::prelude::*;

/// Controls how images and Card Body are placed in the Card
#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum CardVariant {
    #[default]
    Default,
    Horizontal,
    /// Stacks image and body vertically on mobile and horizontal on desktop
    Responsive,
}

impl CardVariant {
    fn class(&self) -> &'static str {
        match self {
            CardVariant::Default => "",
            CardVariant::Horizontal => "card-side",
            CardVariant::Responsive => "lg:card-side",
        }
    }
}

/// Generic Card component with a border and shadow.
/// Default usage:
/// ```
/// # use dioxus::prelude::*;
/// # rsx! {
/// Card {
///   CardBody { "..." }
/// }
/// # }
/// ```
/// In case you want an image in the card, place it outside the CardBody components like so:
/// ```
/// # use dioxus::prelude::*;
/// # rsx! {
/// Card {
///   figure { img {...} }
///   CardBody { "..." }
/// }
/// # }
/// ```
#[component]
pub fn Card(
    /// If true, places the first element behind the Card Body
    #[props(default)]
    image_full: bool,
    #[props(default)] variant: CardVariant,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let variant_class = variant.class();
    rsx!(
        div {
            class: "card card-border bg-base-100 shadow-sm {variant_class}",
            class: if image_full { "image-full" },
            ..attributes,
            {children}
        }
    )
}

#[component]
pub fn CardBody(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div { class: "card-body", ..attributes, {children} }
    }
}

#[component]
pub fn CardTitle(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        h2 { class: "card-title", ..attributes, {children} }
    }
}

#[component]
pub fn CardActions(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div { class: "card-actions justify-end", ..attributes, {children} }
    }
}
