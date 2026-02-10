use crate::components::ui::button::{Button, ButtonShape, ButtonVariant};
use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::ld_icons::{LdCircleCheck, LdCircleX, LdInfo, LdTriangleAlert, LdX},
};
use dioxus_sdk::time::use_timeout;
use std::time::Duration;

/// Style of the Toast. Controls color and icon
#[allow(dead_code)]
#[derive(Clone, Copy, Default, PartialEq, Debug)]
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
    fn class(self) -> &'static str {
        match self {
            ToastVariant::Plain => "",
            ToastVariant::Info => "alert-info",
            ToastVariant::Success => "alert-success",
            ToastVariant::Warning => "alert-warning",
            ToastVariant::Error => "alert-error",
        }
    }

    fn icon(self) -> Element {
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

/// Options for how the toaster behaves. Default values are
/// - No Description
/// - Always dismissable
/// - Gets removed after 5 second
///
/// For examples refer to [`use_toaster`]
#[derive(Clone, PartialEq, Debug)]
pub struct ToastOptions {
    /// Element rendered below the title. If omitted, the title fills the toast
    description: Option<Element>,
    /// Whether to dismiss a button to dismiss the toast
    close_button: bool,
    /// Duration after which the toast is removed. If set to none, the toast will never disappear on its own
    duration: Option<Duration>,
}

impl ToastOptions {
    #[must_use]
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            description: None,
            close_button: true,
            duration: Some(Duration::from_secs(5)),
        }
    }

    #[must_use]
    #[allow(dead_code)]
    pub fn description(mut self, description: Element) -> Self {
        self.description = Some(description);
        self
    }

    #[must_use]
    #[allow(dead_code)]
    pub fn not_dismissable(mut self) -> Self {
        self.close_button = false;
        self
    }

    #[must_use]
    #[allow(dead_code)]
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = Some(duration);
        self
    }

    #[must_use]
    #[allow(dead_code)]
    pub fn persistent(mut self) -> Self {
        self.duration = None;
        self
    }
}

/// Specification for a Toast
#[derive(Clone, PartialEq, Debug)]
pub struct Toast {
    id: uuid::Uuid,
    title: String,
    /// Toast Variant, modifies color and icon
    variant: ToastVariant,
    options: ToastOptions,
}

impl Toast {
    #[must_use]
    #[allow(dead_code)]
    #[deprecated(note = "Please use `toaster.custom()` instead")]
    pub fn new(
        title: String,
        description: Option<Element>,
        close_button: bool,
        variant: ToastVariant,
    ) -> Self {
        let mut options = ToastOptions::new();
        if let Some(description) = description {
            options = options.description(description);
        }
        if !close_button {
            options = options.not_dismissable();
        }
        Self::from_options(&title, variant, options)
    }

    fn from_options(title: &str, variant: ToastVariant, options: ToastOptions) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            title: title.to_string(),
            variant,
            options,
        }
    }
}

/// Toast Element with an icon and close button (if enabled in toast)
#[component]
pub fn ToastComponent(toast: Toast) -> Element {
    let mut toaster_state = use_toaster();
    let toast_class = toast.variant.class();
    let duration = toast.options.duration;

    let mut current_timeout = use_signal(|| None);
    let timeout = use_timeout(duration.unwrap_or(Duration::from_secs(60)), move |()| {
        current_timeout.set(None);
        let result = toaster_state.remove_toast(toast.id);
        if let Err(error) = result {
            warn!("Failed to remove toast {} with error: {}", toast.id, error);
        }
    });
    use_effect(move || {
        if duration.is_some() {
            let t = timeout.action(());
            current_timeout.set(Some(t));
        }
    });

    rsx! {
        div {
            class: "alert {toast_class}",
            onmouseenter: move |_| {
                if duration.is_some() && let Some(handler) = *current_timeout.read() {
                    handler.cancel();
                }
            },
            onmouseleave: move |_| {
                if duration.is_some() {
                    timeout.action(());
                }
            },
            {toast.variant.icon()}
            div {
                h5 { class: "font-bold", {toast.title} }
                {toast.options.description}
            }
            if toast.options.close_button {
                Button {
                    variant: ButtonVariant::None,
                    shape: ButtonShape::Round,
                    ghost: true,
                    class: "btn-sm",
                    onclick: move |_| {
                        let result = toaster_state.remove_toast(toast.id);
                        if let Err(error) = result {
                            warn!("Failed to remove toast {} with error: {}", toast.id, error);
                        }
                    },
                    Icon { icon: LdX }
                }
            }
        }

    }
}

#[derive(Clone, Copy, Debug)]
pub struct ToasterState {
    toasts: Signal<Vec<Toast>>,
}

impl ToasterState {
    /// Adds a toast to the UI
    #[deprecated(note = "Please use `toaster.custom()` instead")]
    pub fn toast(&mut self, toast: Toast) {
        self.toasts.push(toast);
    }

    #[allow(dead_code)]
    pub fn success(&mut self, title: &str, options: ToastOptions) {
        self.custom(title, ToastVariant::Success, options);
    }

    #[allow(dead_code)]
    pub fn error(&mut self, title: &str, options: ToastOptions) {
        self.custom(title, ToastVariant::Error, options);
    }

    #[allow(dead_code)]
    pub fn info(&mut self, title: &str, options: ToastOptions) {
        self.custom(title, ToastVariant::Info, options);
    }

    #[allow(dead_code)]
    pub fn warning(&mut self, title: &str, options: ToastOptions) {
        self.custom(title, ToastVariant::Warning, options);
    }

    #[allow(dead_code)]
    pub fn plain(&mut self, title: &str, options: ToastOptions) {
        self.custom(title, ToastVariant::Plain, options);
    }

    #[allow(dead_code)]
    pub fn custom(&mut self, title: &str, variant: ToastVariant, options: ToastOptions) {
        #[allow(deprecated)] // TODO: Remove after `toast` has been removed everywhere
        self.toast(Toast::from_options(title, variant, options));
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

/// Provider and Renderer for Toasts
///
/// # Examples
/// For examples refer to [`use_toaster`]
#[component]
pub fn ToastProvider(children: Element) -> Element {
    let toasts = use_signal(Vec::<Toast>::new);

    use_context_provider(|| ToasterState { toasts });

    rsx! {
        {children}
        div { class: "toast toast-top md:max-w-1/3",
            {toasts.iter().map(|toast| rsx! {
                ToastComponent { key: "{toast.id}", toast: toast.clone() }
            })}
        }
    }
}

/// Hook which returns a [`ToasterState`] on which toasts can be added
///
/// # Panics
/// Panics if the hook is called outside `ToastProvider`
///
/// # Examples
/// ```ignore
/// let mut toaster = use_toaster();
/// rsx! {
///     Button {
///         onclick: |_| {
///             // Adds a success toast with a description of "Hello World" which can only be removed using the close button
///             toaster.success(
///                 "Example Toast",
///                 ToastOptions::new()
///                     .description(rsx! { span{ "Hello World!" }})
///                     .persistent()
///             );
///          },
///         "Click me!"
///     }
/// }
/// ```
pub fn use_toaster() -> ToasterState {
    try_consume_context::<ToasterState>().expect("Toaster can only be used in a ToastProvider")
}
