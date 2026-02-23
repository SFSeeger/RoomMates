use crate::Route;
use crate::components::ui::card::{Card, CardActions, CardBody, CardTitle};
use crate::components::ui::form::input::Input;
use crate::components::ui::form::submit_button::SubmitButton;
use crate::components::ui::list::{ComplexListDetails, List, ListRow};
use api::routes::events::invitations::send_invite;
use api::routes::events::{list_event_members, retrieve_event};
use api::routes::users::EMAIL_REGEX;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::LdMail;
use form_hooks::prelude::{use_form, use_form_field, use_on_submit};
use form_hooks::validators;
use regex::Regex;
use roommates::message_from_captured_error;

#[component]
pub fn SendInvite(invite_id: i32) -> Element {
    let event = use_loader(move || async move { retrieve_event(invite_id).await })?();
    let members = use_loader(move || async move { list_event_members(invite_id).await })?();

    let mut invite_action = use_action(send_invite);

    let mut form_errors = use_signal(Vec::<String>::new);
    let mut form_state = use_form();
    let email = use_form_field("email", String::new()).with_validator(validators::pattern(
        Regex::new(EMAIL_REGEX)?,
        "Email must be valid!",
    ));
    form_state.register_field(&email);
    form_state.revalidate();

    let onsubmit = use_on_submit(&form_state, move |_| async move {
        let email_value = email.value.peek().clone();
        invite_action.call(email_value, event.id).await;
        match invite_action.value().as_ref() {
            Some(Ok(_)) => {
                let nav = navigator();
                nav.push(Route::ListEventView {});
            }
            Some(Err(error)) => {
                form_errors.push(message_from_captured_error(error));
            }
            None => {
                warn!("Invite user request did not finish!");
            }
        }
    });

    rsx! {

        div { class: " flex flex-col items-center justify-center w-full  h-[90vh]",
            div { class: "w-full lg:w-1/2",

                Card {

                    CardBody {
                        CardTitle { class: "flex items-center justify-between", "{event.title} Members:" }

                        List { header: "",
                            for member in members.iter() {

                                if member.id == event.owner_id {
                                    ListRow {
                                        ComplexListDetails {
                                            title: rsx! {
                                                h3 { class: "flex flex-wrap items-center gap-2",
                                                    "{member.first_name} {member.last_name}"
                                                    span { class: "badge badge-outline badge-info badge-md", "Owner" }
                                                }
                                            },
                                        }
                                    }
                                } else {
                                    MemberEntry {
                                        key: "{member.id}",
                                        member: member.clone(),
                                    }
                                }
                            }
                        }
                    }
                    div { class: "divider" }

                    p { "Invite somebody to {event.title}" }

                    form { onsubmit,
                        div {
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
                        }
                        Input {
                            label: "Email",
                            icon: rsx! {
                                Icon { icon: LdMail }
                            },
                            field: email,
                        }
                        CardActions {
                            SubmitButton { form: form_state.clone(), label: "Invite" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn MemberEntry(member: entity::user::Model) -> Element {
    rsx! {
        ListRow {
            ComplexListDetails {
                title: rsx! {
                    h3 { class: "flex flex-wrap items-center gap-2",
                        "{member.first_name} {member.last_name}"
                        span { class: "badge badge-outline badge-info badge-md", "Member" }
                    }
                },
            }
        }
    }
}
