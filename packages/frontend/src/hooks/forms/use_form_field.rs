#![allow(dead_code)] // TODO: Remove this when UI compoents are in use

use crate::hooks::forms::validators::Validator;
use dioxus::core::IntoAttributeValue;
use dioxus::prelude::*;
use serde::Serialize;
use serde_json::Value;
// Based on dioxus-forms by ap-1 (https://github.com/ap-1/dioxus-forms/)

// region FieldValue Declaration

/// Representation of a form value.
/// Currently supports the following types out of the box:
/// - `String`
/// - `i32`
/// - `bool`
/// - `Option<T: FieldValue>`
/// # Examples
/// To use an enum in a form, the enum needs to implement [FieldValue]
/// ```
/// #[derive(serde::Serialize)]
/// enum MyEnum {
///     Option1,
///     Option2
/// }
/// impl FieldValue for MyEnum {
///     fn to_input_value(&self) -> String {
///         match &self {
///             MyEnum::Option1 => "1".to_string(),
///             MyEnum::Option2 => "2".to_string(),
///         }
///     }
///
///     fn from_input_value(value: &str) -> Result<Self, String> {
///         match value {
///             "1" => Ok(MyEnum::Option1),
///             "2" => Ok(MyEnum::Option2),
///             _ => Err(format!("Unknown field value: {}", value)),
///         }
///     }
/// }
/// ```
pub trait FieldValue: Clone + PartialEq + Serialize + 'static {
    fn to_input_value(&self) -> String;

    fn from_input_value(value: &str) -> Result<Self, String>;
}

impl FieldValue for String {
    fn to_input_value(&self) -> String {
        self.clone()
    }

    fn from_input_value(value: &str) -> Result<Self, String> {
        Ok(value.to_string())
    }
}

impl FieldValue for i32 {
    fn to_input_value(&self) -> String {
        std::string::ToString::to_string(&self)
    }

    fn from_input_value(value: &str) -> Result<Self, String> {
        value
            .parse::<i32>()
            .map_err(|e| format!("Invalid Number {e}"))
    }
}

impl FieldValue for bool {
    fn to_input_value(&self) -> String {
        if *self {
            return "true".to_string();
        }
        "".to_string()
    }

    fn from_input_value(value: &str) -> Result<Self, String> {
        match value {
            "true" => Ok(true),
            &_ => Ok(false),
        }
    }
}

impl<T: FieldValue> FieldValue for Option<T> {
    fn to_input_value(&self) -> String {
        match self {
            None => "".to_string(),
            Some(value) => value.to_input_value(),
        }
    }

    fn from_input_value(value: &str) -> Result<Self, String> {
        match value {
            "" => Ok(None),
            value => Ok(Some(T::from_input_value(value)?)),
        }
    }
}

// endregion

/// Representation of a form field.
/// See [use_form_field] for example usages
#[derive(Clone)]
pub struct FormField<T: FieldValue> {
    /// Current value of the field
    pub value: Signal<T>,
    original_value: T,
    /// Name of the field. Used in parsing
    pub name: String,
    /// Vector of field errors
    pub errors: Signal<Vec<String>>,
    /// True if the field has been focused before
    pub touched: Signal<bool>,
    validators: Vec<Validator<T>>,
}

impl<T: FieldValue> FormField<T> {
    pub fn new(name: &str, initial_value: T) -> Self {
        FormField {
            value: Signal::new(initial_value.clone()),
            original_value: initial_value,
            name: name.to_string(),
            errors: Signal::new(Vec::new()),
            touched: Signal::new(false),
            validators: Vec::new(),
        }
    }

    /// Adds a [Validator] to the field
    /// # Examples
    /// ```
    /// let name = use_form_field("name", "Bob".to_string())
    ///     .with_validator(validators::required("Name is required")
    ///     .with_validator(validators::min_length(2, "Name needs to be at least 2 Characters long");
    /// ```
    pub fn with_validator(mut self, validator: Validator<T>) -> Self {
        self.validators.push(validator);
        self
    }

    /// Validates the field based on registered validators
    pub fn validate(&mut self) -> bool {
        self.errors.set(Vec::new());
        for validator in &self.validators {
            let validation_result = validator.validate(&self.value.read());
            if let Err(error) = validation_result {
                self.errors.push(error)
            }
        }
        self.errors.len() == 0
    }

    /// Returns true if the field's value has been changed
    pub fn is_dirty(&self) -> bool {
        self.value.read().clone() != self.original_value
    }

    /// Sets the current value as the new original value. Can be used for forms that perform a "save"
    #[allow(dead_code)]
    pub fn mark_clean(&mut self) {
        self.original_value = self.value.read().clone();
    }

