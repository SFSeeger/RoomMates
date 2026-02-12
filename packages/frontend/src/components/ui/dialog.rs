use crate::components::ui::button::{Button, ButtonProps, ButtonShape, ButtonVariant};
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::LdX;
use uuid::Uuid;

#[derive(Clone, Copy, PartialEq)]
pub struct DialogContext {
    pub id: Signal<Uuid>,
}

impl DialogContext {
    #[must_use]
    pub fn new(uuid: Uuid) -> Self {
        DialogContext {
            id: Signal::new(uuid),
        }
    }

    pub fn open(&self) {
        let _eval = document::eval(&format!(
            "document.getElementById('dialog-{}').showModal()",
            self.id,
        ));
    }

    #[allow(dead_code)]
    pub fn close(&self) {
        let _eval = document::eval(&format!(
            "document.getElementById('dialog-{}').close()",
            self.id,
        ));
    }
}

/// Root of the dialog component
/// Provides a context of type [`DialogContext`] which holds relevant information for the dialog.
/// A Dialog should contain [`DialogTrigger`] and [`DialogContent`].
///
/// # Example
/// ```ignore
/// rsx! {
///   Dialog {
///      DialogTrigger { "click me!" }
///      DialogContent {
///         "You can press Esc to close this Modal"
///     }
///   }
/// }
/// ```
#[component]
pub fn Dialog(children: Element) -> Element {
    let dialog_id = use_server_cached(Uuid::new_v4);
    use_context_provider(|| DialogContext::new(dialog_id));

    children
}

/// Wrapper around [Button] which controls a Dialog. Since it shares all the Properties of Button, the
/// trigger can be fully customized.
///
/// # Example
/// ```ignore
/// rsx! {
///     DialogTrigger {
///         variant: ButonVariant::Success,
///         ghost: true,
///         "Click Me!"
///     }
/// }
/// ```
#[component]
pub fn DialogTrigger(props: ButtonProps) -> Element {
    let context = use_dialog();

    rsx! {
        Button {
            onclick: move |event| {
                if let Some(f) = &props.onclick {
                    f.call(event);
                }
                context.open();
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
            variant: props.variant,
            disabled: props.disabled,
            ghost: props.ghost,
            shape: props.shape,
            class: props.class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

/// Main content of the Dialog. Uses a `<dialog>` to handle opening and closing.
/// To ensure valid screen reader interaction, a title must be supplied.
///
/// # Example
/// ```
/// rsx!{
///     DialogContent {
///         title: "Example Modal"
///         "This Modal can be dismissed using Esc, The close button or by pressing outside the modal"
///     }
///     DialogContent {
///         title: "Example Modal 2"
///         dismissible: false,
///         close_button: false,
///         "This Modal can only be dismissed using Esc"
///     }
/// }
/// ```
#[component]
pub fn DialogContent(
    title: ReadSignal<String>,
    #[props(default = true)] dismissible: bool,
    #[props(default = true)] close_button: bool,
    children: Element,
) -> Element {
    let context = use_dialog();
    let id = context.id;

    rsx! {
        dialog {
            class: "modal modal-bottom sm:modal-middle",
            id: "dialog-{id}",
            aria_labelledby: "dialog-title-{id}",
            div { class: "modal-box",
                if close_button {
                    form { method: "dialog",
                        Button {
                            variant: ButtonVariant::None,
                            shape: ButtonShape::Round,
                            ghost: true,
                            class: "btn-sm absolute right-2 top-2",
                            aria_label: "Close",
                            Icon { icon: LdX }
                        }
                    }
                }
                h3 { class: "text-lg font-bold", id: "dialog-title-{id}", "{title}" }
                {children}
            }
            if dismissible {
                form { method: "dialog", class: "modal-backdrop",
                    button { "close" }
                }
            }
        }
    }
}

/// Actions for the dialog. Can be used to display e.g. a submit button.
/// If you want to close the modal from a button inside the actions either consume the [`DialogContext`] and call
/// `DialogContext.close()` or using a `form` with `method: "dialog"`
#[component]
pub fn DialogAction(children: Element) -> Element {
    rsx! {
        div { class: "modal-action", {children} }
    }
}

/// Returns a [`DialogContext`] to enable control a dialog.
///
/// # Panics
///
/// Panics if used outside Dialog
pub fn use_dialog() -> DialogContext {
    try_use_context::<DialogContext>().expect("`use_dialog` can only be used inside Dialog!")
}
