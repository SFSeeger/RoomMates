use crate::Route;
use crate::components::ui::{
    button::{Button, ButtonShape, ButtonVariant},
    card::{Card, CardActions, CardBody},
};
use dioxus::prelude::*;

#[component]
pub fn EventEditor(event_id: i32) -> Element {
    rsx! {
        " Edit Event"

        div {
            Card {
                CardBody {

                    div { class: "flex flex-col items-center gap-4 justify-center h-full",
                        p { class: "label", "Type Titel" }
                        input {
                            class: "input",
                            placeholder: " title", //change to be event specific
                            r#type: "text",
                        }
                        p { class: "label", "Reocurring Event" }
                        input {
                            checked: "checked",
                            class: "checkbox",
                            r#type: "checkbox",
                        }
                        p { class: "label", "Privat Event" }
                        input {
                            checked: "checked",
                            class: "checkbox",
                            r#type: "checkbox",
                        }
                        textarea {
                            class: "textarea",
                            placeholder: "Describe your event",
                        }
                        textarea {
                            class: "textarea",
                            placeholder: "Give your event a location",
                        }
                        input {
                            class: "input",
                            placeholder: " date", //change to be event specific
                            r#type: "date",
                        }
                        input {
                            class: "input",
                            placeholder: " start", //change to be event specific
                            r#type: "time",
                        }
                        input {
                            class: "input",
                            placeholder: " end", //change to be event specific
                            r#type: "time",
                        }
                        select { class: "select",
                            option { disabled: "false", selected: "false", "Weekday" }
                            option { "Monday" }
                            option { "Tuesday" }
                            option { "Wednesday" }
                            option { "Thursday" }
                            option { "Friday" }
                            option { "Saturday" }
                            option { "Sunday" }
                        }
                    }
                    CardActions {
                        Link { to: Route::Events {},
                            Button {
                                variant: ButtonVariant::Primary,
                                shape: ButtonShape::Wide,
                                "Submit"
                            }
                        }
                    }
                }
            }
        }
    }
}
