use dioxus::fullstack::HttpError;
use dioxus::prelude::{OrHttpError, ServerFnError, error};
use entity::prelude::TodoListInvitation;
use entity::todo_list_invitation::Column as InviteColumn;
use sea_orm::ModelTrait;
use sea_orm::QueryFilter;
use sea_orm::{ColumnTrait, DbErr};
use sea_orm::{DatabaseConnection, EntityTrait};

async fn find_active_todo_list_invitation(
    todo_list_id: i32,
    user_id: i32,
    database: &DatabaseConnection,
) -> Result<Option<entity::todo_list_invitation::Model>, DbErr> {
    TodoListInvitation::find()
        .filter(InviteColumn::ReceivingUserId.eq(user_id))
        .filter(InviteColumn::TodoListId.eq(todo_list_id))
        .filter(InviteColumn::IsAccepted.eq(true))
        .one(database)
        .await
        .inspect_err(|e| error!("{e}"))
}
async fn find_todo_list_invitation(
    todo_list_id: i32,
    user_id: i32,
    database: &DatabaseConnection,
) -> Result<Option<entity::todo_list_invitation::Model>, DbErr> {
    TodoListInvitation::find()
        .filter(InviteColumn::ReceivingUserId.eq(user_id))
        .filter(InviteColumn::TodoListId.eq(todo_list_id))
        .one(database)
        .await
        .inspect_err(|e| error!("{e}"))
}

pub(crate) async fn get_todo_list_permission(
    todo_list_id: i32,
    user_id: i32,
    database: &DatabaseConnection,
) -> Result<Option<entity::todo_list_invitation::InvitationPermission>, ServerFnError> {
    let invitation = find_active_todo_list_invitation(todo_list_id, user_id, database)
        .await
        .or_internal_server_error("Failed to load todo list invitation")?;

    Ok(invitation.map(|inv| inv.permission))
}

