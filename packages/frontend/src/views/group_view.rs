use crate::Route;
use crate::components::ui::groupcard::GroupCard;
use api::routes::groups::list_groups;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::LdPlus;

#[component]
pub fn GroupView() -> Element {
    let groups = use_server_future(move || async move { list_groups().await })?;

    rsx! {
        div {
            h1 { class: "text-2xl font-bold text-center relative ", "Your groups" }
            Link {
                to: Route::NewGroup {},
                class: "fixed bottom-16 lg:bottom-4 right-4 btn btn-primary btn-circle lg:btn-lg",
                Icon { icon: LdPlus }
            }
            match &*groups.read() {
                Some(Ok(groups)) => rsx! {
                    div { class: "space-y-4 grid grid-cols-1 md:grid-cols-2 gap-6",
                        for group in groups.iter() {
                            GroupCard { key: "{group.id}", group_id: group.id }
                        }
                    }
                },
                Some(Err(error)) => rsx! {
                    p { class: "text-red-500", "Loading groups failed: {error}" }
                },
                None => rsx! {
                    p { "Loading..." }
                },
            }
        }

    }
}
