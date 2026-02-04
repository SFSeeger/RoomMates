use dioxus::prelude::*;
use form_hooks::use_form::use_form;
use form_hooks::use_form_field::use_form_field;
use form_hooks::validators;

#[component]
pub fn NewGroup() -> Element {
    let mut form_state = use_form();
    let group_name = use_form_field("Group name", String::new())
        .with_validator(validators::required("Group name is required!"));

    form_state.register_field(&group_name);

    rsx! {
        div {}
    }
}
