use crate::components::ui::button::Button;
use dioxus::prelude::*;
use form_hooks::use_form::FormState;

/// Submit Button for Forms. When not supplied, `label` defaults to "Submit" and `submitting_label` to "Submitting"
/// # Example
/// ```
/// SubmitButton {
///     form = form_state.clone()
///     label: "Save"
///     submittin_label: "Saving..."
/// }
/// ```
#[component]
pub fn SubmitButton(
    form: FormState,
    label: Option<String>,
    submitting_label: Option<String>,
    #[props(extends=GlobalAttributes, extends=button)] attributes: Vec<Attribute>,
) -> Element {
    let label = label.unwrap_or("Submit".to_string());
    let submitting_label = submitting_label.unwrap_or("Submitting".to_string());

    rsx! {
        Button {
            disabled: form.has_errors() || !*form.is_touched.read(),
            class: "w-full",
            if *form.is_submitting.read() {
                {submitting_label}
            } else {
                {label}
            }
        }
    }
}
