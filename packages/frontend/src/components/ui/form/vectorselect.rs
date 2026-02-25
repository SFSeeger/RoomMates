use dioxus::core::AttributeValue;
use dioxus::prelude::*;
use form_hooks::prelude::*;

/// Generic Select Component with validation
/// automatically creates options
#[component]
pub fn VectorSelect<T: FieldValue + Default>(
    label: Option<String>,
    field: FormField<T>,
    options: Vec<(T, String)>,
    #[props(extends=GlobalAttributes, extends=select)] attributes: Vec<Attribute>,
) -> Element {
    let field_clone = field.clone();
    let field_attributes = field.field_attributes();

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
                class: "select w-full",
                class: if is_invalid() { "input-error" },
                aria_invalid: is_invalid,
                onblur: field_attributes.onblur,
                onchange: field_attributes.oninput,
                ..field_attributes.into_input_attributes(),
                ..attributes,
                {options.iter().map(|(value, label)| rsx! {
                    option {
                        key: "{value.to_input_value()}",
                        selected: field.value.read().to_input_value() == (*value).to_input_value(),
                        value: value.to_input_value(),
                        // Disable a blank input if the field is required
                        disabled: value == &T::default()
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
                ul { class: "text-error text-sm", role: "alert",
                    {field.errors.iter().map(|error| rsx! {
                        li { "{error}" }
                    })}
                }
            }
        }
    }
}
