use crate::components::ui::button;
use crate::components::ui::card::{Card, CardActions, CardBody, CardTitle};
use crate::components::ui::fieldset::Fieldset;
use crate::components::ui::form::input::Input;
use crate::components::ui::form::submit_button::SubmitButton;
//use crate::components::ui::toaster::{Toast, ToastVariant, ToasterState};
use crate::{Route, components::ui::button::Button};
use api::routes::users::EMAIL_REGEX;
use api::routes::users::get_me;
use api::routes::users::{UserInfo, change_password, change_user_info};
use dioxus::prelude::*;
use form_hooks::use_form::{use_form, use_on_submit};
use form_hooks::use_form_field::use_form_field;
use form_hooks::validators;
use regex::Regex;
use std::rc::Rc;

//use dioxus_free_icons::Icon;
//use dioxus_free_icons::icons::ld_icons::LdCircleHelp;

#[derive(Clone, serde::Deserialize)]
struct UpdateFormData {
    first_name: String,
    last_name: String,
    email: String,
}

#[component]
pub fn Profile() -> Element {
    let mut user = use_loader(move || async move { get_me().await })?;
    let real_user = user.read().clone();

    let onupdate = move |new: UserInfo| {
        let mut write_user = user.write();
        write_user.email = new.email;
        write_user.first_name = new.first_name;
        write_user.last_name = new.last_name;
    };
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
                        List_Info_Display { real_user, onupdate }
                        Password_Display {}

                    }
                }
            }
        }
    }
}

#[component]
pub fn List_Info_Display(real_user: UserInfo, onupdate: EventHandler<UserInfo>) -> Element {
    let mut form_state = use_form();
    let mut update_action = use_action(move |form_data: UpdateFormData| async move {
        change_user_info(form_data.first_name, form_data.last_name, form_data.email).await
    });

    let first_name = use_form_field("first_name", real_user.first_name.clone());
    let last_name = use_form_field("last_name", real_user.last_name.clone());
    let email = use_form_field("email", real_user.email.clone()).with_validator(
        validators::pattern(Regex::new(EMAIL_REGEX)?, "Email must be a valid email"),
    );

    form_state.register_field(&first_name);
    form_state.register_field(&last_name);
    form_state.register_field(&email);

    form_state.revalidate();

    let onsubmit = use_on_submit(&form_state, move |form| async move {
        let form_data: UpdateFormData = form.parsed_values().unwrap();
        update_action.call(form_data).await;

        match update_action.value() {
            Some(Ok(new_user)) => {
                onupdate.call(new_user.read().clone());
            }
            Some(Err(_)) => {}
            None => {
                warn!("Error signing up user. API call did not complete")
            }
        }
    });

    rsx! {
        form { onsubmit,
            Card {
                CardBody {
                    " Username: {real_user.first_name} {real_user.last_name}"
                    div {
                        fieldset { class: "fieldset",
                            Input {
                                label: "Edit first name",
                                field: first_name,
                                r#type: "text",
                            }
                            Input {
                                label: "Edit last name",
                                field: last_name,
                                r#type: "text",
                            }
                        }
                    }
                }

                CardBody {
                    p { "Email: {real_user.email}" }
                    Input { label: "Set Email", field: email, r#type: "email" }
                }
                CardActions {
                    SubmitButton { form: form_state.clone(), label: "Confirm New Info" }
                }
            }
        }

    }
}

#[component]
pub fn Password_Display() -> Element {
    let mut password_state = use_form();

    let mut password_change = use_action(change_password);

    let password = use_form_field("password", String::new())
        .with_validator(validators::required("enter password"));

    let password_repeat_func = Rc::new(move |value: &String| {
        let password_value = password.value.peek();
        if *value != *password_value || value.is_empty() {
            Err("Passwords do not match!".to_string())
        } else {
            Ok(())
        }
    });

    let password_repeat = use_form_field("password", String::new())
        .with_validator(validators::custom(password_repeat_func));

    password_state.register_field(&password);
    password_state.register_field(&password_repeat);

    password_state.revalidate();

    let onsubmit = use_on_submit(&password_state, move |_| async move {
        let password_value = password.value.peek().clone();
        //let toaster = use_context::<ToasterState>();
        //let mut toaster_clone = toaster.clone();

        password_change.call(password_value);
        match password_change.value() {
            Some(Ok(_)) => {
                /*toaster_clone.toast(Toast::new(
                    "Changed Password successfully!".to_owned(),
                    None,
                    true,
                    ToastVariant::Success,
                ));*/
            }
            Some(Err(_error)) => {
                /*toaster_clone.toast(Toast::new(
                    "Failed to change Password".to_owned(),
                    Some(rsx! {
                        span { "{error.to_string()}" }
                    }),
                    true,
                    ToastVariant::Error,
                ));*/
            }
            None => {
                warn!("Request did not finish!");
            }
        }
    });

    rsx! {
        Card {
            form { onsubmit,
                CardBody {
                    p { "Set new password!" }
                    Fieldset {
                        Input {
                            field: password,
                            label: "Type New Password",
                            r#type: "password",
                        }
                        Input {
                            field: password_repeat,
                            label: "Repeat New Password",
                            r#type: "password",
                        }
                    }
                }
                CardActions {
                    SubmitButton {
                        form: password_state.clone(),
                        label: "Change Password",
                    }
                }
            }
        }
    }
}
