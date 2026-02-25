use crate::components::ui::button::{Button, ButtonShape, ButtonVariant};
use crate::components::ui::dialog::{Dialog, DialogAction, DialogContent, DialogTrigger};
use crate::components::ui::list::{ComplexListDetails, ListRow};
use crate::components::ui::toaster::{ToastOptions, use_toaster};
use api::routes::todos::{delete_todo, update_todo};
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{LdCircle, LdCircleCheckBig, LdTrash};
use entity::todo::UpdateToDo;
use entity::todo_list_invitation::InvitationPermission;

#[component]
pub fn TodoEntry(
    todo: entity::todo::Model,
    user_permission: InvitationPermission,
    ondelete: EventHandler<i32>,
    onupdate: EventHandler<()>,
) -> Element {
    let mut toaster = use_toaster();
    let title = todo.title.clone();
    let title_for_update = title.clone();

    let mut delete_todo = use_action(delete_todo);
    let mut update_completed = use_action(move |completed: bool| async move {
        update_todo(
            todo.id,
            UpdateToDo {
                completed: Some(completed),
                ..Default::default()
            },
        )
        .await
    });

    rsx! {
        ListRow {
            div {}
            ComplexListDetails {
                title: rsx! {
                    h3 { class: if todo.completed { "line-through text-base-content/60" }, "{todo.title}" }
                },
                if let Some(details) = todo.details {
                    p { class: "text-ellipsis", "{details}" }
                }
            }
            div { class: "grid grid-cols-2 items-center gap-2",
                Button {
                    onclick: move |_| {
                        if !user_permission.can_write() {
                            return;
                        }
                        let title_clone = title_for_update.clone();
                        spawn(async move {
                            update_completed.call(!todo.completed).await;

                            match update_completed.value() {
                                Some(Ok(_)) => {
                                    onupdate.call(());
                                }
                                Some(Err(error)) => {
                                    toaster
                                        .error(
                                            &format!("Failed to update {title_clone}!"),
                                            ToastOptions::new().description(rsx! {
                                                span { "{error.to_string()}" }
                                            }),
                                        );
                                }
                                None => {
                                    warn!("Update request did not finish!");
                                }
                            }
                        });
                    },
                    variant: ButtonVariant::Primary,
                    shape: ButtonShape::Square,
                    ghost: true,
                    class: "btn-sm",
                    disabled: !user_permission.can_write(),
                    if todo.completed {
                        Icon { icon: LdCircleCheckBig, class: "stroke-success" }
                    } else {
                        Icon { icon: LdCircle }
                    }
                }
                if user_permission.can_write() {
                    Dialog {
                        DialogTrigger {
                            variant: ButtonVariant::Error,
                            shape: ButtonShape::Square,
                            ghost: true,
                            class: "btn-sm",
                            Icon { icon: LdTrash }
                        }
                        DialogContent { title: "Do you want to delete {title.clone()}?",
                            form { method: "dialog",
                                DialogAction {
                                    Button { variant: ButtonVariant::Secondary, "Cancel" }
                                    Button {
                                        onclick: move |_| {
                                            let title_clone = title.clone();
                                            async move {
                                                delete_todo.call(todo.id).await;
                                                match delete_todo.value() {
                                                    Some(Ok(_)) => {
                                                        toaster
                                                            .success(
                                                                &format!("Deleted {} successfully!", title_clone),
                                                                ToastOptions::new(),
                                                            );
                                                        ondelete.call(todo.id);
                                                    }
                                                    Some(Err(error)) => {
                                                        toaster
                                                            .error(
                                                                &format!("Failed to delete {}!", title_clone),
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
                    // Placeholder to keep the grid layout consistent
                    div {}
                }
            }
        }
    }
}
