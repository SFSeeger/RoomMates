use crate::components::ui::button;
use crate::components::ui::card::{Card, CardActions, CardTitle};
use crate::components::ui::fieldset::Fieldset;
use crate::components::ui::form::input::Input;
use crate::components::ui::form::submit_button::SubmitButton;
use crate::components::ui::toaster::{ToastOptions, use_toaster};
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
                    img { src: format!("https://api.dicebear.com/9.x/bottts/avif?seed={}", user().id) }
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
        div { class: "divider divider-primary" }
        Card {

            CardTitle { "Profile Information" }
            ListInfoDisplay { real_user, onupdate }
            PasswordDisplay {}
        }
    }
}

#[component]
pub fn ListInfoDisplay(real_user: UserInfo, onupdate: EventHandler<UserInfo>) -> Element {
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
    let mut toaster = use_toaster();

    let onsubmit = use_on_submit(&form_state, move |mut form| async move {
        let form_data: UpdateFormData = form.parsed_values().unwrap();
        update_action.call(form_data).await;

        match update_action.value() {
            Some(Ok(new_user)) => {
                onupdate.call(new_user.read().clone());
                toaster.success("Successfully changed user info!", ToastOptions::new());
                form.mark_clean();
            }
            Some(Err(_)) => {
                toaster.error("Failed to update user info!", ToastOptions::new());
            }
            None => {
                warn!("Error changing user info")
            }
        }
    });

    rsx! {
        form { onsubmit,
            Card {
                " Username: {real_user.first_name} {real_user.last_name}"
                Fieldset {
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

                Fieldset {
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
pub fn PasswordDisplay() -> Element {
    let mut password_state = use_form();

    let mut password_change = use_action(change_password);

    let password = use_form_field("password", String::new())
        .with_validator(validators::required("Enter new password"));

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
    let mut toaster = use_toaster();

    let onsubmit = use_on_submit(&password_state, move |mut password_state| async move {
        let password_value = password.value.peek().clone();

        password_change.call(password_value).await;
        match password_change.value() {
            Some(Ok(_)) => {
                toaster.success("Changed password successfully!", ToastOptions::new());
                password_state.reset();
            }
            Some(Err(_)) => {
                toaster.error("Failed to change password!", ToastOptions::new());
            }
            None => {
                warn!("Request did not finish!");
            }
        }
    });

    rsx! {
        Card {
            form { onsubmit,
                Fieldset {
                    p { "Set new password!" }
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
