use crate::components::ui::button;
//use crate::components::ui::card::{Card, CardBody};
use crate::{Route, components::ui::button::Button};
use api::routes::users::get_me;
use dioxus::prelude::*;
//use dioxus_free_icons::Icon;
//use dioxus_free_icons::icons::ld_icons::LdCircleHelp;

#[component]
pub fn Profile() -> Element {
    let user =
        use_server_future(move || async move { get_me().await })?;

    rsx! {

        div { class: "flex flex-col items-center gap-4 justify-center h-full",
            div { class: "avatar",
                div { class: "w-24 rounded",
                    img { src: "https://img.daisyui.com/images/profile/demo/yellingcat@192.webp" }
                }
            }
            //Icon { class: "size-30", icon: LdCircleHelp }
            h1 { class: "text-2xl font-bold text-center", "you reached your profile, success" }
            Link { to: Route::Home {},
                Button {
                    variant: button::ButtonVariant::Primary,
                    ghost: false,
                    shape: button::ButtonShape::Wide,
                    disabled: false,
                    "Homepage"
                }
            }
        }
        div { class: "divider divider-primar" }

        //div { children: CardBody {}, Card {} } figure out how to do card with component!!
        // how to turn off auto format
        div { class: "card card-border bg-base-100 shadow-sm",
            div { class: "card-body",
                h2 { class: "card-title", "Profile Information" }
                p { "" }

                match &*user.read() {
                    Some(Ok(data)) => rsx! {
                        ul { class: "list bg-base-100 rounded-box shadow-md",

                            li { class: "list-row", key: "{data.id}",

                                " First Name: {data.first_name} "

                                div {
                                    fieldset { class: "fieldset",
                                        legend { class: "fieldset-legend", "Edit first name!" }
                                        input {
                                            class: "input",
                                            placeholder: "firstname",
                                            r#type: "text",
                                        }
                                    }
                                }
                            }
                            li { class: "list-row", key: "{data.id}",

                                " Last Name: {data.last_name}"

                                div {
                                    fieldset { class: "fieldset",
                                        legend { class: "fieldset-legend", "Edit last name" }

                                        input { class: "input", placeholder: "lastname", r#type: "text" }
                                    }
                                }
                            }
                            li { class: "list-row", key: "{data.id}",
                                "Email: {data.email}"
                                div {

                                    fieldset { class: "fieldset",
                                        legend { class: "fieldset-legend", "Edit your email!" }

                                        input {
                                            class: "input",
                                            placeholder: "new email",
                                            r#type: "text",
                                        }
                                    }
                                }
                            }
                        }
                    },
                    Some(Err(err)) => rsx! {
                        p { class: "text-red-500", "Loading Events failed with {err}" }
                    },
                    None => rsx! {
                        p { "cant connect to db" }
                    },
                }
            }
            div { class: "card-actions justify-end",
                Button {
                    variant: button::ButtonVariant::Primary,
                    ghost: false,
                    shape: button::ButtonShape::Wide,
                    disabled: false,
                    //how to add attributes
                    "Confirm New Info"
                }
            }
        }
        div {
            label { class: "btn", r#for: "my_modal_6", "open modal" }
            input { class: "modal-toggle", id: "my_modal_6", r#type: "checkbox" }
            div { class: "modal", role: "dialog",
                div { class: "modal-box",
                    div { class: "modal-action" }
                }
            }
        }
    }
}
