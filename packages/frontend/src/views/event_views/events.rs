use crate::Route;
use crate::components::ui::fieldset::Fieldset;
use crate::components::ui::list::{List, ListRow};
use crate::components::ui::toaster::{Toast, ToastVariant, ToasterState};
use crate::components::ui::{
    button::{Button, ButtonShape, ButtonVariant},
    card::{Card, CardBody, CardTitle},
};
use api::routes::events::{delete_event, list_events};
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{LdPencil, LdPlus, LdTrash};

//use dioxus_free_icons::Icon;
//use dioxus_free_icons::icons::ld_icons::LdCircleHelp;
// div { class: "flex flex-col items-center gap-4 justify-center h-full" }

#[component]
pub fn Events() -> Element {
    rsx! {
        div {
            Link { to: Route::EventCreator {},
                Button { variant: ButtonVariant::Primary, shape: ButtonShape::Wide,
                    Icon { icon: LdPlus }
                    "create new event"
                }
            }
        }
        div { class: "divider" }
        div { class: "flex w-full",
            Card {
                CardBody { Event_List {} }
            }
        }
    }
}

#[component]
pub fn Event_List() -> Element {
    let mut events = use_loader(move || async move { list_events().await })?;

    let ondelete = move |id: i32| {
        let mut lists_write = events.write();
        lists_write.retain(|list| list.id != id);
    };

    rsx! {
        List { header: "Your Events",
            for event in events.iter() {
                Event_List_Component { event: event.clone(), ondelete }
            }
        }
    }
}

#[component]
pub fn Event_List_Component(event: entity::event::Model, ondelete: EventHandler<i32>) -> Element {
    let mut delete_action: Action<(i32,), dioxus_fullstack::NoContent> = use_action(delete_event);
    let toaster = use_context::<ToasterState>();
    let title = event.title.clone();

    rsx! {
        ListRow { key: "{event.id}",
            Card {
                CardTitle { "{event.title}" }
                CardBody {
                    div { class: "flex flex-row gap-4 justify-content full",
                        div {
                            match &event.reocurring {
                                true => rsx! {
                                    h1 { "{event.weekday:?}" }
                                },
                                false => rsx! {
                                    h1 { "Date: {event.date}" }
                                },
                            }
                            p { "begins at: {event.start_time}" }
                            p { "ends at: {event.end_time}" }
                        }
                        div {
                            p {
                                match &event.private {
                                    true => rsx! { "Event is privat" },
                                    false => rsx! { "Event is public" },
                                }
                            }
                            p {
                                match &event.reocurring {
                                    true => rsx! { "Event is reocurring" },
                                    false => rsx! { "Event happens once" },
                                }
                            }
                        }
                        div {
                            match &event.description {
                                Some(Text) => rsx! {
                                    Card { class: "w-100 bg-base-100 card-s", "{Text}" }
                                },
                                None => rsx! {
                                    Card { class: "w-50 bg-base-100 card-s", "no description" }
                                },
                            }
                        }
                        div {
                            match &event.location {
                                Some(Text) => rsx! {
                                    Card { class: "w-50 bg-base-100 card-s", "{Text}" }
                                },
                                None => rsx! {
                                    Card { class: "w-50 bg-base-100 card-s", "no location" }
                                },
                            }
                        }
                        div {
                            Fieldset {
                                Link {
                                    to: Route::EventEditor {
                                        event_id: event.id,
                                    },
                                    Button { variant: ButtonVariant::Info,
                                        Icon { icon: LdPencil }
                                        "Edit"
                                    }
                                }
                                Button {
                                    onclick: move |_| {
                                        let mut toaster_clone = toaster.clone();
                                        let title_clone = title.clone();
                                        async move {
                                            delete_action.call(event.id).await;
                                            match delete_action.value() {
                                                Some(Ok(_)) => {
                                                    toaster_clone
                                                        .toast(
                                                            Toast::new(
                                                                format!("Deleted {} successfully!", title_clone),
                                                                None,
                                                                true,
                                                                ToastVariant::Success,
                                                            ),
                                                        );
                                                    ondelete.call(event.id);

                                                }
                                                Some(Err(_)) => {
                                                    warn!("failed to delete event!");

                                                }
                                                None => {
                                                    warn!("Request did not finish!");
                                                }
                                            }
                                        }
                                    },
                                    variant: ButtonVariant::Error,
                                    Icon { icon: LdTrash }
                                    "Delete"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
/*Some(Err(err)) => rsx! {
    p { class: "text-red-500", "Loading Events failed with {err}" }
},
None => rsx! {
    p { "cant connect to db" }
},*/
