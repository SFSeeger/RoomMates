use crate::Route;
use crate::components::ui::card::CardTitle;
use crate::components::ui::{
    card::{Card, CardActions, CardBody},
    form::checkbox::Checkbox,
    form::input::Input,
    form::select::Select,
    form::submit_button::SubmitButton,
    form::textarea::Textarea,
};
use api::routes::events::{retrieve_event, update_event};
use chrono::{NaiveDate, NaiveTime};
use dioxus::prelude::*;
use entity::event::PartialEvModel;
use form_hooks::use_form::{use_form, use_on_submit};
use form_hooks::use_form_field::{FormField, use_form_field};
use form_hooks::validators;

#[component]
pub fn EventEditor(event_id: i32) -> Element {
    let event = use_loader(move || async move { retrieve_event(event_id).await })?;
    let mut form_state = use_form();
    let mut update_action: Action<(i32, PartialEvModel), entity::event::Model> =
        use_action(update_event);
    let mut form_errors = use_signal(Vec::<String>::new);

    let title: FormField<String> = use_form_field("title", event.read().clone().title)
        .with_validator(validators::required("event needs a title"));
    let reocurring: FormField<bool> = use_form_field("reocurring", event.read().reocurring);
    let private: FormField<bool> = use_form_field("private", event.read().private);
    let desc: FormField<Option<String>> =
        use_form_field("description", event.read().clone().description);
    let loc: FormField<Option<String>> = use_form_field("location", event.read().clone().location);
    let date: FormField<NaiveDate> = use_form_field("date", event.read().date);
    let start = use_form_field("start_time", event.read().start_time);
    let end: FormField<NaiveTime> = use_form_field("end_time", event.read().end_time);
    let weekday = use_form_field("weekday", event.read().clone().weekday);

    form_state.register_field(&title);
    form_state.register_field(&reocurring);
    form_state.register_field(&private);
    form_state.register_field(&desc);
    form_state.register_field(&loc);
    form_state.register_field(&date);
    form_state.register_field(&start);
    form_state.register_field(&end);
    form_state.register_field(&weekday);

    form_state.revalidate();

    let onsubmit = use_on_submit(&form_state, move |submit_state| async move {
        form_errors.clear();
        let form_data: PartialEvModel = submit_state.parsed_values().unwrap();

        update_action.call(event.read().id, form_data).await;

        match update_action.value() {
            Some(Ok(_)) => {
                let nav = navigator();
                nav.push(Route::Events {});
            }
            Some(Err(error)) => {
                form_errors.push(error.to_string());
            }
            None => {
                warn!("Error creating event. API call did not complete")
            }
        };
    });

    rsx! {
        div {
            Card {
                CardTitle { "Edit Event" }

                CardBody {

                    div { class: "flex flex-col items-center gap-4 justify-center w-full",

                        form { onsubmit, class: "w-full text-left",
                            if form_errors.len() > 0 {
                                div {
                                    class: "alert alert-error mb-4",
                                    role: "alert",
                                    ul {
                                        for error in form_errors.read().iter() {
                                            li { key: "{error}", "{error}" }
                                        }
                                    }
                                }
                            }
                            Input::<String> { field: title, label: "Title" }
                            Checkbox::<bool> { label: "Reocurring", field: reocurring }
                            Checkbox::<bool> { label: "Private", field: private }
                            Textarea {
                                label: "Description(optional)",
                                placeholder: "Describe your event",
                                field: desc,
                            }
                            Textarea {
                                placeholder: "Location(optional)",
                                label: "Add a location",
                                field: loc,
                            }
                            Input { label: "date", field: date, r#type: "date" }

                            Input {
                                label: "start",
                                field: start,
                                r#type: "time",
                            }
                            Input { label: "end", field: end, r#type: "time" }

                            Select::<entity::event::Weekday> {
                                label: "Choose Weekday when reocurring",
                                field: weekday,
                            }

                            CardActions {
                                SubmitButton {
                                    form: form_state.clone(),
                                    label: "Confirm Edits",
                                    submitting_label: "editing event",
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
