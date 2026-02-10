use crate::components::ui::fieldset::Fieldset;
use dioxus::prelude::*;
use form_hooks::use_form_field::{AnyField, FieldValue, FormField};

#[component]
pub fn Textarea<T: FieldValue>(
    placeholder: Option<String>,
    label: Option<String>,
    field: FormField<T>,
    #[props(extends=GlobalAttributes, extends=textarea)] attributes: Vec<Attribute>,
) -> Element {
    let field_attributes = field.field_attributes();
    let errors = field.errors;
    let is_touched = field.touched;

    let mut field_clone = field.clone();

    let is_invalid =
        use_memo(move || field.errors.len() != 0 && (field.is_touched() || field.is_dirty()));

    use_effect(move || {
        field_clone.validate();
    });

    rsx! {
        Fieldset { title: label,
            textarea {
                class: "textarea w-full",
                onblur: field_attributes.onblur,
                oninput: field_attributes.oninput,
                placeholder: if placeholder.is_some() { placeholder.unwrap() },
                ..field_attributes.into_input_attributes(),
                ..attributes,
            }
        }
        if is_invalid() && is_touched() {
            ul { class: "text-error text-sm", role: "alert",
                {errors.iter().map(|error| rsx! {
                    li { "{error}" }
                })}
            }
        }
    }
}
