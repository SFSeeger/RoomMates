use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::card::CardActions;
use crate::components::ui::dialog::{DialogContent, use_dialog};
use crate::components::ui::form::input::Input;
use crate::components::ui::form::submit_button::SubmitButton;
use crate::views::todo::todos_group::use_todo_list;
use api::routes::todos::create_todo;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::LdCircleX;
use entity::todo::CreateToDo;
use form_hooks::use_form::{use_form, use_on_submit};
use form_hooks::use_form_field::use_form_field;
use form_hooks::validators;
use roommates::message_from_captured_error;

#[component]
pub fn TodoCreateForm(ontodochange: EventHandler<()>) -> Element {
    let dialog = use_dialog();
    let todo_list_context = use_todo_list();
    let todo_list_id = todo_list_context.todo_list().id;
    let mut create_todo = use_action(create_todo);

    let mut form_errors = use_signal(Vec::<String>::new);
    let mut form_state = use_form();

    let title = use_form_field("title", String::new())
        .with_validator(validators::required("Title is required"));
    let details = use_form_field("details", None::<String>);
    form_state.register_field(&title);
    form_state.register_field(&details);
    form_state.revalidate();
    let form_state_clone = form_state.clone();

    let onsubmit = use_on_submit(&form_state, move |mut form_state| async move {
        form_errors.clear();
        let form_data: CreateToDo = form_state.parsed_values().unwrap();
        create_todo.call(todo_list_id, form_data).await;
        match create_todo.value() {
            Some(Ok(_)) => {
                ontodochange.call(());
                form_state.reset();
                dialog.close();
            }
            Some(Err(error)) => {
                form_errors.push(message_from_captured_error(&error));
            }
            None => {
                warn!("Task still creating...");
            }
        }
    });

    rsx! {
        DialogContent { title: "Create To-Do", dismissible: false, close_button: false,
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
                Input { field: title, label: "Title", r#type: "text" }
                Input { field: details, label: "Details", r#type: "text" }
                CardActions {
                    Button {
                        variant: ButtonVariant::Secondary,
                        class: "grow",
                        r#type: "button",
                        onclick: move |_| {
                            dialog.close();
                            form_state.reset()
                        },
                        "Cancel"
                    }
                    SubmitButton {
                        form: form_state_clone,
                        label: "Create To-Do",
                        submitting_label: "Creating To-Do...",
                    }
                }
            }
        }
    }
}
