use dioxus::prelude::*;

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq, Default)]
pub enum TooltipPlacement {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

impl TooltipPlacement {
    #[allow(dead_code)] // TODO: Remove when this component is in use
    fn class(&self) -> &'static str {
        match self {
            TooltipPlacement::Top => "tooltip-top",
            TooltipPlacement::Bottom => "tooltip-bottom",
            TooltipPlacement::Left => "tooltip-left",
            TooltipPlacement::Right => "tooltip-right",
        }
    }
}

/// Wrapper component for displaying a tooltip when hovering over the child element
#[component]
pub fn Tooltip(
    #[props(default)] placement: TooltipPlacement,
    tooltip: ReadSignal<String>,
    children: Element,
) -> Element {
    let placement_class = placement.class();

    rsx! {
        div { class: "tooltip {placement_class}", "data-tip": tooltip, {children} }
    }
}