pub(crate) async fn remove_user_from_todo_list(
    todo_list_id: i32,
    user_id: i32,
    request_user_id: i32,
    database: &DatabaseConnection,
) -> Result<(), HttpError> {
    (request_user_id != user_id).or_bad_request("Cannot remove yourself from todo list")?;

    get_todo_list_permission(todo_list_id, request_user_id, database)
        .await?
        .or_forbidden("Unauthorized to remove user")?
        .can_admin()
        .or_forbidden("Unauthorized to remove user")?;

    let invitation = find_todo_list_invitation(todo_list_id, user_id, database)
        .await
        .or_internal_server_error("Failed to retrieve invite")?
        .or_not_found("Cannot remove user from todo list")?;

    invitation
        .delete(database)
        .await
        .or_internal_server_error("Failed to remove user from todo list")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server;
    use sea_orm::{ActiveModelTrait, ActiveValue};
    use tokio;

    async fn setup() -> DatabaseConnection {
        let database = server::database::establish_connection().await.unwrap();
        database
            .get_schema_registry("entity::*")
            .sync(&database)
            .await
            .unwrap();
        database
    }

    // region: Factories
    async fn create_test_user(database: &DatabaseConnection, id: i32) -> entity::user::Model {
        entity::user::ActiveModel {
            id: sea_orm::Set(id),
            email: sea_orm::Set(format!("test{id}@test.de")),
            first_name: sea_orm::Set(format!("firstname{id}")),
            last_name: sea_orm::Set(format!("firstname{id}")),
            password: sea_orm::Set("test".to_string()),
        }
        .insert(database)
        .await
        .unwrap()
    }

    async fn create_test_todo_list(
        database: &DatabaseConnection,
        id: i32,
        created_by_id: i32,
    ) -> entity::todo_list::Model {
        let todo_list = entity::todo_list::ActiveModel {
            id: sea_orm::Set(id),
            title: sea_orm::Set(format!("Test Todo List {id}")),
            description: ActiveValue::default(),
            created_by_id: ActiveValue::Set(Some(created_by_id)),
        }
        .insert(database)
        .await
        .unwrap();

        create_test_todo_list_invitation(
            database,
            todo_list.id,
            created_by_id,
            created_by_id,
            true,
            entity::todo_list_invitation::InvitationPermission::Admin,
        )
        .await;

        todo_list
    }

    async fn create_test_todo_list_invitation(
        database: &DatabaseConnection,
        todo_list_id: i32,
        receiver: i32,
        sender: i32,
        accepted: bool,
        permission: entity::todo_list_invitation::InvitationPermission,
    ) -> entity::todo_list_invitation::Model {
        entity::todo_list_invitation::ActiveModel {
            todo_list_id: sea_orm::Set(todo_list_id),
            receiving_user_id: sea_orm::Set(receiver),
            permission: sea_orm::Set(permission),
            is_accepted: sea_orm::Set(accepted),
            is_favorite: sea_orm::Set(false),
            sender_user_id: sea_orm::Set(Some(sender)),
        }
        .insert(database)
        .await
        .unwrap()
    }
    // endregion

    #[tokio::test]
    async fn test_get_todo_list_permission() {
        let database = setup().await;
        let user1 = create_test_user(&database, 1).await;
        let user2 = create_test_user(&database, 2).await;
        let user3 = create_test_user(&database, 3).await;
        let todo_list = create_test_todo_list(&database, 1, user1.id).await;
        let _todo_list_invitation = create_test_todo_list_invitation(
            &database,
            todo_list.id,
            user2.id,
            user1.id,
            true,
            entity::todo_list_invitation::InvitationPermission::Read,
        )
        .await;
        let _todo_list_invitation2 = create_test_todo_list_invitation(
            &database,
            todo_list.id,
            user3.id,
            user1.id,
            false,
            entity::todo_list_invitation::InvitationPermission::Write,
        )
        .await;

        let owner_permission = get_todo_list_permission(todo_list.id, user1.id, &database)
            .await
            .unwrap();
        assert_eq!(
            owner_permission,
            Some(entity::todo_list_invitation::InvitationPermission::Admin),
            "Expected owner to have admin permission"
        );

        let invited_permission = get_todo_list_permission(todo_list.id, user2.id, &database)
            .await
            .unwrap();
        assert_eq!(
            invited_permission,
            Some(entity::todo_list_invitation::InvitationPermission::Read),
            "Expected invited user with read permission to have read permission"
        );
        let invited_permission = get_todo_list_permission(todo_list.id, user3.id, &database)
            .await
            .unwrap();
        assert_eq!(
            invited_permission, None,
            "Expected user with pending invitation to have no permission"
        );

        let non_invited_permission = get_todo_list_permission(todo_list.id, 999, &database)
            .await
            .unwrap();
        assert_eq!(
            non_invited_permission, None,
            "Expected non-invited user to have no permission"
        );
    }

    #[tokio::test]
    async fn test_remove_user_from_todo_list() {
        let database = setup().await;
        let user1 = create_test_user(&database, 1).await;
        let user2 = create_test_user(&database, 2).await;
        let user3 = create_test_user(&database, 3).await;
        let todo_list = create_test_todo_list(&database, 1, user1.id).await;
        create_test_todo_list_invitation(
            &database,
            todo_list.id,
            user2.id,
            user1.id,
            true,
            entity::todo_list_invitation::InvitationPermission::Read,
        )
        .await;
        create_test_todo_list_invitation(
            &database,
            todo_list.id,
            user3.id,
            user1.id,
            true,
            entity::todo_list_invitation::InvitationPermission::Admin,
        )
        .await;

        let result = remove_user_from_todo_list(todo_list.id, user1.id, user2.id, &database).await;
        assert!(
            result.is_err(),
            "Expected non-admin user to not be able to remove another user"
        );

        let result = remove_user_from_todo_list(todo_list.id, user1.id, user1.id, &database).await;
        assert!(
            result.is_err(),
            "Expected user to not be able to remove themselves"
        );

        let result = remove_user_from_todo_list(todo_list.id, user2.id, user1.id, &database).await;
        assert!(
            result.is_ok(),
            "Expected owner to be able to remove invited user"
        );

        let invitation = find_active_todo_list_invitation(todo_list.id, user2.id, &database)
            .await
            .unwrap();
        assert!(
            invitation.is_none(),
            "Expected invitation to be removed after removing user from todo list"
        );

        let result = remove_user_from_todo_list(todo_list.id, user1.id, user3.id, &database).await;
        assert!(
            result.is_ok(),
            "Expected admin user to be able to remove owner from todo list"
        );
    }
}
