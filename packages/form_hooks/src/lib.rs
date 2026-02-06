pub mod prelude;
pub mod use_form;
pub mod use_form_field;
pub mod validators;

/// Utility Trait to enable automatic select generation
pub trait EnumSelect {
    /// Maps each enum variant's input_value to a label
    fn select_options() -> Vec<(String, &'static str)>;
}

/// Trait which allows customizing the label when nothing (`None`) is selected
pub trait EnumSelectDefault {
    fn default_label() -> &'static str;
}

impl<T> EnumSelect for Option<T>
where
    T: EnumSelect + EnumSelectDefault,
{
    fn select_options() -> Vec<(String, &'static str)> {
        let mut options = vec![("".to_string(), T::default_label())];
        options.append(&mut T::select_options());
        options
    }
}
