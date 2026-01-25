use dioxus::prelude::*;
use form_hooks::use_form_field::{AnyField, FieldValue, FormField};

#[component]
pub fn Input<T: FieldValue>(
    label: Option<String>,
    icon: Option<Element>,
    field: FormField<T>,
    #[props(extends=GlobalAttributes, extends=input)] attributes: Vec<Attribute>,
) -> Element {
    let field_attributes = field.field_attributes();
    let has_icon = icon.is_some();
    let errors = field.errors;
    let is_touched = field.touched;

    let mut field_clone = field.clone();

    let is_invalid =
        use_memo(move || field.errors.len() != 0 && (field.is_touched() || field.is_dirty()));

    use_effect(move || {
        field_clone.validate();
    });

    rsx! {
        div { class: "my-2",
            label {
                class: "input w-full",
                class: if *is_invalid.read() { "input-error" },
                aria_invalid: is_invalid,
                if label.is_some() && !has_icon {
                    span { class: "label", {label.clone().unwrap()} }
                }
                if has_icon {
                    {icon.unwrap()}
                }
                input {
                    onblur: field_attributes.onblur,
                    oninput: field_attributes.oninput,
                    placeholder: if has_icon && label.is_some() { label.unwrap() },
                    ..field_attributes.into_input_attributes(),
                    ..attributes,
                }
            }
            if is_invalid() && is_touched() {
                ul { class: "text-error", role: "alert",
                    {errors.iter().map(|error| rsx! {
                        li { "{error}" }
                    })}
                }
            }
        }
    }
}
