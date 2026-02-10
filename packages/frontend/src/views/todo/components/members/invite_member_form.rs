use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::dialog::{DialogAction, use_dialog};
use crate::components::ui::form::input::Input;
use crate::components::ui::form::select::Select;
use crate::components::ui::form::submit_button::SubmitButton;
use crate::components::ui::toaster::{ToastOptions, use_toaster};
use crate::views::todo::todos_group::use_todo_list;
use api::routes::todo_list::invite::{InviteToTodoListData, invite_to_todo_list};
use api::routes::users::EMAIL_REGEX;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::LdMail;
use entity::todo_list_invitation::InvitationPermission;
use form_hooks::prelude::{use_form, use_form_field, use_on_submit};
use form_hooks::validators;
use frontend::message_from_captured_error;
use regex::Regex;

#[component]
pub fn InviteMemberForm(onmemberinvited: EventHandler<()>) -> Element {
    let dialog = use_dialog();
    let mut toaster = use_toaster();
    let todo_list_context = use_todo_list();
    let todo_list_id = todo_list_context.todo_list().id;

    let mut invite_user =
        use_action(move |data| async move { invite_to_todo_list(todo_list_id, data).await });

    let mut form_error = use_signal(|| None);

    let mut form_state = use_form();
    let email = use_form_field("email", String::new()).with_validator(validators::pattern(
        Regex::new(EMAIL_REGEX)?,
        "Email must be valid!",
    ));
    let permission = use_form_field("permission", InvitationPermission::Read);
    form_state.register_field(&email);
    form_state.register_field(&permission);

    form_state.revalidate();

    let mut form_state_clone = form_state.clone();

    let onsubmit = use_on_submit(&form_state, move |mut form_state| async move {
        let data: InviteToTodoListData = form_state.parsed_values().unwrap();
        invite_user.call(data).await;
        match invite_user.value().as_ref() {
            Some(Ok(_)) => {
                toaster.success("Invited User successfully", ToastOptions::new());
                dialog.close();
                form_error.set(None);
                form_state.reset();
                onmemberinvited.call(());
            }
            Some(Err(error)) => {
                let message = message_from_captured_error(error);
                form_error.set(Some(message));
            }
            None => {
                warn!("Invite user request did not finish!");
            }
        }
    });

    rsx! {
        form { onsubmit,
            div {
                class: "alert alert-error",
                class: if form_error().is_some() { "visible" } else { "invisible" },
                role: "alert",
                "Error inviting user: "
                if let Some(error) = form_error() {
                    "{error}"
                }
            }
            Input {
                label: "Email",
                icon: rsx! {
                    Icon { icon: LdMail }
                },
                field: email,
            }
            Select { label: "Permission", field: permission }
            DialogAction {
                Button {
                    onclick: move |_| {
                        dialog.close();
                        form_error.set(None);
                        form_state_clone.reset();
                    },
                    variant: ButtonVariant::Secondary,
                    r#type: "button",
                    class: "grow",
                    "Cancel"
                }
                SubmitButton { form: form_state.clone() }
            }
        }
    }
}
