use dioxus::core::AttributeValue;
use dioxus::prelude::*;
use form_hooks::use_form_field::{AnyField, FieldValue, FormField};

pub trait EnumSelect {
    #[allow(dead_code)] //TODO: Remove after usage
    fn select_options() -> Vec<(String, &'static str)>;
}

/// Generic Select Component with validation
/// automatically creates options
#[component]
pub fn Select<T: FieldValue + EnumSelect>(
    label: Option<String>,
    field: FormField<T>,
    #[props(extends=GlobalAttributes, extends=select)] attributes: Vec<Attribute>,
) -> Element {
    let field_clone = field.clone();
    let field_attributes = field.field_attributes();

    let options = T::select_options();
    let is_invalid = use_memo(move || {
        field_clone.errors.len() != 0 && (field_clone.is_touched() || field_clone.is_dirty())
    });

    let mut field_clone = field.clone();
    use_effect(move || {
        field_clone.validate();
    });

    rsx! {
        fieldset { class: "my-2 fieldset",
            if label.is_some() {
                legend { class: "fieldset_legend", {label.unwrap()} }
            }
            select {
                class: "select",
                class: if is_invalid() { "input-error" },
                aria_invalid: is_invalid,
                onblur: field_attributes.onblur,
                onchange: field_attributes.oninput,
                ..field_attributes.into_input_attributes(),
                ..attributes,
                {options.iter().map(|(value, label)| rsx! {
                    option {
                        key: "{value}",
                        selected: field.value.read().to_input_value() == *value,
                        value: "{value}",
                        // Disable a blank input if the field is required
                        disabled: value.is_empty()
                            && field_attributes
                                .additional_attributes
                                .iter()
                                .any(|a| {
                                    if a.name == "required" && let AttributeValue::Bool(value) = a.value {
                                        value
                                    } else {
                                        false
                                    }
                                }),
                        aria_label: "{label}",
                        "{label}"
                    }
                })}
            }
            if is_invalid() && field.is_touched() {
                ul { class: "text-error", role: "alert",
                    {field.errors.iter().map(|error| rsx! {
                        li { "{error}" }
                    })}
                }
            }
        }
    }
}
