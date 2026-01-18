use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::LdCalendarPlus;
use dioxus_free_icons::icons::ld_icons::LdCircleAlert;

#[component]
pub fn List() -> Element {
    let mut button_active = use_signal(|| false);
    rsx! {
        ul { class: "list bg-base-100 rounded-box shadow-md",
            li { class: "p-4 pb-2 text-xs opacity-60 tracking-wide", "Header" }

            li { class: "list-row",
                div { "" } //TODO: find out how to add checkbox
                div {
                    div { "Task Titel" }
                    div { class: "text-xs font-semibold opacity-60", "Task Details" }
                }
                button {
                    class: "btn btn-ghost",
                    onclick: move |_| button_active.toggle(),
                    if button_active() {
                        Icon { class: "size-6 text-red-600", icon: LdCircleAlert } //Lucide clock-alert
                    } else {
                        Icon { class: "size-6", icon: LdCircleAlert }
                    }
                }
                button {
                    class: "btn btn-ghost",
                    onclick: move |_| button_active.toggle(),
                    Icon { class: "size-6", icon: LdCalendarPlus }
                }
                li { class: "list-row",
                    div { "" } //TODO: find out how to add checkbox
                    div {
                        div { "Task Titel 2" }
                        div { class: "text-xs font-semibold opacity-60", "Task Details 2" }
                    }
                    button {
                        class: "btn btn-ghost",
                        onclick: move |_| button_active.toggle(),
                        if button_active() {
                            Icon { class: "size-6 fill-red", icon: LdCircleAlert } //Lucide clock-alert
                        } else {
                            Icon { class: "size-6", icon: LdCircleAlert }
                        }
                    }
                    button {
                        class: "btn btn-ghost",
                        onclick: move |_| button_active.toggle(),
                        Icon { class: "size-6", icon: LdCalendarPlus }
                    }
                }
            }
        }
    }
}

//#[component]
//fn ColorchangeIcon()->
//let mut active = use_signal(|| false);
//rsx!{
//Icon{
// class: if active(){
//     "size-6 text-red-700"
// } else{
//    "size-6"
//},
// icon: LdCircleAlert,
// onclick: move |_| active.toggle()
// }
// }

//changes to red no matter what button you press - fix
