use super::components::{MemberList, TodoCreateForm, TodoEntry, TodoListForm};
use crate::components::ui::button::{ButtonShape, ButtonVariant};
use crate::components::ui::card::{Card, CardBody};
use crate::components::ui::dialog::{Dialog, DialogTrigger};
use crate::components::ui::list::{List, ListDetails, ListRow};
use api::routes::todo_list::retrieve_todo_list;
use api::routes::todos::list_todo;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::LdPlus;
use entity::todo_list::TodoListWithPermission;
use entity::todo_list_invitation::InvitationPermission;

#[derive(Clone, Copy, PartialEq)]
pub struct TodoListContext {
    todo_list: Signal<TodoListWithPermission>,
    update_todo_list: EventHandler<()>,
}

impl TodoListContext {
    pub fn new(
        todo_list: Signal<TodoListWithPermission>,
        update_todo_list: EventHandler<()>,
    ) -> Self {
        Self {
            todo_list,
            update_todo_list,
        }
    }

    /// Returns the To-Do List
    pub fn todo_list(&self) -> TodoListWithPermission {
        (self.todo_list)()
    }

    pub fn set_todo_list(&mut self, todo_list: TodoListWithPermission) {
        self.todo_list.set(todo_list);
    }

    /// Returns the user permission or Admin if permission is None
    pub fn permission(&self) -> InvitationPermission {
        (self.todo_list)().invitation.permission
    }

    /// Call this to trigger a reload of the To-Do List and all dependent data
    pub fn update_todo_list(&self) {
        self.update_todo_list.call(());
    }
}

#[component]
pub fn TodosGroupView(todo_list_id: i32) -> Element {
    let mut todo_list = use_loader(move || retrieve_todo_list(todo_list_id))?;
    let mut todo_list_signal = use_signal(|| todo_list.cloned());
    let update_todo_list = EventHandler::new(move |()| {
        todo_list.restart();
    });

    use_effect(move || {
        todo_list_signal.set(todo_list.read().cloned());
    });

    use_context_provider(|| TodoListContext::new(todo_list_signal, update_todo_list));
    let user_permission = todo_list_signal.read().invitation.permission;

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
                if todo.completed {
                    None
                } else {
                    Some(todo.cloned())
                }
            })
            .collect::<Vec<entity::todo::Model>>()
    });

    let ondelete = move |id: i32| {
        let mut todos_write = todos.write();
        todos_write.retain(|todo| todo.id != id);
    };

    let onupdate = move || {
        todos.restart();
    };

    rsx! {
        div { class: "flex gap-2 flex-col lg:flex-row mb-16 lg:mb-0",
            Card { class: "grow w-full",
                CardBody {
                    TodoListForm {}
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
            MemberList {}
        }

        if user_permission.can_write() {
            div { class: "fixed bottom-16 lg:bottom-4 right-4",
                Dialog {
                    DialogTrigger {
                        variant: ButtonVariant::Primary,
                        shape: ButtonShape::Round,
                        class: "lg:btn-lg",
                        Icon { icon: LdPlus }
                    }
                    TodoCreateForm { ontodochange: onupdate }
                }
            }
        }
    }
}

pub fn use_todo_list() -> TodoListContext {
    try_use_context::<TodoListContext>()
        .expect("Cannot use 'use_todo_list' outside of TodosGroupView!")
}
