use super::components::TodoEntry;
use crate::Route;
use crate::components::ui::button::{Button, ButtonShape, ButtonVariant};
use crate::components::ui::card::{Card, CardBody, CardTitle};
use crate::components::ui::dialog::{Dialog, DialogAction, DialogContent, DialogTrigger};
use crate::components::ui::list::{ComplexListDetails, List, ListDetails, ListRow};
use crate::components::ui::toaster::{ToastOptions, use_toaster};
use api::routes::todo_list::{list_todo_list_members, retrieve_todo_list};
use api::routes::todos::{delete_todo, list_todo, update_todo};
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{
    LdCircle, LdCircleCheckBig, LdPlus, LdTrash, LdUserRoundMinus,
};
use entity::todo::UpdateToDo;
use entity::todo_list_invitation::InvitationPermission;

#[component]
pub fn TodosGroupView(todo_list_id: i32) -> Element {
    let todo_list = use_loader(move || retrieve_todo_list(todo_list_id))?;
    let mut todos = use_loader(move || list_todo(todo_list_id))?;
    let mut members = use_resource(move || list_todo_list_members(todo_list_id));

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
                                permission: todo_list.read().permission.unwrap_or(InvitationPermission::Admin),
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
                                permission: todo_list.read().permission.unwrap_or(InvitationPermission::Admin),
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
                    List { header: "",
                        match &*members.read() {
                            Some(Ok(members)) => {
                                if members.is_empty() {
                                    rsx! {
                                        ListRow {
                                            ListDetails { title: "No members yet" }
                                        }
                                    }
                                } else {
                                    rsx! {
                                        for member in members.iter() {
                                            MemberEntry {
                                                key: "{member.id}",
                                                member: member.clone(),
                                                user_permission: todo_list.read().permission.unwrap_or(InvitationPermission::Admin),
                                            }
                                        }
                                    }
                                }
                            }
                            Some(Err(_)) => rsx! {
                                ListRow {
                                    ListDetails { title: "Failed to load members" }
                                }
                            },
                            None => rsx! {
                                ListRow {
                                    ListDetails { title: "Loading members..." }
                                }
                            },
                        }
                    }
                }
            }
        }

        if todo_list.read().permission.unwrap_or(InvitationPermission::Admin).can_write() {
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

#[component]
fn MemberEntry(
    member: entity::user::UserWithTodoListPermission,
    user_permission: InvitationPermission,
) -> Element {
    rsx! {
        ListRow {
            ListDetails { title: "{member.first_name} {member.last_name}" }
            if user_permission.can_admin() {
                Button { variant: ButtonVariant::Error,
                    Icon { icon: LdUserRoundMinus }
                }
            }
        }
    }
}
