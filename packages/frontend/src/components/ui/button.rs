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
        }
    }
}

#[component]
pub fn Button(
    #[props(default)] variant: ButtonVariant,
    #[props(extends=GlobalAttributes)]
    #[props(extends=button)]
    attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    onmousedown: Option<EventHandler<MouseEvent>>,
    onmouseup: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let variant_class = variant.class();
    rsx! {
        button {
            class: "btn {variant_class}",
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
