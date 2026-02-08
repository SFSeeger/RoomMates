use crate::server;
use dioxus::{fullstack::NoContent, prelude::*};
use entity::{links::FriendEvents, prelude::*};

#[cfg(feature = "server")]
use dioxus::server::axum::Extension;
use entity::invitation::InvitationStatus;

//TODO maybe optimize with joined queries, to not load event and user seperatley
#[get("/api/invitations", ext: Extension<server::AppState>, auth: Extension<server::AuthenticationState>)]
pub async fn list_received_invites() -> Result<Vec<entity::invitation::Model>, ServerFnError> {
    use sea_orm::ModelTrait;
    use sea_orm::{ColumnTrait, QueryFilter};
    let user = auth.user.as_ref().or_unauthorized("Not authenticated")?;

    let invites = user
        .find_related(Invitation)
        .filter(entity::invitation::Column::Status.eq(InvitationStatus::Sent))
        .all(&ext.database)
        .await
        .or_internal_server_error("failed to retrieve invitations")?;

    Ok(invites)
}

#[post("/api/invitations", ext: Extension<server::AppState>)]
pub async fn send_invite(
    reciever_mail: String,
    event_id: i32,
) -> Result<entity::invitation::Model, ServerFnError> {
    use sea_orm::ActiveModelTrait;
    use sea_orm::Set;
    use sea_orm::TryIntoModel;
    use server::auth::find_user_by_email;

    let user = find_user_by_email(reciever_mail, &ext.database).await?;

    let invite = entity::invitation::ActiveModel {
        status: Set(InvitationStatus::Sent),
        recieving_user: Set(user.id),
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

#[post("/api/invitations/{invitation_id}/accept", ext: Extension<server::AppState>)]
pub async fn accept_invite(invitation_id: i32) -> Result<NoContent, ServerFnError> {
    use sea_orm::ActiveModelTrait;
    use sea_orm::EntityTrait;
    use sea_orm::IntoActiveModel;
    use sea_orm::Set;

    let invite = Invitation::find_by_id(invitation_id)
        .one(&ext.database)
        .await
        .or_internal_server_error("Error loading invitation")?
        .or_not_found("cant find invitation")?;

    let link = entity::shared_friend_event::ActiveModel {
        user_id: Set(invite.recieving_user),
        event_id: Set(invite.event_id),
    };

    entity::shared_friend_event::Entity::insert(link)
        .exec(&ext.database)
        .await
        .or_internal_server_error("error adding to shared events")?;

    let mut active_invite = invite.into_active_model();

    active_invite.status = Set(InvitationStatus::Accepted);

    active_invite
        .save(&ext.database)
        .await
        .or_internal_server_error("Error updating invite status")?;

    Ok(NoContent)
}

#[post("/api/invitations/{invitation_id}/delete", ext: Extension<server::AppState>)]
pub async fn decline_invite(invitation_id: i32) -> Result<NoContent, ServerFnError> {
    use sea_orm::ActiveModelTrait;
    use sea_orm::EntityTrait;
    use sea_orm::IntoActiveModel;
    use sea_orm::Set;

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

#[get("/api/invitations/events", ext: Extension<server::AppState>, auth: Extension<server::AuthenticationState>)]
pub async fn list_shared_friend_events() -> Result<Vec<entity::event::Model>, ServerFnError> {
    use sea_orm::ModelTrait;
    let user = auth.user.as_ref().or_unauthorized("Not authenticated")?;

    let events = user
        .find_linked(FriendEvents)
        .all(&ext.database)
        .await
        .or_internal_server_error("failed to retrieve invitations")?;

    Ok(events)
}
