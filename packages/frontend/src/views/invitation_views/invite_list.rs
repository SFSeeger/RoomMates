use crate::components::tooltip::Tooltip;
use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::list::{ComplexListDetails, List, ListRow};
use crate::components::ui::toaster::{ToastOptions, use_toaster};
use api::routes::events::invitations::{accept_invite, decline_invite, list_received_invites};
use api::routes::events::retrieve_event;
use api::routes::todo_list::invite::{
    accept_todo_list_invite, decline_todo_list_invite, list_todo_invites,
};
use api::routes::todo_list::retrieve_todo_list;
use api::routes::users::retrieve_user;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{LdCheck, LdMapPin, LdRefreshCcw, LdX};
use roommates::message_from_captured_error;
use time::macros::format_description;

#[component]
pub fn ListInviteView() -> Element {
    let mut invites = use_loader(move || async move { list_received_invites().await })?;
    let mut todo_invites = use_loader(move || async move { list_todo_invites(Some(false)).await })?;

    let ondecide = move |id: i32| {
        let mut lists_write = invites.write();
        lists_write.retain(|invite| invite.id != id);
    };

    let onclick_todo = move |(todo_id, user_id)| {
        let mut lists_write = todo_invites.write();
        lists_write
            .retain(|invite| invite.todo_list_id != todo_id && invite.receiving_user_id != user_id);
    };

    rsx! {

        List { header: "Your Inbox",
            for element in invites.iter() {

                EventInviteRow { invite: element.clone(), ondecide }
            }
            for element in todo_invites.iter() {

                TodoInviteRow { invite: element.clone(), onclick_todo } //change on l
            }
        }

    }
}

#[component]
pub fn EventInviteRow(invite: entity::invitation::Model, ondecide: EventHandler<i32>) -> Element {
    let event = use_loader(move || async move { retrieve_event(invite.event_id).await })?();
    let mut accept_action = use_action(accept_invite);
    let mut decline_action = use_action(decline_invite);
    let user = use_loader(move || async move { retrieve_user(event.owner_id).await })?();

    let mut toaster = use_toaster();

    let start = event
        .start_time
        .format(format_description!("[hour]:[minute]"))
        .unwrap();
    let end = event
        .end_time
        .format(format_description!("[hour]:[minute]"))
        .unwrap();
    let date = event
        .date
        .format(format_description!("[day].[month].[year]"))
        .unwrap();

    rsx! {
        div { class: "w-full",
            ListRow {
                ComplexListDetails {
                    title: rsx! {
                        "You were invited to: {event.title} by {user.first_name} {user.last_name}"
                        div { class: "whitespace-nowrap",
                            if event.reoccurring {
                                h1 { "{event.weekday:?}" }
                            } else {
                                h1 { "{date}" }
                            }
                            p { "{start} - {end}" }
                        }
                        div { class: "flex flex-wrap items-center gap-2",
                            div {
                                if event.reoccurring {
                                    Tooltip { tooltip: "Reoccurring event",
                                        Icon { icon: LdRefreshCcw }
                                    }
                                }
                            }


                            div { class: "flex items-center gap-1 whitespace-nowrap",
                                if let Some(text) = &event.location {
                                    div {
                                        Icon { icon: LdMapPin }
                                    }
                                    br {}
                                    div { "{text}" }
                                }
                            }
                        }
                    },

                    div { class: "flex w-full items-center gap-4 flex-wrap md:flex-nowrap",

                        div { class: "flex gap-2 flex-1 items-center",
                            if let Some(text) = &event.description {
                                span { "{text}" }
                            }
                        }
                    }
                }

                div { class: "flex gap-2 ml-auto",
                    Button {
                        onclick: move |_| async move {
                            accept_action.call(invite.id).await;
                            match accept_action.value() {

                                Some(Ok(_)) => {
                                    ondecide.call(invite.id);
                                }
                                Some(Err(error)) => {
                                    toaster
                                        .error(
                                            "Failed to update favorite!",
                                            ToastOptions::new().description(rsx! {
                                                p { "{message_from_captured_error(&error)}" }
                                            }),
                                        );
                                }
                                None => {
                                    warn!("Request to update favorite did not finish");
                                }
                            }
                        },
                        variant: ButtonVariant::Success,
                        Icon { icon: LdCheck }
                    }

                    Button {
                        onclick: move |_| async move {
                            decline_action.call(invite.id).await;
                            match decline_action.value() {

                                Some(Ok(_)) => {
                                    ondecide.call(invite.id);
                                }
                                Some(Err(error)) => {
                                    toaster
                                        .error(
                                            "Failed to update favorite!",
                                            ToastOptions::new().description(rsx! {
                                                p { "{message_from_captured_error(&error)}" }
                                            }),
                                        );
                                }
                                None => {
                                    warn!("Request to update favorite did not finish");
                                }
                            }
                        },
                        variant: ButtonVariant::Error,
                        Icon { icon: LdX }
                    }
                }
            }
        }
    }
}

#[component]
pub fn TodoInviteRow(
    invite: entity::todo_list_invitation::Model,
    onclick_todo: EventHandler<(i32, i32)>,
) -> Element {
    let todo_list =
        use_loader(move || async move { retrieve_todo_list(invite.todo_list_id).await })?();
    let title = todo_list.title.clone();

    //unsafe?? überhaupt nötig?
    let user =
        use_loader(move || async move { retrieve_user(invite.sender_user_id.unwrap()).await })?();

    let mut accept_action = use_action(accept_todo_list_invite);
    let mut decline_action = use_action(decline_todo_list_invite);

    let mut toaster = use_toaster();

    rsx! {
        ListRow {
            ComplexListDetails {
                title: rsx! {
                    h3 { class: "flex items-center gap-2" }
                    "{user.first_name} {user.last_name} wants to collaborate on {title}"
                },
                if let Some(description) = todo_list.description {
                    p { class: "overflow-hidden text-ellipsis", "{description}" }
                }
            }

            div { class: "flex gap-2 ml-auto",
                Button {
                    onclick: move |_| async move {
                        accept_action.call(invite.todo_list_id).await;

                        match accept_action.value() {
                            Some(Ok(_)) => {
                                onclick_todo.call((invite.todo_list_id, invite.receiving_user_id));
                            }
                            Some(Err(error)) => {
                                toaster
                                    .error(
                                        "Failed to update favorite!",
                                        ToastOptions::new().description(rsx! {
                                            p { "{message_from_captured_error(&error)}" }
                                        }),
                                    );
                            }
                            None => {
                                warn!("Request to update favorite did not finish");
                            }
                        }
                    },
                    variant: ButtonVariant::Success,
                    Icon { icon: LdCheck }
                }

                Button {
                    onclick: move |_| async move {
                        decline_action.call(invite.todo_list_id).await;
                        match decline_action.value() {
                            Some(Ok(_)) => {
                                onclick_todo.call((invite.todo_list_id, invite.receiving_user_id));
                            }
                            Some(Err(error)) => {
                                toaster
                                    .error(
                                        "Failed to update favorite!",
                                        ToastOptions::new().description(rsx! {
                                            p { "{message_from_captured_error(&error)}" }
                                        }),
                                    );
                            }
                            None => {
                                warn!("Request to update favorite did not finish");
                            }
                        }
                    },
                    variant: ButtonVariant::Error,
                    Icon { icon: LdX }
                }
            }
        }
    }
}
