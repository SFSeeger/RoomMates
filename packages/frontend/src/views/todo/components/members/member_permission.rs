use super::MembersContext;
use crate::components::contexts::use_auth;
use crate::components::tooltip::Tooltip;
use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::dialog::{
    Dialog, DialogAction, DialogContent, DialogTrigger, use_dialog,
};
use crate::components::ui::form::select::Select;
use crate::components::ui::form::submit_button::SubmitButton;
use crate::components::ui::toaster::{ToastOptions, use_toaster};
use crate::views::todo::todos_group::use_todo_list;
use api::routes::todo_list::invite::update_todo_list_invitation;
use dioxus::prelude::*;
use entity::todo_list_invitation::UpdateTodoListInvitation;
use entity::user::UserWithTodoListInvitation;
use form_hooks::prelude::{use_form_field, use_form_signal, use_on_submit};
use roommates::message_from_captured_error;

#[component]
pub fn MemberPermissionEditButton(member: UserWithTodoListInvitation) -> Element {
    let todo_list_context = use_todo_list();
    rsx! {
        if todo_list_context.permission().can_admin() {
            Dialog {
                Tooltip { tooltip: "Edit User Permission",
                    DialogTrigger {
                        variant: ButtonVariant::Accent,
                        outline: true,
                        class: "inline-block btn-xs",
                        "{member.invitation.permission}"
                    }
                }
                DialogContent {
                    title: "Edit User Permission for {member.first_name} {member.last_name}",
                    dismissible: false,
                    close_button: false,
                    EditMemberPermissionForm { member: member.clone() }
                }
            }
        } else {
            span { class: "badge badge-outline badge-accent badge-md", "{member.invitation.permission}" }
        }
    }
}

#[component]
fn EditMemberPermissionForm(member: UserWithTodoListInvitation) -> Element {
    let user = use_auth().user;
    let members_context = use_context::<MembersContext>();
    let todo_list_context = use_todo_list();
    let todo_list_id = todo_list_context.todo_list().id;
    let dialog = use_dialog();
    let mut toaster = use_toaster();

    let mut update_permission = use_action(move |data| async move {
        update_todo_list_invitation(todo_list_id, member.id, data).await
    });
    let form_signal = use_form_signal();
    let mut form_state = form_signal();
    let permission = use_form_field("permission", member.invitation.permission);
    form_state.register_field(&permission);
    let onsubmit = use_on_submit(&form_state, move |mut form_state| async move {
        let data: UpdateTodoListInvitation = form_state.parsed_values().unwrap();
        update_permission.call(data).await;
        match update_permission.value() {
            Some(Ok(updated_member)) => {
                toaster.success("Permission Updated", ToastOptions::new());
                // Our current user permissions might have changed, so we need to reload the To-Do List
                if let Some(user) = user.peek().as_ref()
                    && user.id == member.id
                {
                    todo_list_context.update_todo_list();
                }
                members_context.update_member(updated_member());
                form_state.mark_clean();
            }
            Some(Err(error)) => {
                toaster.error(
                    "Failed to update permission",
                    ToastOptions::new()
                        .description(rsx! { "{message_from_captured_error(&error)}" }),
                );
                form_state.reset();
            }
            None => {
                warn!("Request to update permission did not finish");
            }
        }
        dialog.close();
    });
    form_state.revalidate();

    rsx! {
        form { onsubmit,
            Select { label: "Permission", field: permission.clone() }
            DialogAction {
                Button {
                    variant: ButtonVariant::Secondary,
                    r#type: "button",
                    class: "grow",
                    onclick: move |_| {
                        dialog.close();
                        form_signal().reset();
                    },
                    "Cancel"
                }
                SubmitButton { form: form_signal() }
            }
        }
    }
}