    /// Generates a [FieldAttributes] struct for the form field, which provides:
    /// - The current value of the field as a string (`value`)
    /// - The field's name (`name`)
    /// - Event handlers for input changes (`oninput`) and blur events (`onblur`)
    /// - Any additional attributes required by validators (e.g. `min`, `max`, `required`)
    ///
    /// This method is used to easily bind form field state and validation to Dioxus input components.
    /// The returned [FieldAttributes] can be converted into a vector of Dioxus [Attributes](dioxus_core::Attribute)
    /// for different input types (e.g., text, checkbox) using methods like [into_input_attributes()] or [into_checkbox_attributes()].
    ///
    /// ## Example
    /// ```
    /// # let name = use_form_field("name", "Bob".to_string())
    /// let name_attributes = name.field_attributes;
    /// rsx! {
    ///     input {
    ///         r#type: "text",
    ///         oninput: name_attributes.oninput,
    ///         onblur: name_attributes.onblur,
    ///         ..name_attributes.into_input_attributes()
    ///     }
    /// }
    /// ```
    pub fn field_attributes(&self) -> FieldAttributes {
        let value = self.value.read().clone().to_input_value();
        let name = self.name.clone();

        let mut field_for_oninput = self.clone();
        let oninput =
            EventHandler::new(
                move |event: FormEvent| match T::from_input_value(&event.value()) {
                    Ok(value) => {
                        field_for_oninput.value.set(value);
                        field_for_oninput.validate();
                    }
                    Err(error) => field_for_oninput.add_error(error),
                },
            );

        let mut field_for_onfocus = self.clone();
        let onblur = EventHandler::new(move |_event: FocusEvent| {
            field_for_onfocus.touched.set(true);
            field_for_onfocus.validate();
        });

        // Collect additional attributes from validators
        let mut additional_attributes = Vec::with_capacity(self.validators.len());

        for validator in &self.validators {
            let additional_attribute = validator.clone().into_attribute();
            if let Some(attribute) = additional_attribute {
                additional_attributes.push(attribute);
            }
        }

        FieldAttributes {
            value,
            name,
            oninput,
            onblur,
            additional_attributes,
        }
    }
}

pub struct FieldAttributes {
    pub value: String,
    pub name: String,
    pub oninput: EventHandler<FormEvent>,
    pub onblur: EventHandler<FocusEvent>,
    pub additional_attributes: Vec<Attribute>,
}

impl FieldAttributes {
    fn build_attributes(mut self, value_attr_name: &'static str) -> Vec<Attribute> {
        let mut attributes = vec![
            Attribute {
                name: value_attr_name,
                value: self.value.into_value(),
                namespace: None,
                volatile: false,
            },
            Attribute {
                name: "name",
                value: self.name.into_value(),
                namespace: None,
                volatile: false,
            },
        ];
        attributes.append(&mut self.additional_attributes);
        attributes
    }

    /// Returns the field attributes for an generic input element
    pub fn into_input_attributes(self) -> Vec<Attribute> {
        self.build_attributes("value")
    }

    /// Returns the field attributes for an input element of type checkbox
    pub fn into_checkbox_attributes(self) -> Vec<Attribute> {
        self.build_attributes("checked")
    }
}

pub trait AnyField {
    fn name(&self) -> &str;
    fn value_json(&self) -> Result<Value, serde_json::Error>;
    fn is_touched(&self) -> bool;
    fn add_error(&mut self, error: String);
    fn reset(&mut self);
}

impl<T> AnyField for FormField<T>
where
    T: FieldValue,
{
    fn name(&self) -> &str {
        &self.name
    }

    fn value_json(&self) -> Result<Value, serde_json::Error> {
        serde_json::to_value(self.value.read().cloned())
    }
    fn is_touched(&self) -> bool {
        *self.touched.read()
    }
    fn add_error(&mut self, error: String) {
        self.errors.push(error)
    }

    fn reset(&mut self) {
        self.value.set(self.original_value.clone());
        self.errors.set(Vec::new());
        self.touched.set(false);
    }
}

/// Initializes and returns a [FormField<T>] for managing the state, validation, and event handling of a single form field.
///
/// # Arguments
///
/// * `name`: The field's name as a string.
/// * `initial_value`: The initial value for the field (must implement [FieldValue]).
///
/// returns: [FormField<T>]
///
/// # Examples
///
/// ```
/// let name = use_form_field("name", String::new())
///     .with_validator(validators::required_with_default_message());
/// let age = use_form_field("age", Some(30))
///     .with_validator(validators::required("Age is required"))
///     .with_validator(validators::min(18, "No minors allowed)";
/// ```
pub fn use_form_field<T: FieldValue>(name: &str, initial_value: T) -> FormField<T> {
    use_signal(|| FormField::new(name, initial_value))()
}
