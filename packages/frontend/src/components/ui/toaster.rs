use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::ld_icons::{LdCircleCheck, LdCircleX, LdInfo, LdTriangleAlert, LdX},
};

use crate::components::ui::button::{Button, ButtonShape, ButtonVariant};

/// Style of the Toast. Controls colro and icon
#[allow(dead_code)]
#[derive(Clone, Default, PartialEq, Debug)]
pub enum ToastVariant {
    /// Toast without an icon and base background color
    #[default]
    Plain,
    Info,
    Success,
    Warning,
    Error,
}

impl ToastVariant {
    fn class(&self) -> &'static str {
        match self {
            ToastVariant::Plain => "",
            ToastVariant::Info => "alert-info",
            ToastVariant::Success => "alert-success",
            ToastVariant::Warning => "alert-warning",
            ToastVariant::Error => "alert-error",
        }
    }

    fn icon(&self) -> Element {
        match self {
            ToastVariant::Plain => rsx! {},
            ToastVariant::Info => rsx! {
                Icon { icon: LdInfo }
            },
            ToastVariant::Success => rsx! {
                Icon { icon: LdCircleCheck }
            },
            ToastVariant::Warning => rsx! {
                Icon { icon: LdTriangleAlert }
            },
            ToastVariant::Error => rsx! {
                Icon { icon: LdCircleX }
            },
        }
    }
}

/// Specification for a Toast
#[derive(Clone, PartialEq, Debug)]
pub struct Toast {
    id: uuid::Uuid,
    pub title: String,
    /// Element rendered below the title. If omitted, the title fills the toast
    pub description: Option<Element>,
    /// Whether to dismiss a button to dismiss the toast
    pub close_button: bool,
    /// Toast Variant, modifies color and icon
    pub variant: ToastVariant,
}

impl Toast {
    #[allow(dead_code)] //Todo remove after usage
    pub fn new(
        title: String,
        description: Option<Element>,
        close_button: bool,
        variant: ToastVariant,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            title,
            description,
            close_button,
            variant,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ToasterState {
    toasts: Signal<Vec<Toast>>,
}

impl ToasterState {
    /// Adds a toast to the UI
    #[allow(dead_code)] //Todo remove after usage
    pub fn toast(&mut self, toast: Toast) {
        self.toasts.push(toast);
    }

    /// Removes a toast from the UI
    pub fn remove_toast(&mut self, id: uuid::Uuid) -> Result<()> {
        let index = self
            .toasts
            .read()
            .iter()
            .position(|toast| toast.id == id)
            .ok_or(anyhow::anyhow!("Could not find toast"))?;
        self.toasts.remove(index);
        Ok(())
    }
}

/// Toast Element with an icon and close button (if enabled in toast)
#[component]
pub fn ToastComponent(toast: Toast) -> Element {
    let mut toaster_state = use_context::<ToasterState>();
    let toast_class = toast.variant.class();

    rsx! {
        div { class: "alert {toast_class}",
            {toast.variant.icon()}
            div {
                h5 { class: "font-bold", {toast.title} }
                {toast.description}
            }
            if toast.close_button {
                Button {
                    variant: ButtonVariant::None,
                    shape: ButtonShape::Round,
                    ghost: true,
                    class: "btn-sm",
                    onclick: move |_| {
                        let _ = toaster_state.remove_toast(toast.id);
                    },
                    Icon { icon: LdX }
                }
            }
        }

    }
}

/// Provider and Renderer for Toasts
/// Example usage:
/// ```
/// # use dioxus::prelude::*;
/// #[component]
/// fn ToastButton() {
///     let mut toaster_state = use_context::<ToasterState>();
///     rsx! {
///         button {
///             onclick: move |_| {
///                 // Add a new Toast to the UI
///                 toaster_state.toast(Toast::new(/* ... */))
///              },
///             "Add toast"
///         }
///     }
/// }
///
/// rsx! {
///     ToastProvider {
///         ToastButton { }
///     }
/// }
///
/// ```
#[component]
pub fn ToastProvider(children: Element) -> Element {
    let toasts = use_signal(Vec::<Toast>::new);

    use_context_provider(|| ToasterState { toasts });

    rsx! {
        {children}
        div { class: "toast toast-top",
            {toasts.iter().map(|toast| rsx! {
                ToastComponent { key: "{toast.id}", toast: {toast.clone()} }
            })}
        }
    }
}
