use crate::components::ui::button;
use crate::components::ui::card::{Card, CardActions, CardBody, CardTitle};
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
        Card {

            CardBody {
                {
                    rsx! {
                        CardTitle { "Profile Information" }

                        List_Info_Display {}

                        CardActions {
                            {
                                rsx! {
                                    Button {
                                        variant: button::ButtonVariant::Primary,
                                        ghost: false,
                                        shape: button::ButtonShape::Wide,
                                        disabled: false,
                                        "Confirm New Info"
                                    }
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
pub fn List_Info_Display() -> Element {
    let user: Resource<std::result::Result<entity::user::Model, ServerFnError>> =
        use_server_future(move || async move { retrieve_user(1).await })?;

    rsx! {
        match &*user.read() {
            Some(Ok(data)) => rsx! {
                ul { class: "list bg-base-100 rounded-box shadow-md",
                    li { class: "list-row", key: "{data.id}",
                        " Username: {data.first_name} {data.last_name}"

                        div {

                            fieldset { class: "fieldset",

                                legend { class: "fieldset-legend", "Edit first name!" }
                                input {
                                    class: "input",
                                    placeholder: " {data.first_name}",
                                    r#type: "text",
                                }
                                legend { class: "fieldset-legend", "Edit last name!" }

                                input {
                                    class: "input",
                                    placeholder: " {data.last_name}",
                                    r#type: "text",
                                }
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
}

// div {
//   label { class: "btn", r#for: "my_modal_6", "open modal" }
//   input { class: "modal-toggle", id: "my_modal_6", r#type: "checkbox" }
//   div { class: "modal", role: "dialog",
//      div { class: "modal-box",
//          div { class: "modal-action" }
