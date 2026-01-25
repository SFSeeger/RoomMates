use crate::Route;
use crate::components::contexts::AuthState;
use crate::components::ui::card::{Card, CardActions, CardBody, CardTitle};
use crate::components::ui::form::input::Input;
use crate::components::ui::form::submit_button::SubmitButton;
use api::routes::users::{EMAIL_REGEX, get_me, login};
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{LdKey, LdMail};
use form_hooks::use_form::{use_form, use_on_submit};
use form_hooks::use_form_field::use_form_field;
use form_hooks::validators;
use regex::Regex;
use serde::Deserialize;

const ICON: Asset = asset!("/assets/icon.svg");

#[derive(Clone, Deserialize)]
struct LoginFormData {
    email: String,
    password: String,
}

#[component]
pub fn LoginPage() -> Element {
    let mut login_action = use_action(login);
    let mut get_me = use_action(get_me);
    let mut form_errors = use_signal(Vec::<String>::new);

    let mut auth_state = use_context::<AuthState>();

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

    let password = use_form_field("password", String::new())
        .with_validator(validators::required("Password is required!"));

    form_state.register_field(&email);
    form_state.register_field(&password);

    form_state.revalidate();

    let onsubmit = use_on_submit(&form_state, move |form| async move {
        form_errors.set(Vec::new());
        let form_data: LoginFormData = form.parsed_values().unwrap();
        login_action.call(form_data.email, form_data.password).await;
        match login_action.value() {
            Some(Ok(_)) => {
                get_me.call().await;
                match get_me.value() {
                    Some(Ok(_fetched_user)) => {
                        auth_state.user.set(Some(_fetched_user.peek().clone()));
                        let nav = navigator();
                        nav.push(Route::Home {});
                    }
                    _ => {
                        form_errors.push("Login failed: Failed to retrieve user".into());
                    }
                }
            }
            Some(Err(error)) => {
                debug!("Failed to log in with error {:?}", error);
            }
            None => {
                debug!("No value present!")
            }
        };
    });

    rsx! {
        div { class: "flex flex-col justify-center items-center w-full h-[90vh]",
            div { class: "w-full lg:w-1/2",
                Card {
                    CardBody { class: "items-center text-center",
                        img { src: ICON, class: "aspect-square w-20" }
                        CardTitle { class: "lg:mb-8", "RoomMates" }

                        form { onsubmit, class: "w-full text-left",
                            if form_errors.len() > 0 {
                                div { class: "alert alert-error mb-4",
                                    ul {
                                        for error in form_errors.read().iter() {
                                            li { "{error}" }
                                        }
                                    }
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
                                    label: "Login",
                                    submitting_label: "Logging in...",
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
