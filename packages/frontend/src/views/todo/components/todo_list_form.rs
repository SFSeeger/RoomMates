use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::card::CardTitle;
use crate::components::ui::form::input::Input;
use crate::components::ui::form::submit_button::SubmitButton;
use crate::components::ui::form::textarea::Textarea;
use crate::components::ui::toaster::{ToastOptions, use_toaster};
use crate::views::todo::todos_group::use_todo_list;
use api::routes::todo_list::update_todo_list;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{LdPen, LdX};
use entity::todo_list::{TodoListWithPermission, UpdateTodoList};
use form_hooks::prelude::{use_form_signal, use_on_submit};
use form_hooks::use_form_field::use_form_field;
use frontend::message_from_captured_error;

#[component]
pub fn TodoListForm() -> Element {
    let mut toaster = use_toaster();

    let mut todo_list_context = use_todo_list();
    let todo_list = todo_list_context.todo_list();
    let todo_list_id = todo_list.id;

    let mut show_edit = use_signal(|| false);
    let mut edit_todo_list = use_action(move |data| update_todo_list(todo_list_id, data));

    let form_state_signal = use_form_signal();
    let mut form_state = form_state_signal();
    let name_field = use_form_field("title", todo_list.title.clone());
    let description_field = use_form_field("description", todo_list.description.clone());
    form_state.register_field(&name_field);
    form_state.register_field(&description_field);

    let onsubmit = use_on_submit(&form_state, move |mut form_state| async move {
        let data: UpdateTodoList = form_state.parsed_values().unwrap();
        edit_todo_list.call(data).await;
        match edit_todo_list.value() {
            Some(Ok(new_todo_list)) => {
                let new_todo_list = new_todo_list.read();
                todo_list_context.set_todo_list(TodoListWithPermission {
                    id: new_todo_list.id,
                    title: new_todo_list.title.clone(),
                    description: new_todo_list.description.clone(),
                    invitation: todo_list.invitation,
                });
                form_state.mark_clean();
                show_edit.set(false);
            }
            Some(Err(error)) => {
                toaster.error(
                    "Failed to update To-Do List",
                    ToastOptions::new().description(rsx! {
                        p { "{message_from_captured_error(&error)}" }
                    }),
                );
            }
            None => {
                warn!("Request to update To-Do List did not finish");
            }
        }
    });

    form_state.revalidate();

    rsx! {
        if *show_edit.read() {
            form { onsubmit,
                div { class: "flex justify-between items-center gap-2",
                    Input { label: "Title", field: name_field }
                    div { class: "flex gap-2",
                        Button {
                            variant: ButtonVariant::Secondary,
                            class: "hidden md:block",
                            r#type: "button",
                            onclick: move |_| {
                                show_edit.set(false);
                                form_state_signal().reset();
                            },
                            Icon { icon: LdX }
                        }
                        SubmitButton {
                            class: "shrink hidden md:block",
                            form: form_state_signal(),
                        }
                    }
                }
                Textarea { label: "Description", field: description_field }
                div { class: "flex items-center justify-between gap-2",
                    Button {
                        variant: ButtonVariant::Secondary,
                        class: "grow visible md:hidden",
                        r#type: "button",
                        onclick: move |_| {
                            show_edit.set(false);
                            form_state_signal().reset();
                        },
                        "Cancel"
                    }
                    SubmitButton {
                        class: "visible md:hidden",
                        form: form_state_signal(),
                        label: "Save",
                        submitting_label: "Saving...",
                    }
                }
            }
        } else {
            CardTitle { class: "flex items-center justify-between gap-2",
                "{todo_list.title}"
                if todo_list_context.permission().can_write() {
                    Button {
                        variant: ButtonVariant::Primary,
                        onclick: move |_| show_edit.set(true),
                        Icon { icon: LdPen }
                    }
                }
            }
            if let Some(description) = &todo_list.description {
                p { "{description}" }
            }
        }
    }
}
