use dioxus::prelude::*;

use form_hooks::use_form_field::{AnyField, FieldValue, FormField};

#[allow(dead_code)]
#[derive(Clone, PartialEq, Default)]
pub enum CheckboxVariant {
    #[default]
    Default,
    Switch,
}

impl CheckboxVariant {
    #[inline]
    #[allow(dead_code)]
    fn class(&self) -> &'static str {
        match self {
            CheckboxVariant::Default => "checkbox",
            CheckboxVariant::Switch => "toggle",
        }
    }
}
#[component]
pub fn Checkbox<T: FieldValue>(
    label: Option<String>,
    field: FormField<T>,
    #[props(default)] variant: CheckboxVariant,
    #[props(extends=GlobalAttributes, extends=input)] attributes: Vec<Attribute>,
) -> Element {
    let checkbox_class = variant.class();

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
        div { class: "my-2",
            label { class: "flex items-center",
                input {
                    class: "mr-2 {checkbox_class}",
                    aria_invalid: is_invalid,
                    onblur: field_attributes.onblur,
                    oninput: field_attributes.oninput,
                    r#type: "checkbox",
                    ..field_attributes.into_input_attributes(),
                    ..attributes,
                }
                {label}
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
}
