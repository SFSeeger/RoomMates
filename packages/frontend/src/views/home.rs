use crate::Route;
use crate::components::contexts::{AuthGuard, AuthState};
use crate::components::ui::button::{Button, ButtonShape, ButtonVariant};
use crate::components::ui::calendar_small::CalendarDashview;
use crate::components::ui::list::{ComplexListDetails, List, ListDetails, ListRow};
use crate::components::ui::toaster::{ToastOptions, use_toaster};
use api::routes::events::list_events;
use api::routes::todos::list_todos;
use api::routes::todos::update_todo;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{LdCircle, LdCircleCheckBig};
use entity::todo::UpdateToDo;
use roommates::message_from_captured_error;

#[component]
pub fn Home() -> Element {
    rsx! {
        AuthGuard { Dashboard {} }
    }
}

#[component]
fn Dashboard() -> Element {
    let mut selected_date = use_signal(|| time::OffsetDateTime::now_utc().date());
    let auth_state = use_context::<AuthState>();
    let user_ref = auth_state.user.read();

    let mut events = use_loader(move || async move {
        list_events(Some(selected_date()), Some(selected_date())).await
    })?;
    let mut is_loading_events = use_signal(|| events.loading());

    let on_date_change = move |date| {
        selected_date.set(date);
        events.restart();
        is_loading_events.set(true);
    };

    // Sadly Loader.loading() is not reactive, so we have to keep a seperate loading state
    use_effect(move || {
        // events.read() is required to subscribe to state chamges. When events changes, it has finished loading new data
        events.read();
        is_loading_events.set(false);
    });

    let mut todos = use_loader(move || async move { list_todos(Some(false), Some(true)).await })?;

    let on_todo_update = move |id| {
        todos.write().retain(|list| list.id != id);
    };

    rsx! {
        div {
            h1 { class: "text-3xl font-bold",
                if let Some(u) = user_ref.as_ref() {
                    "Hello, {u.first_name}!"
                } else {
                    "Welcome!"
                }
            }
            div { class: "flex gap-4 flex-col lg:flex-row mb-16 lg:mb-0 mt-2",
                div { class: "w-full md:flex-1",
                    CalendarDashview {
                        events: events.read().cloned(),
                        selected_date,
                        on_date_change,
                        is_loading: *is_loading_events.read(),
                    }
                }
                div { class: "w-full md:flex-1",
                    List { header: "Todos",
                        if todos.read().is_empty() {
                            ListRow {
                                ListDetails { title: "No favorite todos yet" }
                            }
                        } else {
                            for todo in todos.iter() {
                                DashboardTodoEntry {
                                    key: "{todo.id}",
                                    todo: todo.clone(),
                                    onupdate: on_todo_update,
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DashboardTodoEntry(
    todo: entity::todo::TodoWithPermission,
    onupdate: EventHandler<i32>,
) -> Element {
    let mut toaster = use_toaster();

    let mut update_completed = use_action(move |completed| async move {
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
            Button {
                onclick: move |_| async move {
                    if !todo.invitation.permission.can_write() {
                        return;
                    }
                    update_completed.call(!todo.completed).await;
                    match update_completed.value() {
                        Some(Ok(todo)) => {
                            toaster.success("Todo completed!", ToastOptions::new());
                            onupdate.call(todo.peek().id);
                        }
                        Some(Err(error)) => {
                            toaster
                                .error(
                                    "Failed to update Todo!",
                                    ToastOptions::new().description(rsx! {
                                        span { "{message_from_captured_error(&error)}" }
                                    }),
                                );
                        }
                        None => {
                            warn!("Update request did not finish!");
                        }
                    }
                },
                variant: ButtonVariant::Primary,
                shape: ButtonShape::Square,
                ghost: true,
                class: "btn-lg",
                disabled: !todo.invitation.permission.can_write(),
                if todo.completed {
                    Icon { icon: LdCircleCheckBig, class: "stroke-success" }
                } else {
                    Icon { icon: LdCircle }
                }
            }
            ComplexListDetails {
                link: Route::TodosGroupView {
                    todo_list_id: todo.todo_list_id,
                },
                title: rsx! {
                    h3 { class: if todo.completed { "line-through text-base-content/60" }, "{todo.title}" }
                },
                if let Some(details) = todo.details {
                    p { class: "text-ellipsis", "{details}" }
                }
            }
        }
    }
}
