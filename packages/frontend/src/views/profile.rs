use crate::components::ui::button;
use crate::{Route, components::ui::button::Button};
use api::routes::users::retrieve_user;
use dioxus::prelude::*;
//use dioxus_free_icons::Icon;
//use dioxus_free_icons::icons::ld_icons::LdCircleHelp;

#[component]
pub fn Profile() -> Element {
    let user: Resource<std::result::Result<entity::user::Model, ServerFnError>> =
        use_server_future(move || async move { retrieve_user(1).await })?;

    rsx! {

        div { class: "flex flex-col items-center gap-4 justify-center h-full",
            div { class: "avatar",
                div { class: "w-24 rounded",
                    img { src: "https://img.daisyui.com/images/profile/demo/yellingcat@192.webp" }
                }
            }
            //Icon { class: "size-30", icon: LdCircleHelp }
            h1 { class: "text-2xl font-bold text-center", "you reached your profile, success" }
            Link { class: "btn btn-lg btn-outline", to: Route::Home {}, "Homepage" }
        }
        div { class: "card",
            div { class: "card-body",
                h2 { class: "card-title", "Profile Information" }
                p { "View and Edit your Info" }

                match &*user.read() {
                    Some(Ok(data)) => rsx! {
                        p { "user information" }
                        ul { class: "list bg-base-100 rounded-box shadow-md",
                            li { class: "list-row", "Username: {data.id}" }
                            li { class: "list-row", "Email: {data.email}" }
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
                    shape: button::ButtonShape::Square,
                    disabled: false,
                    "Edit Profile"
                }
            }
        }
    }
}

#[component]
pub fn Profile_Editor() -> Element {
    rsx! {

        label { class: "btn", "open modal" }
        input { class: "modal-toggle", id: "my_modal_6", r#type: "checkbox" }
        div { class: "modal", role: "dialog",
            div { class: "modal-box",
                div { class: "modal-action" }
            }
        }

    }
}
