use dioxus::prelude::*;
use dioxus_free_icons::{Icon, IconShape};

use crate::{
    Route,
    components::ui::button::{Button, ButtonVariant},
};

#[component]
pub fn ErrorDisplay<T: IconShape + Clone + PartialEq + 'static>(
    title: ReadSignal<String>,
    description: ReadSignal<String>,
    action_text: ReadSignal<String>,
    icon: Option<T>,
    redirect_route: Route,
    error_context: Option<ErrorContext>,
) -> Element {
    let nav = use_navigator();
    let button_component = match error_context {
        Some(context) => {
            let onclick = move |_| {
                context.clear_errors();
                nav.push(redirect_route.clone());
            };
            rsx! {
                Button {
                    variant: ButtonVariant::Primary,
                    class: "mt-4",
                    onclick,
                    {action_text}
                }
            }
        }
        None => rsx! {
            Link { to: redirect_route.clone(), class: "btn btn-primary mt-4", {action_text} }
        },
    };

    rsx! {
        div { class: "flex flex-col items-center gap-4 justify-center text-center h-full",
            if let Some(icon) = icon {
                Icon { class: "size-30", icon }
            }
            h1 { class: "text-2xl font-bold", {title} }
            p { {description} }
            {button_component}
        }
    }
}
