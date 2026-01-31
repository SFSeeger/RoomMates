use crate::Route;
use crate::components::ui::card::{Card, CardActions, CardBody, CardTitle};
use crate::components::ui::form::input::Input;
use crate::components::ui::form::submit_button::SubmitButton;
use api::routes::todo_list::create_todo_list;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::LdCircleX;
use entity::todo_list::CreateTodoList;
use form_hooks::use_form::{use_form, use_on_submit};
use form_hooks::use_form_field::use_form_field;
use form_hooks::validators;

#[component]
pub fn TodoListCreateView() -> Element {
    let mut create_todo_list = use_action(create_todo_list);
    let nav = use_navigator();

    let mut form_errors = use_signal(Vec::<String>::new);
    let mut form_state = use_form();
    let title = use_form_field("title", String::new())
        .with_validator(validators::required("Title is required"));
    let description = use_form_field("description", None::<String>);
    form_state.register_field(&title);
    form_state.register_field(&description);
    form_state.revalidate();

    let onsubmit = use_on_submit(&form_state, move |form_state| async move {
        form_errors.clear();
        let form_data: CreateTodoList = form_state.parsed_values().unwrap();
        create_todo_list.call(form_data).await;
        match create_todo_list.value() {
            Some(Ok(_)) => {
                nav.push(Route::TodoListListView {});
            }
            Some(Err(error)) => {
                form_errors.push(error.to_string());
            }
            None => {
                warn!("Todo list still creating...");
            }
        }
    });

    rsx! {
        div { class: "flex flex-col justify-center items-center w-full h-[90vh]",
            div { class: "w-full lg:w-1/2",
                Card {
                    CardBody { class: "items-center text-center",
                        CardTitle { class: "lg:mb-8", "Create a new Todo List" }

                        form { onsubmit, class: "w-full text-left",
                            if form_errors.len() > 0 {
                                div {
                                    class: "alert alert-error mb-4",
                                    role: "alert",
                                    Icon { icon: LdCircleX }
                                    ul {
                                        for error in form_errors.read().iter() {
                                            li { key: "{error}", "{error}" }
                                        }
                                    }
                                }
                            }
                            Input {
                                field: title,
                                label: "Title",
                                r#type: "text",
                            }
                            Input {
                                field: description,
                                label: "Description",
                                r#type: "text",
                            }
                            CardActions {
                                Link {
                                    to: Route::TodoListListView {},
                                    class: "btn btn-secondary grow",
                                    "Cancel"
                                }
                                SubmitButton {
                                    form: form_state.clone(),
                                    label: "Create Todo List",
                                    submitting_label: "Creating Todo List...",
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
