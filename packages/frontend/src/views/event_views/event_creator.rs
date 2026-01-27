use crate::Route;
use crate::components::ui::card::CardTitle;
use crate::components::ui::{
    card::{Card, CardActions, CardBody},
    form::checkbox::Checkbox,
    form::input::Input,
    form::select::{EnumSelect, Select},
    form::submit_button::SubmitButton,
    form::textarea::Textarea,
};
use api::routes::events::create_event;
use chrono::{NaiveDate, NaiveTime};
use dioxus::prelude::*;
use entity::event::PartialEvModel;
use form_hooks::use_form::{use_form, use_on_submit};
use form_hooks::use_form_field::{FormField, use_form_field};
use form_hooks::validators;

#[component]
pub fn EventCreator() -> Element {
    let mut form_state = use_form();
    let mut create_action = use_action(create_event);

    let mut form_errors = use_signal(Vec::<String>::new);
    let mut _reocuring_enabled = use_signal(|| true);

    let title: FormField<String> = use_form_field("title", String::new())
        .with_validator(validators::required("event needs a title"));
    let reocurring: FormField<bool> = use_form_field("reocurring", false);
    let private: FormField<bool> = use_form_field("private", false);
    let desc: FormField<Option<String>> = use_form_field("description", None);
    let loc: FormField<Option<String>> = use_form_field("location", None);
    let date = use_form_field("date", NaiveDate::from_ymd_opt(2026, 1, 1).unwrap());
    let start = use_form_field("start_time", NaiveTime::from_hms_opt(8, 30, 0).unwrap());
    let end: FormField<NaiveTime> =
        use_form_field("end_time", NaiveTime::from_hms_opt(23, 00, 0).unwrap());
    let weekday = use_form_field("weekday", entity::event::Weekday::Monday);

    // let reoc_att = reocurring.field_attributes();
    //reoc_att.oninput = move |ev| reocuring_enabled.toggle();

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

        create_action.call(form_data).await;

        match create_action.value() {
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
                CardTitle { "Create Event" }

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
                                                        //class: if reocurring.clone().value.peek() { "disabled" } else { "" },
                            }

                            CardActions {
                                SubmitButton {
                                    form: form_state.clone(),
                                    label: "Create Event",
                                    submitting_label: "creating event",
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

impl EnumSelect for entity::event::Weekday {
    fn select_options() -> Vec<(String, &'static str)> {
        let options = vec![
            ("Monday".to_owned(), "Monday"),
            ("Tuesday".to_owned(), "Tuesday"),
            ("Wednesday".to_owned(), "Wednesday"),
            ("Thursday".to_owned(), "Thursday"),
            ("Friday".to_owned(), "Friday"),
            ("Saturday".to_owned(), "Saturday"),
            ("Sunday".to_owned(), "Sunday"),
        ];
        options
    }
}

//fix day select
//disable components
