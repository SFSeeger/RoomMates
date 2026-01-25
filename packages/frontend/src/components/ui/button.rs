use dioxus::prelude::*;

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Accent,
    Info,
    Success,
    Warning,
    Error,
    None,
}

impl ButtonVariant {
    #[allow(dead_code)] // TODO: Remove when this component is in use
    fn class(&self) -> &'static str {
        match self {
            ButtonVariant::Primary => "btn-primary",
            ButtonVariant::Secondary => "btn-secondary",
            ButtonVariant::Accent => "btn-accent",
            ButtonVariant::Info => "btn-info",
            ButtonVariant::Success => "btn-success",
            ButtonVariant::Warning => "btn-warning",
            ButtonVariant::Error => "btn-error",
            ButtonVariant::None => "",
        }
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq, Default)]
pub enum ButtonShape {
    #[default]
    Default,
    Round,
    Square,
    Block,
    Wide,
}

impl ButtonShape {
    #[allow(dead_code)] // TODO: Remove when this component is in use
    fn class(&self) -> &'static str {
        match self {
            ButtonShape::Default => "",
            ButtonShape::Round => "btn-round",
            ButtonShape::Square => "btn-square",
            ButtonShape::Block => "btn-block",
            ButtonShape::Wide => "btn-wide",
        }
    }
}

#[component]
pub fn Button(
    #[props(default)] variant: ButtonVariant,
    #[props(default)] ghost: bool,
    #[props(default)] shape: ButtonShape,
    #[props(default)] disabled: bool,
    #[props(extends=GlobalAttributes, extends=button)] attributes: Vec<Attribute>,
    class: Option<String>,
    onclick: Option<EventHandler<MouseEvent>>,
    onmousedown: Option<EventHandler<MouseEvent>>,
    onmouseup: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let variant_class = variant.class();
    let shape_class = shape.class();
    let class = class.unwrap_or_default();
    rsx! {
        button {
            class: "btn {variant_class} {shape_class} {class}",
            class: if ghost { "btn-ghost" },
            class: if disabled { "btn-disabled" },
            disabled,
            onclick: move |event| {
                if let Some(f) = &onclick {
                    f.call(event);
                }
            },
            onmousedown: move |event| {
                if let Some(f) = &onmousedown {
                    f.call(event);
                }
            },
            onmouseup: move |event| {
                if let Some(f) = &onmouseup {
                    f.call(event);
                }
            },
            ..attributes,
            {children}
        }
    }
}
