#![allow(dead_code)] // TODO: Remove this when UI compoents are in use

use crate::hooks::forms::use_form_field::{AnyField, FieldValue, FormField};
use dioxus::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

// Based on dioxus-forms by ap-1 (https://github.com/ap-1/dioxus-forms/)

type FieldErrorCallbacks = Signal<Vec<Rc<dyn Fn() -> Vec<String>>>>;

#[derive(Clone)]
pub struct FormState {
    /// True if the form has changed values
    pub is_dirty: Signal<bool>,
    /// True if the form is currently being submitted. Useful for async submits
    pub is_submitting: Signal<bool>,
    /// True if the form contains any errors
    pub has_errors: Signal<bool>,
    /// True if any of the fields have been selected
    pub is_touched: Signal<bool>,
    dirty_checkers: Signal<Vec<Rc<dyn Fn() -> bool>>>,
    field_error_checkers: FieldErrorCallbacks,
    fields: Signal<Vec<Rc<RefCell<dyn AnyField>>>>,
}

impl FormState {
    pub fn new() -> Self {
        FormState {
            is_dirty: Signal::new(false),
            is_submitting: Signal::new(false),
            is_touched: Signal::new(false),
            has_errors: Signal::new(false),
            dirty_checkers: Signal::new(Vec::new()),
            field_error_checkers: Signal::new(Vec::new()),
            fields: Signal::new(Vec::new()),
        }
    }

    /// Registers a [FormField] into the form. Allows for updating errors, dirty and touched
    /// # Examples
    /// ```
    /// let mut form_state = use_form();
    /// let name = use_form_field("name", "Bob".to_string())
    /// form_state.register(&name);
    pub fn register_field<T: FieldValue>(&mut self, field: &FormField<T>) {
        let field_clone = field.clone();
        self.dirty_checkers
            .push(Rc::new(move || field_clone.is_dirty()));

        let field_clone = field.clone();
        let field_error_checker = Rc::new(move || field_clone.errors.read().clone());
        self.field_error_checkers.push(field_error_checker);

        let field_clone = field.clone();
        self.fields.push(Rc::new(RefCell::new(field_clone)));
    }

    /// Checks all registered fields for dirty status
    pub fn check_dirty(&mut self) {
        let is_dirty = self.dirty_checkers.read().iter().any(|checker| checker());
        self.is_dirty.set(is_dirty);
    }

    /// Checks all registered fields for touched status
    pub fn check_touched(&mut self) {
        let touched = self
            .fields
            .read()
            .iter()
            .any(|field| field.borrow().is_touched());
        self.is_touched.set(touched)
    }

    /// Returns true, if any registered field has an error. Does not alter has_errors
    pub fn has_errors(&self) -> bool {
        if self.field_error_checkers.len() == 0 {
            return false;
        }

        self.field_error_checkers
            .read()
            .iter()
            .any(|checker| !checker().is_empty())
    }

    /// Checks all registered fields for errors and updates has_errors accordingly
    pub fn check_errors(&mut self) {
        self.has_errors.set(self.has_errors())
    }

    /// Checks all fields for updates
    pub fn revalidate(&mut self) {
        self.check_dirty();
        self.check_touched();
        self.check_errors();
    }

    /// Resets all fields and the form
    pub fn reset(&mut self) {
        self.is_touched.set(false);
        self.is_dirty.set(false);
        self.has_errors.set(false);
        for field in self.fields.read().iter() {
            field.borrow_mut().reset();
        }
    }

    /// Parses the forms values into a struct. The struct needs to implement [serde::Deserialize]
    /// and all struct fields must have a form field registered with the same name.
    /// Since [FieldValue] needs to implement [serde::Serialize] by default, any [FieldValue] can be used
    ///
    /// # Examples
    /// ```
    /// #[derive(serde::Deserialize)]
    /// struct FormData {
    ///     name: String,
    /// }
    /// let mut form_state = use_form();
    /// let name = use_form_field("name", "Bob".to_string())
    /// form_state.register(&name);
    ///
    /// // Parse form data into the struct
    /// let form_data: FormData = form_state.parsed_values().unwrap();
    /// assert_eq!(FormData {name: "Bob".to_string()}, form_data)
    /// ```
    pub fn parsed_values<T>(&self) -> Result<T, serde_json::Error>
    where
        T: serde::de::DeserializeOwned,
    {
        let mut map = serde_json::Map::new();

        for field in self.fields.read().iter() {
            let field = field.borrow();
            let value = field.value_json()?;
            map.insert(field.name().to_string(), value);
        }
        serde_json::from_value(serde_json::Value::Object(map))
    }

    fn start_submitting(&mut self) {
        self.is_submitting.set(true);
    }
    fn stop_submitting(&mut self) {
        self.is_submitting.set(false);
    }
}

/// Hook which returns a FormState.
/// # Examples
/// ```
/// let mut form_state = use_form();
/// ```
pub fn use_form() -> FormState {
    use_signal(FormState::new)()
}

/// Creates an event handler to handle form submission.
/// The event handler returns early, if the form has errors.
/// # Examples
/// ```
/// let mut form_state = use_form();
/// let onsubmit = use_on_submit(&form_state, move |form| async move {
///         // Do something with the data
///     }
/// );
/// rsx! {
///     form { onsubmit }
/// }
/// ```
pub fn use_on_submit<F, Fut>(form_state: &FormState, on_submit: F) -> EventHandler<FormEvent>
where
    F: Fn(FormState) -> Fut + Clone + 'static,
    Fut: Future<Output = ()> + 'static,
{
    let form_state_owned = form_state.clone();

    EventHandler::new(move |event: FormEvent| {
        let mut form_state_clone = form_state_owned.clone();
        let on_submit = on_submit.clone();
        async move {
            event.prevent_default();

            if form_state_clone.has_errors() {
                return;
            }

            form_state_clone.start_submitting();
            on_submit(form_state_clone.clone()).await;
            form_state_clone.stop_submitting();
        }
    })
}
