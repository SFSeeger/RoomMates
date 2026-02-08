use crate::server;
use dioxus::{fullstack::NoContent, prelude::*};
use entity::prelude::*;

#[cfg(feature = "server")]
use dioxus::server::axum::Extension;
use entity::invitation::InvitationStatus;
use sea_orm::{ActiveValue::Set, EntityTrait, IntoActiveModel};

#[get("/api/invitations", ext: Extension<server::AppState>, auth: Extension<server::AuthenticationState>)]
pub async fn list_invites() -> Result<Vec<entity::invitation::Model>, ServerFnError> {
    use sea_orm::ModelTrait;

    let user = auth.user.as_ref().or_unauthorized("Not authenticated")?;

    let invites = user
        .find_related(Invitation)
        .all(&ext.database)
        .await
        .or_internal_server_error("failed to retrieve invitations")?;

    Ok(invites)
}

#[post("/api/invitations", ext: Extension<server::AppState>)]
pub async fn send_invite(
    reciever: i32,
    event_id: i32,
) -> Result<entity::invitation::Model, ServerFnError> {
    use sea_orm::ActiveModelTrait;
    use sea_orm::TryIntoModel;

    let invite = entity::invitation::ActiveModel {
        status: Set(InvitationStatus::Sent),
        recieving_user: Set(reciever),
        event_id: Set(event_id),
        ..Default::default()
    };

    let result = invite
        .save(&ext.database)
        .await
        .or_internal_server_error("couldnt send invitation")?;

    Ok(result
        .try_into_model()
        .or_internal_server_error("Failed to convert active model to model")?)
}

#[post("/api/invitations/{invitation_id}", ext: Extension<server::AppState>)]
pub async fn accept_invite(invitation_id: i32) -> Result<NoContent, ServerFnError> {
    use sea_orm::ActiveModelTrait;

    let invite = Invitation::find_by_id(invitation_id)
        .one(&ext.database)
        .await
        .or_internal_server_error("Error loading invitation")?
        .or_not_found("cant find invitation")?;

    let mut active_invite = invite.into_active_model();

    active_invite.status = Set(InvitationStatus::Accepted);

    //TODO create new shared friend event
    active_invite
        .save(&ext.database)
        .await
        .or_internal_server_error("Error updating invite status")?;

    Ok(NoContent)
}

#[post("/api/invitations/{invitation_id}", ext: Extension<server::AppState>)]
pub async fn decline_invite(invitation_id: i32) -> Result<NoContent, ServerFnError> {
    use sea_orm::ActiveModelTrait;

    let invite = Invitation::find_by_id(invitation_id)
        .one(&ext.database)
        .await
        .or_internal_server_error("Error loading invitation")?
        .or_not_found("cant find invitation")?;

    let mut active_invite = invite.into_active_model();

    active_invite.status = Set(InvitationStatus::Declined);

    active_invite
        .save(&ext.database)
        .await
        .or_internal_server_error("Error updating invite status")?;

    Ok(NoContent)
}
