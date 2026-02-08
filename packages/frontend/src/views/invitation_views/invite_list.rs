use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::list::{ComplexListDetails, List, ListDetails, ListRow};
use api::routes::events::retrieve_event;
use api::routes::invitations::{accept_invite, decline_invite, list_received_invites};
use api::routes::todo_list::invite::{
    accept_todo_list_invite, decline_todo_list_invite, list_todo_invites,
};
use api::routes::todo_list::retrieve_todo_list;
use api::routes::users::retrieve_user;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{LdBadgeInfo, LdCheck, LdNavigation, LdX};

#[component]
pub fn ListInviteView() -> Element {
    let mut invites = use_loader(move || async move { list_received_invites().await })?;
    let mut todo_invites = use_loader(move || async move { list_todo_invites().await })?;

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
    let load_event = use_loader(move || async move { retrieve_event(invite.event_id).await })?;
    let event = load_event();
    let user = use_loader(move || async move { retrieve_user(event.owner_id).await })?();

    let mut accept_action = use_action(accept_invite);
    let mut decline_action = use_action(decline_invite);

    rsx! {
        ListRow {
            ListDetails { title: "{user.first_name} {user.last_name} invites you to {event.title}",
                div { class: "flex-row gap-w justify-content full",
                    p { "{event.start_time} - {event.end_time}" }

                    if event.reoccurring {
                        h1 { class: "w-20", "Every {event.weekday:?}" }
                    } else {
                        h1 { class: "w-20", "{event.date}" }
                    }

                    div {
                        Icon { icon: LdBadgeInfo }
                        p { class: "w-100 ",
                            match &event.description {
                                Some(Text) => rsx! { "{Text}" },
                                None => rsx! { "no description" },
                            }
                        }
                    }
                    div {
                        Icon { icon: LdNavigation }
                        p { class: "w-50",
                            match &event.location {
                                Some(Text) => rsx! { "{Text}" },
                                None => rsx! { "no location" },
                            }
                        }
                    }

                    div {
                        Button {
                            onclick: move |_| async move {
                                accept_action.call(invite.id).await;
                                //TODO add error hadneling

                                ondecide.call(invite.id);

                            },
                            variant: ButtonVariant::Success,
                            Icon { icon: LdCheck }
                        }
                    }
                    div {
                        Button {
                            onclick: move |_| async move {
                                decline_action.call(invite.id).await;

                                ondecide.call(invite.id);

                            },
                            variant: ButtonVariant::Error,
                            Icon { icon: LdX }
                        }
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

    let mut accept_action = use_action(accept_todo_list_invite);
    let mut decline_action = use_action(decline_todo_list_invite);

    rsx! {
        ListRow {
            ComplexListDetails {
                title: rsx! {
                    h3 { class: "flex items-center gap-2" }
                    "{title}"
                },
                if let Some(description) = todo_list.description {
                    p { class: "overflow-hidden text-ellipsis", "{description}" }
                }

                div { class: "grid grid-cols-2 gap-2",
                    Button {
                        onclick: move |_| async move {
                            accept_action.call(invite.todo_list_id).await;

                            onclick_todo.call((invite.todo_list_id, invite.receiving_user_id));

                        },
                        variant: ButtonVariant::Success,
                        class: "btn-sm",
                        Icon { icon: LdCheck }
                    }
                }
                div { class: "grid grid-cols-2 gap-2",
                    Button {
                        onclick: move |_| async move {
                            decline_action.call(invite.todo_list_id).await;

                            onclick_todo.call((invite.todo_list_id, invite.receiving_user_id));

                        },
                        variant: ButtonVariant::Error,
                        class: "btn-sm",
                        Icon { icon: LdX }
                    }
                }
            }
        }

    }
}
