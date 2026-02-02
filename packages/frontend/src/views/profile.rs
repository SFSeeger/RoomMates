use crate::components::ui::button;
use crate::components::ui::card::{Card, CardBody, CardTitle};
use crate::components::ui::fieldset::Fieldset;
use crate::components::ui::form::input::Input;
use crate::components::ui::form::submit_button::SubmitButton;
use crate::components::ui::list::{List, ListRow};
use crate::{Route, components::ui::button::Button};
use api::routes::users::get_me;
use dioxus::prelude::*;
use form_hooks::use_form::{use_form, use_on_submit};
use form_hooks::use_form_field::use_form_field;
//use dioxus_free_icons::Icon;
//use dioxus_free_icons::icons::ld_icons::LdCircleHelp;

#[component]
pub fn Profile() -> Element {
    rsx! {

        div { class: "flex flex-col items-center gap-4 justify-center h-full",
            div { class: "avatar",
                div { class: "w-24 rounded",
                    img { src: "https://img.daisyui.com/images/profile/demo/yellingcat@192.webp" }
                }
            }
            h1 { class: "text-2xl font-bold text-center", "you reached your profile, success" }
            Link { to: Route::Home {},
                Button {
                    variant: button::ButtonVariant::Primary,
                    ghost: false,
                    shape: button::ButtonShape::Wide,
                    disabled: false,
                    "Homepage"
                }
            }
        }
        div { class: "divider divider-primar" }
        Card {
            CardBody {
                {
                    rsx! {
                        CardTitle { "Profile Information" }
                        List_Info_Display {}

                    }
                }
            }
        }
    }
}

#[component]
pub fn List_Info_Display() -> Element {
    let user = use_server_future(move || async move { get_me().await })?;

    let mut form_state = use_form();

    //let mut update = use_action();

    let first_name = use_form_field("first", String::new());
    let last_name = use_form_field("last", String::new());
    let email = use_form_field("email", String::new());

    let password = use_form_field("password", String::new());
    let password2 = use_form_field("password", String::new());

    form_state.register_field(&email);
    form_state.register_field(&first_name);
    form_state.register_field(&last_name);

    form_state.revalidate();

    let _on_submit = use_on_submit(&form_state, move |_form| async move {
        match &*user.read() {
            Some(Ok(_data)) => {}
            _ => debug!("User Auth Error!"),
        }
    });

    rsx! {
        match &*user.read() {

            Some(Ok(data)) => rsx! {
                form {
                    List { header: "",
                        ListRow {
                            " Username: {data.first_name} {data.last_name}"
                            div {
                                fieldset { class: "fieldset",
                                    legend { class: "fieldset-legend", "Edit first name!" }
                                    input {
                                        class: "input",
                                        placeholder: " {data.first_name}",
                                        r#type: "text",
                                    }
                                    legend { class: "fieldset-legend", "Edit last name!" }
                                    input {
                                        class: "input",
                                        placeholder: " {data.last_name}",
                                        r#type: "text",
                                    }
                                }
                            }
                        }
                        ListRow {
                            p { "Email: {data.email}" }
                            Input { label: "Set Email", field: email, r#type: "email" }
                        }


                        ListRow {
                            p { "Set new password!" }
                            Fieldset { title: "Set new password",
                                Input {
                                    field: password,
                                    label: "Type New Password",
                                    r#type: "email",
                                }
                                Input {
                                    field: password2,
                                    label: "Repeat New Password",
                                    r#type: "email",
                                }
                            }
                        }
                        SubmitButton { form: form_state, label: "Confirm New Info" }
                    }
                }
            },
            Some(Err(err)) => rsx! {
                p { class: "text-red-500", " failed with {err}" }
            },
            None => rsx! {
                p { "cant connect to db" }
            },
        }
    }
}

// div {
//   label { class: "btn", r#for: "my_modal_6", "open modal" }
//   input { class: "modal-toggle", id: "my_modal_6", r#type: "checkbox" }
//   div { class: "modal", role: "dialog",
//      div { class: "modal-box",
//          div { class: "modal-action" }
