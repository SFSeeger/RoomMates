use crate::Route;
use crate::components::ui::button::{Button, ButtonShape, ButtonVariant};
use crate::components::ui::dialog::{Dialog, DialogAction, DialogContent, DialogTrigger};
use crate::components::ui::list::{List, ListDetails, ListRow};
use crate::components::ui::toaster::{ToastOptions, use_toaster};
use api::routes::todo_list::{delete_todo_list, list_todo_lists, update_todo_list};
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{LdHeart, LdPlus, LdTrash};
use entity::todo_list::UpdateTodoList;
use std::default::Default;

#[component]
pub fn TodoListListView() -> Element {
    let mut todo_lists = use_loader(list_todo_lists)?;

    let ondelete = move |id: i32| {
        let mut lists_write = todo_lists.write();
        lists_write.retain(|list| list.id != id);
    };

    let onupdate = move |model: entity::todo_list::Model| {
        if let Some(item) = todo_lists
            .write()
            .iter_mut()
            .find(|list| list.id == model.id)
        {
            *item = model;
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
    todo_list: entity::todo_list::Model,
    ondelete: EventHandler<i32>,
    onupdate: EventHandler<entity::todo_list::Model>,
) -> Element {
    let mut toaster = use_toaster();
    let title = todo_list.title.clone();

    let mut delete_todo_list = use_action(delete_todo_list);
    let mut update_favorite = use_action(move |is_favorite: bool| async move {
        update_todo_list(
            todo_list.id,
            UpdateTodoList {
                is_favorite: Some(is_favorite),
                ..Default::default()
            },
        )
        .await
    });

    rsx! {
        ListRow {
            ListDetails { title: todo_list.title,
                if let Some(description) = todo_list.description {
                    p { class: "text-ellipsis", "{description}" }
                }
            }
            Button {
                onclick: move |_| async move {
                    update_favorite.call(!todo_list.is_favorite).await;
                    if let Some(Ok(updated_todo_list)) = update_favorite.value() {
                        onupdate.call(updated_todo_list());
                    }
                },
                variant: ButtonVariant::Primary,
                shape: ButtonShape::Square,
                ghost: true,
                class: "btn-sm",
                Icon {
                    icon: LdHeart,
                    class: if todo_list.is_favorite { "fill-primary" } else { "" },
                }
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
        }
    }
}
