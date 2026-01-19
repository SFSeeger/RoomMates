use dioxus::prelude::*;
use crate::Route;
use form_hooks::use_form_field::{AnyField, FieldValue, FormField};
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{LdUsers, LdPlus, LdMinus};
use crate::components::ui::card::{Card, CardBody, CardTitle};
use form_hooks::use_form::{use_form, use_on_submit};
use form_hooks::use_form_field::use_form_field;
use form_hooks::validators;
use api::routes::groups::retrieve_group;
use crate::components::ui::form::input::Input;
use api::routes::groups::{GroupCardData, delete_user_from_group};
use crate::components::ui::sidebar::sidebar_provider::SidebarState;
use crate::components::ui::button::{Button, ButtonVariant, ButtonShape};
use crate::components::ui::list::{List, ListDetails, ListRow};
use crate::components::ui::toaster::{Toast, ToastVariant, ToasterState};
use entity::user;
use crate::components::ui::dialog::{Dialog, DialogAction, DialogContent, DialogTrigger};

#[component]
pub fn EditGroup(group_id: i32) -> Element {
    let mut group = use_loader(move || async move { retrieve_group(group_id).await })?;
    let group_data = group.read().clone();
    let mut form_state = use_form();
    let group_name = use_form_field("group name", String::new())
        .with_validator(validators::required("Group name is required!"));

    form_state.register_field(&group_name);
    
    let toaster = use_context::<ToasterState>();

    let onmemberremove = move |member_id: i32| { 
        group.write().members.retain(|member| member.id != member_id);
    };

    let onsubmitnewmember = move |_updated_group: entity::group::Entity| { 
        group.restart();
    };
    
    rsx!{
        div {
            h1 { class: "relative text-2xl font-bold text-center", "Edit your groups" }
            div { class: "flex w-full flex-col items-start md:flex-row",
                div { class: "Group-Events flex items-center grow justify-center",
                    Card { class: "w-full",
                        CardBody {
                            CardTitle { class: "flex items-center justify-center",
                                div { class: "flex justify-center w-full",
                                    div { class: "w-3/4",
                                        Input {
                                            field: group_name,
                                            label: "Group name",
                                            r#type: "text",
                                            class: "h-12 text-lg px-4",
                                            icon: {
                                                rsx! {
                                                    Icon { icon: LdUsers }
                                                }
                                            },
                                        }
                                    }
                                }
                            }
                            p { "Events" }
                        }
                    }
                }
                div { class: "Member-Sidecard gap-4 flex w-72", //ToDo: Button um Member hinzufügen
                    List { header: "Members",
                        if group_data.members.len() < 1 {
                            ListRow {
                                ListDetails { title: "No members yet" }
                            }
                        }
                        ListRow {
                            ListDetails { title: "",
                                for member in group_data.members {
                                    GroupListEntry {
                                        key: "{member.id}",
                                        member: member.clone(),
                                        group_id,
                                        onmemberremove,
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
pub fn GroupListEntry(member: entity::user::Model, group_id: i32, onmemberremove: EventHandler<i32>) -> Element{
    let toaster = use_context::<ToasterState>();
    let name = format!{"{}{}", member.first_name, member.last_name};
    rsx!{
        div { class: "flex justify-between items-center mb-2 w-full",
            span { class: "flex-1", "{name}" }
            Dialog {
                DialogTrigger {
                    variant: ButtonVariant::Primary,
                    shape: ButtonShape::Round,
                    ghost: false,
                    class: "btn-sm",
                    Icon { icon: LdMinus }
                }
                DialogContent { title: "Do you want to delete {name.clone()}?",
                    form { method: "dialog",
                        DialogAction {
                            Button { variant: ButtonVariant::Secondary, "Cancel" }
                            Button {
                                onclick: move |_| {
                                    let mut toaster_clone = toaster.clone();
                                    let name_clone = name.clone();
                                    async move {

                                        match delete_user_from_group.call((member.id, group_id)).await {
                                            Ok(_) => {
                                                toaster_clone
                                                    .toast(
                                                        Toast::new(
                                                            format!("Deleted {} successfully!", name_clone),
                                                            None,
                                                            true,
                                                            ToastVariant::Success,
                                                        ),
                                                    );
                                                onmemberremove.call(member.id);
                                            }
                                            Err(error) => {
                                                toaster_clone
                                                    .toast(
                                                        Toast::new(
                                                            format!("Failed to delete {}!", name_clone),
                                                            Some(rsx! {
                                                                span { "{error.to_string()}" }
                                                            }),
                                                            true,
                                                            ToastVariant::Error,
                                                        ),
                                                    );
                                            }
                                        }
                                    }
                                },
                                variant: ButtonVariant::Error,
                                "Delete"
                            }
                        }
                    }
                }
            }
        
        }
    }
}

