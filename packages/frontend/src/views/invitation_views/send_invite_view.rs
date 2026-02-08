use crate::Route;
use crate::components::ui::card::{Card, CardBody, CardTitle};
use crate::components::ui::form::input::Input;
use crate::components::ui::form::submit_button::SubmitButton;
use crate::components::ui::list::{ComplexListDetails, List, ListRow};
use api::routes::events::{list_event_members, retrieve_event};
use api::routes::invitations::send_invite;
use api::routes::users::EMAIL_REGEX;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::LdMail;
use form_hooks::prelude::{use_form, use_form_field, use_on_submit};
use form_hooks::validators;
use regex::Regex;

#[component]
pub fn SendInvite(invite_id: i32) -> Element {
    let load_event = use_loader(move || async move { retrieve_event(invite_id).await })?;
    let event = load_event();
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
                nav.push(Route::ListInviteView {});
            }
            Some(Err(error)) => {
                form_errors.push(error.to_string());
            }
            None => {
                warn!("Invite user request did not finish!");
            }
        }
    });

    rsx! {

        Card { class: "shrink-0 w-full lg:w-1/2 xl:w-1/3",

            CardBody {
                CardTitle { class: "flex items-center justify-between", "Members" }

                if members.is_empty() {
                    p { "not shared with anyone yet" }
                } else {
                    List { header: "",
                        for member in members.iter() {
                            MemberEntry { key: "{member.id}", member: member.clone() }
                        }
                    }
                }
            }

            p { "invite somebody to {event.title}" }

            form { onsubmit,
                div {
                    if form_errors.len() > 0 {
                        div { class: "alert alert-error mb-4", role: "alert",
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
                SubmitButton { form: form_state.clone() }
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
                        span { class: "badge badge-outline badge-info badge-md", "Invited" }
                    }
                },
            }
        }
    }
}
