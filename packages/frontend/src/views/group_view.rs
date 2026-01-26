use crate::components::ui::button;
use crate::components::ui::groupcard::GroupCard;
use crate::{Route, components::ui::button::Button};
use api::routes::groups::{list_group_card_data, list_groups};
use dioxus::prelude::*;

#[component]
pub fn GroupView() -> Element {
    let groups = use_server_future(move || async move { list_groups().await })?;
    let number_of_groups = match &*groups.read_unchecked() {
        Some(Ok(groups)) => groups.len(),
        Some(Err(_)) => 0,
        None => 0,
    };
    let groups_data = use_server_future(move || async move {
        let mut result = Vec::new();

        for i in 0..number_of_groups {
            if let Ok(group) = list_group_card_data(i).await {
                result.push(group)
            }
        }
        result
    })?;

    rsx! {
        div {
            div {
                h1 { class: "text-2xl font-bold text-center relative ", "Your groups" }
                div { class: "absolute top-2 right-2",
                    Link { to: Route::NewGroup {},
                        Button {
                            variant: button::ButtonVariant::Primary,
                            ghost: false,
                            shape: button::ButtonShape::Round,
                            disabled: false,
                            "new group +"
                        }
                    }
                }
                match groups_data.read().as_ref() {
                    Some(group_list) => rsx! {
                        div { class: "space-y-4",
                            div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                                for group in group_list {
                                    GroupCard { data: group.clone() }
                                }
                            }
                        }
                    },
                    None => rsx! {
                        div { "Loading..." }
                    },
                }
            }
        }
    }
}
