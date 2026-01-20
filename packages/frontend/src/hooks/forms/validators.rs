use crate::hooks::forms::use_form_field::FieldValue;
use dioxus::core::{Attribute, AttributeValue, IntoAttributeValue};
use std::rc::Rc;
// Based on dioxus-forms by ap-1 (https://github.com/ap-1/dioxus-forms/)

// region Definitions
pub trait ValueRef {
    type Value;
    fn as_ref(&self) -> Option<&Self::Value>;
}

impl ValueRef for i32 {
    type Value = i32;

    fn as_ref(&self) -> Option<&Self::Value> {
        Some(self)
    }
}

impl ValueRef for Option<i32> {
    type Value = i32;

    fn as_ref(&self) -> Option<&Self::Value> {
        self.as_ref()
    }
}

impl ValueRef for String {
    type Value = String;

    fn as_ref(&self) -> Option<&Self::Value> {
        Some(self)
    }
}

impl ValueRef for Option<String> {
    type Value = String;

    fn as_ref(&self) -> Option<&Self::Value> {
        self.as_ref()
    }
}

pub type ValidatorFunc<T> = Rc<dyn Fn(&T) -> Result<(), String>>;

#[derive(Clone, PartialEq)]
struct ValidatorMeta {
    attribute: &'static str,
    attribute_value: AttributeValue,
}

#[derive(Clone)]
pub struct Validator<T> {
    validator_func: ValidatorFunc<T>,
    meta: Option<ValidatorMeta>,
}

// endregion

impl<T> Validator<T> {
    pub fn into_attribute(self) -> Option<Attribute> {
        self.meta.map(|meta| Attribute {
            name: meta.attribute,
            value: meta.attribute_value,
            namespace: None,
            volatile: false,
        })
    }

    pub fn validate(&self, value: &T) -> Result<(), String> {
        (self.validator_func)(value)
    }
}

#[allow(dead_code)]
pub fn required<T: FieldValue + Default>(error_message: &str) -> Validator<T> {
    let message = error_message.to_string();

    let meta = ValidatorMeta {
        attribute: "required",
        attribute_value: true.into_value(),
    };

    Validator {
        validator_func: Rc::new(move |value: &T| {
            if value == &T::default() {
                Err(message.clone())
            } else {
                Ok(())
            }
        }),
        meta: Some(meta),
    }
}

#[allow(dead_code)]
pub fn required_with_default_message<T: FieldValue + Default>() -> Validator<T> {
    required("Value is required")
}

#[allow(dead_code)]
pub fn min_value<T: ValueRef<Value = i32> + Clone + 'static>(
    min: i32,
    error_message: &str,
) -> Validator<T> {
    let message = error_message.to_string();
    let meta = ValidatorMeta {
        attribute: "min",
        attribute_value: min.into_value(),
    };
    Validator {
        validator_func: Rc::new(move |value: &T| match value.as_ref() {
            Some(v) => {
                if *v < min {
                    Err(message.clone())
                } else {
                    Ok(())
                }
            }
            _ => Ok(()),
        }),
        meta: Some(meta),
    }
}

#[allow(dead_code)]
pub fn max_value<T: ValueRef<Value = i32> + Clone + 'static>(
    max: i32,
    error_message: &str,
) -> Validator<T> {
    let message = error_message.to_string();
    let meta = ValidatorMeta {
        attribute: "max",
        attribute_value: max.into_value(),
    };
    Validator {
        validator_func: Rc::new(move |value: &T| match value.as_ref() {
            Some(v) => {
                if *v > max {
                    Err(message.clone())
                } else {
                    Ok(())
                }
            }
            _ => Ok(()),
        }),
        meta: Some(meta),
    }
}

#[allow(dead_code)]
pub fn min_length<T: ValueRef<Value = String> + Clone + 'static>(
    min: usize,
    error_message: &str,
) -> Validator<T> {
    let message = error_message.to_string();
    let meta = ValidatorMeta {
        attribute: "minLength",
        attribute_value: min.into_value(),
    };
    Validator {
        validator_func: Rc::new(move |value: &T| match value.as_ref() {
            Some(v) => {
                if v.len() < min {
                    Err(message.clone())
                } else {
                    Ok(())
                }
            }
            _ => Ok(()),
        }),
        meta: Some(meta),
    }
}

#[allow(dead_code)]
pub fn max_length<T: ValueRef<Value = String> + Clone + 'static>(
    max: usize,
    error_message: &str,
) -> Validator<T> {
    let message = error_message.to_string();
    let meta = ValidatorMeta {
        attribute: "maxLength",
        attribute_value: max.into_value(),
    };
    Validator {
        validator_func: Rc::new(move |value: &T| match value.as_ref() {
            Some(v) => {
                if v.len() > max {
                    Err(message.clone())
                } else {
                    Ok(())
                }
            }
            _ => Ok(()),
        }),
        meta: Some(meta),
    }
}

/// Validates a field's value based on a regex pattern.
///
/// # Arguments
///
/// * `pattern`: [regex::Regex] pattern to validate against
/// * `error_message`: Error message to display when the pattern does not match
///
/// returns: [Validator<T>]
///
/// # Examples
///
/// ```
/// let name = use_form_field("name", String::new())
///     .with_validator(validators::pattern(
///         regex::Regex::new("[a-zA-Z0-9]+").unwrap(),
///         "Only alphanumeric characters allowed!")
///     );
/// ```
#[allow(dead_code)]
pub fn pattern<T: ValueRef<Value = String> + Clone + 'static>(
    pattern: regex::Regex,
    error_message: &str,
) -> Validator<T> {
    let message = error_message.to_string();
    Validator {
        validator_func: Rc::new(move |value: &T| match value.as_ref() {
            Some(v) => {
                if !pattern.is_match(v) {
                    Err(message.clone())
                } else {
                    Ok(())
                }
            }
            _ => Ok(()),
        }),
        meta: None,
    }
}
