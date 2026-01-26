use crate::components::contexts::AuthState;
use crate::components::ui::card::{Card, CardActions, CardBody, CardTitle};
use crate::components::ui::form::input::Input;
use crate::components::ui::form::submit_button::SubmitButton;
use crate::{ICON, Route};
use api::routes::users::{EMAIL_REGEX, sign_up};
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{LdKey, LdMail};
use form_hooks::use_form::{use_form, use_on_submit};
use form_hooks::use_form_field::use_form_field;
use form_hooks::validators;
use regex::Regex;

#[derive(Clone, serde::Deserialize)]
struct SignupFormData {
    email: String,
    password: String,
    first_name: String,
    last_name: String,
}

#[component]
pub fn SignupView() -> Element {
    let nav = navigator();
    let mut form_errors = use_signal(Vec::<String>::new);
    let mut sign_up_action = use_action(move |form_data: SignupFormData| async move {
        sign_up(
            form_data.email,
            form_data.password,
            form_data.first_name,
            form_data.last_name,
        )
        .await
    });

    let auth_state = use_context::<AuthState>();

    // If already logged in, redirect to home
    if auth_state.user.read().is_some() {
        let nav = navigator();
        nav.push(Route::Home {});
    }

    let mut form_state = use_form();
    let email = use_form_field("email", String::new())
        .with_validator(validators::required("Email is required!"))
        .with_validator(validators::pattern(
            Regex::new(EMAIL_REGEX)?,
            "Email must be a valid email",
        ));
    let first_name = use_form_field("first_name", String::new())
        .with_validator(validators::required("First name is required!"));
    let last_name = use_form_field("last_name", String::new())
        .with_validator(validators::required("Last name is required!"));
    let password = use_form_field("password", String::new())
        .with_validator(validators::required("Password is required!"));

    form_state.register_field(&email);
    form_state.register_field(&first_name);
    form_state.register_field(&last_name);
    form_state.register_field(&password);
    form_state.revalidate();

    let onsubmit = use_on_submit(&form_state, move |form_state| async move {
        form_errors.clear();
        let form_data: SignupFormData = form_state.parsed_values().unwrap();
        sign_up_action.call(form_data).await;
        match sign_up_action.value() {
            Some(Ok(_)) => {
                nav.push(Route::LoginPage {});
            }
            Some(Err(error)) => {
                form_errors.push(error.to_string());
            }
            None => {
                warn!("Error signing up user. API call did not complete")
            }
        }
    });

    rsx! {
        div { class: "flex flex-col justify-center items-center w-full h-[90vh]",
            div { class: "w-full lg:w-1/2",
                Card {
                    CardBody { class: "items-center text-center",
                        img { src: ICON, class: "aspect-square w-20" }
                        CardTitle { class: "lg:mb-8", "Sign Up" }

                        form { onsubmit, class: "w-full text-left",
                            if form_errors.len() > 0 {
                                div {
                                    class: "alert alert-error mb-4",
                                    role: "alert",
                                    ul {
                                        for error in form_errors.read().iter() {
                                            li { key: "{error}", "{error}" }
                                        }
                                    }
                                }
                            }
                            div { class: "grid grid-cols-1 md:grid-cols-2 md:gap-2",
                                Input {
                                    field: first_name,
                                    label: "First Name",
                                    r#type: "text",
                                }
                                Input {
                                    field: last_name,
                                    label: "Last Name",
                                    r#type: "text",
                                }
                            }
                            Input {
                                field: email,
                                label: "Email",
                                r#type: "email",
                                icon: {
                                    rsx! {
                                        Icon { icon: LdMail }
                                    }
                                },
                            }
                            Input {
                                field: password,
                                label: "Password",
                                r#type: "password",
                                icon: {
                                    rsx! {
                                        Icon { icon: LdKey }
                                    }
                                },
                            }
                            CardActions {
                                SubmitButton {
                                    form: form_state.clone(),
                                    label: "Create Account",
                                    submitting_label: "Creating account...",
                                }
                                p {
                                    "Already have an account? "
                                    Link {
                                        to: Route::LoginPage {},
                                        class: "link",
                                        "Log In"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
