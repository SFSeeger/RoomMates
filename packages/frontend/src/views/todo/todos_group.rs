use crate::Route;
use crate::components::ui::button::{Button, ButtonShape, ButtonVariant};
use crate::components::ui::dialog::{Dialog, DialogAction, DialogContent, DialogTrigger};
use crate::components::ui::list::{ComplexListDetails, List, ListDetails, ListRow};
use crate::components::ui::toaster::{ToastOptions, use_toaster};
use api::routes::todos::{delete_todo, list_todo, update_todo};
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{LdCircle, LdCircleCheckBig, LdPlus, LdTrash};
use entity::todo::UpdateToDo;
use std::default::Default;

#[component]
pub fn TodosGroupView(todo_list_id: i32) -> Element {
    let mut todos = use_loader(move || list_todo(todo_list_id))?;
    let completed_todos = use_memo(move || {
        todos
            .iter()
            .filter_map(|todo| {
                if todo.completed {
                    Some(todo.cloned())
                } else {
                    None
                }
            })
            .collect::<Vec<entity::todo::Model>>()
    });

    let uncompleted_todos = use_memo(move || {
        todos
            .iter()
            .filter_map(|todo| {
                if !todo.completed {
                    Some(todo.cloned())
                } else {
                    None
                }
            })
            .collect::<Vec<entity::todo::Model>>()
    });

    let ondelete = move |id: i32| {
        let mut todos_write = todos.write();
        todos_write.retain(|todo| todo.id != id);
    };

    let onupdate = move |_| {
        todos.restart();
    };

    rsx! {
        List { header: "Your To-Do's",
            if uncompleted_todos.read().is_empty() {
                ListRow {
                    ListDetails { title: "No To-Do's yet" }
                }
            }
            for todo in uncompleted_todos.iter() {
                TodoEntry {
                    key: "{todo.id}",
                    todo: todo.clone(),
                    ondelete,
                    onupdate,
                }
            }
        }

        List { header: "Completed",
            if completed_todos.read().is_empty() {
                ListRow {
                    ListDetails { title: "No To-Do's yet" }
                }
            }
            for todo in completed_todos.iter() {
                TodoEntry {
                    key: "{todo.id}",
                    todo: todo.clone(),
                    ondelete,
                    onupdate,
                }
            }
        }

        div { class: "fixed bottom-16 lg:bottom-4 right-4",
            Link {
                to: Route::TodosCreateView {
                    todo_list_id,
                },
                class: "btn btn-primary btn-circle lg:btn-lg",
                Icon { icon: LdPlus }
            }
        }
    }
}

#[component]
pub fn TodoEntry(
    todo: entity::todo::Model,
    ondelete: EventHandler<i32>,
    onupdate: EventHandler<entity::todo::Model>,
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
            ComplexListDetails {
                title: rsx! {
                    h3 { class: if todo.completed { "line-through text-base-content/60" }, "{todo.title}" }
                },
                if let Some(details) = todo.details {
                    p { class: "text-ellipsis", "{details}" }
                }
            }
            div { class: "flex items-center gap-2",

                Button {
                    onclick: move |_| {
                        let title_clone = title_for_update.clone();
                        spawn(async move {
                            update_completed.call(!todo.completed).await;

                            match update_completed.value() {
                                Some(Ok(updated_todo)) => {
                                    toaster

                                        .success(
                                            &format!("Updated {} successfully!", title_clone),
                                            ToastOptions::new(),
                                        );
                                    onupdate.call(updated_todo());
                                }
                                Some(Err(error)) => {
                                    toaster
                                        .error(
                                            &format!("Failed to update {}!", title_clone),
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
                    if todo.completed {
                        Icon { icon: LdCircleCheckBig, class: "stroke-success" }
                    } else {
                        Icon { icon: LdCircle }
                    }
                }
                Button {
                    variant: ButtonVariant::Primary,
                    shape: ButtonShape::Square,
                    ghost: true,
                    class: "btn-sm",
                }

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
            }
        }
    }
}
