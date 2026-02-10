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
            ButtonShape::Round => "btn-circle",
            ButtonShape::Square => "btn-square",
            ButtonShape::Block => "btn-block",
            ButtonShape::Wide => "btn-wide",
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct ButtonProps {
    #[props(default)]
    pub variant: ButtonVariant,
    #[props(default)]
    pub ghost: bool,
    #[props(default)]
    pub outline: bool,
    #[props(default)]
    pub shape: ButtonShape,
    #[props(default)]
    pub disabled: bool,
    #[props(extends=GlobalAttributes, extends=button)]
    pub attributes: Vec<Attribute>,
    pub class: Option<String>,
    pub onclick: Option<EventHandler<MouseEvent>>,
    pub onmousedown: Option<EventHandler<MouseEvent>>,
    pub onmouseup: Option<EventHandler<MouseEvent>>,
    pub children: Element,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let variant_class = props.variant.class();
    let shape_class = props.shape.class();
    let class = props.class.unwrap_or_default();
    rsx! {
        button {
            class: "btn {variant_class} {shape_class} {class}",
            class: if props.ghost { "btn-ghost" },
            class: if props.outline { "btn-outline" },
            class: if props.disabled { "btn-disabled" },
            disabled: props.disabled,
            onclick: move |event| {
                if let Some(f) = &props.onclick {
                    f.call(event);
                }
            },
            onmousedown: move |event| {
                if let Some(f) = &props.onmousedown {
                    f.call(event);
                }
            },
            onmouseup: move |event| {
                if let Some(f) = &props.onmouseup {
                    f.call(event);
                }
            },
            ..props.attributes,
            {props.children}
        }
    }
}
