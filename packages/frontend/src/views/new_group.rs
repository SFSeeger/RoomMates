use dioxus::prelude::*;

/*use crate::components::ui::button::{Button, ButtonShape, ButtonVariant};
use crate::components::ui::card::{Card, CardActions, CardBody, CardTitle};
use crate::components::ui::form::input::Input;
use crate::components::ui::form::submit_button::SubmitButton;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::LdUsers;
use form_hooks::use_form::use_form;
use form_hooks::use_form::use_on_submit;
use form_hooks::use_form_field::use_form_field;
use form_hooks::validators;*/

#[component]
pub fn NewGroup() -> Element {
    /*let mut form_state_group_name = use_form();
    let group_name = use_form_field("Group name", String::new())
        .with_validator(validators::required("Group name is required!"));

    form_state_group_name.register_field(&group_name);

    let onsubmitgroupname = use_on_submit(&form_state_group_name, move |form| {

    });
    rsx! {
        div {
            h1 { class: "relative text-2xl font-bold text-center", "Add a new group" }
            div {
                Card {
                    CardBody {
                        CardTitle { class: "flex items-center grow justify-center",
                            div { class: "flex justify-center w-full",
                                div { class: "w-3/4",
                                    form { onsubmit: onsubmitgroupname,
                                        Input {
                                            field: group_name_field,
                                            label: "{group.read().name}",
                                            r#type: "text",
                                            class: "h-12 text-lg px-4",
                                            icon: {
                                                rsx! {
                                                    Icon { icon: LdUsers }
                                                }
                                            },
                                        }
                                    }

                                }
                            }
                        }
                    }
                }
            }
        }
    }*/
    rsx! {
        div {}
    }
}
