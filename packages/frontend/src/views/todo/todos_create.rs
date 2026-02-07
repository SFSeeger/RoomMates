use crate::Route;
use crate::components::ui::card::CardActions;
use crate::components::ui::form::input::Input;
use crate::components::ui::form::submit_button::SubmitButton;
use api::routes::todos::create_todo;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::LdCircleX;
use entity::todo::CreateToDo;
use form_hooks::use_form::{use_form, use_on_submit};
use form_hooks::use_form_field::use_form_field;
use form_hooks::validators;

#[component]
pub fn TodosCreateView(todo_list_id: i32) -> Element {
    let mut create_todo = use_action(create_todo);
    let nav = use_navigator();

    let mut form_errors = use_signal(Vec::<String>::new);
    let mut form_state = use_form();

    let title = use_form_field("title", String::new())
        .with_validator(validators::required("Title is required"));
    let details = use_form_field("details", None::<String>);
    let completed = use_form_field("completed", false);
    form_state.register_field(&title);
    form_state.register_field(&details);
    form_state.register_field(&completed);
    form_state.revalidate();

    let onsubmit = use_on_submit(&form_state, move |form_state| async move {
        form_errors.clear();

        let form_data: CreateToDo = form_state.parsed_values().unwrap();

        create_todo.call(todo_list_id, form_data).await;

        match create_todo.value() {
            Some(Ok(_)) => {
                nav.push(Route::TodosGroupView { todo_list_id });
            }
            Some(Err(error)) => {
                form_errors.push(error.to_string());
            }
            None => {
                warn!("Task still creating...")
            }
        }
    });

    rsx! {
        div { class: "w-full flex flex-col items-center mt-10",
            h2 { class: "text-2xl font-bold mb-6", "Add a Task" }
            form { onsubmit, class: "w-full max-w-l",
                if form_errors.len() > 0 {
                    div { class: "alert alert-error mb-4", role: "alert",
                        Icon { icon: LdCircleX }
                        ul {
                            for error in form_errors.read().iter() {
                                li { key: "{error}", "{error}" }
                            }
                        }
                    }
                }
                ul { class: "menu bg-base-200 rounded-box p-4 space-y-4" }
                li {
                    Input { field: title, label: "Title", r#type: "text" }
                }
                li {
                    Input { field: details, label: "Details", r#type: "text" }
                }

                CardActions {
                    Link {
                        to: Route::TodosGroupView {
                            todo_list_id,
                        },
                        class: "btn btn-secondary grow",
                        "Cancel"
                    }
                    SubmitButton {
                        form: form_state.clone(),
                        label: "Create To-Do",
                        submitting_label: "Creating To-Do...",
                    }
                }
            }
        }
    }
}
