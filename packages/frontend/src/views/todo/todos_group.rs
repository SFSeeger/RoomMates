use super::components::{MemberList, TodoEntry};
use crate::Route;
use crate::components::ui::card::{Card, CardBody, CardTitle};
use crate::components::ui::list::{List, ListDetails, ListRow};
use api::routes::todo_list::retrieve_todo_list;
use api::routes::todos::list_todo;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::LdPlus;
use entity::todo_list_invitation::InvitationPermission;

#[component]
pub fn TodosGroupView(todo_list_id: i32) -> Element {
    let todo_list = use_loader(move || retrieve_todo_list(todo_list_id))?;
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

    let user_permission = todo_list
        .read()
        .permission
        .unwrap_or(InvitationPermission::Admin);

    rsx! {
        div { class: "flex gap-2 flex-col md:flex-row",
            Card { class: "grow w-full",
                CardBody {
                    CardTitle { "{todo_list.read().title}" }
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
                                user_permission,
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
                                user_permission,
                                ondelete,
                                onupdate,
                            }
                        }
                    }
                }
            }
            Card { class: "w-full md:w-1/3",
                CardBody {
                    CardTitle { "Members" }
                    MemberList { todo_list_id, user_permission }
                }
            }
        }

        if user_permission.can_write() {
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
}
