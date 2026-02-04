use crate::Route;
use crate::components::ui::button::{Button, ButtonShape, ButtonVariant};
use crate::components::ui::dialog::{Dialog, DialogAction, DialogContent, DialogTrigger};
use crate::components::ui::list::{ComplexListDetails, List, ListDetails, ListRow};
use crate::components::ui::loader::{Loader, LoaderSize};
use crate::components::ui::toaster::{ToastOptions, use_toaster};
use api::routes::todo_list::invite::update_my_todo_list_invitation;
use api::routes::todo_list::{delete_todo_list, list_todo_lists};
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{LdHeart, LdPlus, LdTrash};
use entity::todo_list_invitation::{TodoListInvitationPartialModel, UpdateMyTodoListInvitation};
use roommates::message_from_captured_error;
use std::default::Default;

#[component]
pub fn TodoListListView() -> Element {
    let mut todo_lists = use_loader(list_todo_lists)?;

    let ondelete = move |id: i32| {
        let mut lists_write = todo_lists.write();
        lists_write.retain(|list| list.id != id);
    };

    let onupdate = move |(todo_list_id, invitation)| {
        if let Some(item) = todo_lists
            .write()
            .iter_mut()
            .find(|list| list.id == todo_list_id)
        {
            let new_item = entity::todo_list::TodoListWithPermission {
                invitation,
                ..item.clone()
            };
            *item = new_item;
        }
    };

    rsx! {
        List { header: "Your Todo Lists",
            if todo_lists.read().is_empty() {
                ListRow {
                    ListDetails { title: "No Todo Lists yet" }
                }
            }
            for todo_list in todo_lists.iter() {
                TodoListEntry {
                    key: "{todo_list.id}",
                    todo_list: todo_list.clone(),
                    ondelete,
                    onupdate,
                }
            }
        }
        div { class: "fixed bottom-16 lg:bottom-4 right-4",
            Link {
                to: Route::TodoListCreateView {},
                class: "btn btn-primary btn-circle lg:btn-lg",
                Icon { icon: LdPlus }
            }
        }
    }
}

#[component]
pub fn TodoListEntry(
    todo_list: entity::todo_list::TodoListWithPermission,
    ondelete: EventHandler<i32>,
    onupdate: EventHandler<(i32, TodoListInvitationPartialModel)>,
) -> Element {
    let mut toaster = use_toaster();
    let title = todo_list.title.clone();

    let permission = todo_list.invitation.permission;

    let mut delete_todo_list = use_action(delete_todo_list);
    let mut update_favorite = use_action(move |is_favorite: bool| async move {
        update_my_todo_list_invitation(
            todo_list.id,
            UpdateMyTodoListInvitation {
                is_favorite: Some(is_favorite),
            },
        )
        .await
    });

    rsx! {
        ListRow {
            ComplexListDetails {
                title: rsx! {
                    h3 { class: "flex items-center gap-2",
                        Link {
                            to: Route::TodosGroupView {
                                todo_list_id: todo_list.id,
                            },
                            "{title}"
                        }
                    }
                },
                if let Some(description) = todo_list.description {
                    Link {
                        to: Route::TodosGroupView {
                            todo_list_id: todo_list.id,
                        },
                        p { class: "overflow-hidden text-ellipsis", "{description}" }
                    }
                }
            }
            div { class: "grid grid-cols-2 gap-2",
                Button {
                    onclick: move |_| async move {
                        update_favorite.call(!todo_list.invitation.is_favorite).await;
                        match update_favorite.value() {
                            Some(Ok(updated_value)) => {
                                onupdate.call((todo_list.id, updated_value().invitation));
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
                    variant: ButtonVariant::Primary,
                    shape: ButtonShape::Square,
                    ghost: true,
                    class: "btn-sm",
                    disabled: update_favorite.pending(),
                    if update_favorite.pending() {
                        Loader { size: LoaderSize::Small, class: "text-primary" }
                    } else {
                        Icon {
                            icon: LdHeart,
                            class: if todo_list.invitation.is_favorite { "fill-primary" } else { "" },
                        }
                    }
                }
                if permission.can_admin() {
                    Dialog {
                        DialogTrigger {
                            variant: ButtonVariant::Error,
                            shape: ButtonShape::Square,
                            ghost: true,
                            class: "btn-sm",
                            Icon { icon: LdTrash }
                        }
                        DialogContent { title: "Do you want to delete {title.clone()}?",
                            "This action cannot be undone! All members will be kicked and all To-Dos are lost!"
                            form { method: "dialog",
                                DialogAction {
                                    Button { variant: ButtonVariant::Secondary, "Cancel" }
                                    Button {
                                        onclick: move |_| {
                                            let title_clone = title.clone();
                                            async move {
                                                delete_todo_list.call(todo_list.id).await;
                                                match delete_todo_list.value() {
                                                    Some(Ok(_)) => {
                                                        toaster

                                                            .success(
                                                                &format!("Deleted {title_clone} successfully!"),
                                                                ToastOptions::new(),
                                                            );
                                                        ondelete.call(todo_list.id);
                                                    }
                                                    Some(Err(error)) => {
                                                        toaster
                                                            .error(
                                                                &format!("Failed to delete {title_clone}!"),
                                                                ToastOptions::new().description(rsx! {
                                                                    span { "{error.to_string()}" }
                                                                }),
                                                            );
                                                    }
                                                    None => {
                                                        warn!("Request did not finish!");
                                                    }
                                                }
                                            }
                                        },
                                        variant: ButtonVariant::Error,
                                        "Delete"
                                    }
                                }
                            }
                        }
                    }
                } else {
                    div {}
                }
            }
        }
    }
}
