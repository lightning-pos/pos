use crate::{
    core::{
        commands::{app_service::AppService, Command},
        models::auth::user_model::{User, UserNewInput, UserUpdateInput},
        repositories::user_repository,
        types::db_uuid::DbUuid
    },
    error::{Error, Result},
};

pub struct AddUserCommand {
    pub user: UserNewInput,
}

pub struct UpdateUserCommand {
    pub user: UserUpdateInput,
}

pub struct DeleteUserCommand {
    pub id: DbUuid,
}

impl Command for AddUserCommand {
    type Output = User;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let username = &self.user.username;
        let user = user_repository::get_user_by_username(service, username).await?;

        if user.is_some() {
            return Err(Error::UniqueConstraintError);
        }

        user_repository::insert_user(service, self.user.clone()).await
    }
}

impl Command for UpdateUserCommand {
    type Output = User;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Check if user exists using SeaQuery
        let existing_user = user_repository::get_user_by_id(service, self.user.id).await?;

        if existing_user.is_none() {
            return Err(Error::NotFoundError);
        }

        user_repository::update_user(service, self.user.clone()).await
    }
}

impl Command for DeleteUserCommand {
    type Output = u64;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        user_repository::delete_user(service, self.id).await
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::core::{commands::tests::setup_service, models::auth::user_model::UserState};

    use super::*;

    #[tokio::test]
    async fn test_add_user_command() {
        let mut service = setup_service().await;
        let add_user_command = AddUserCommand {
            user: UserNewInput {
                username: "newuser".to_string(),
                pin_hash: "newpin".to_string(),
                full_name: "New User".to_string(),
                state: UserState::Active,
                last_login_at: None,
            },
        };

        let result = add_user_command.exec(&mut service).await;
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.username, "newuser");
        assert_eq!(user.full_name, "New User");
        assert_eq!(user.state, UserState::Active);
    }

    #[tokio::test]
    async fn test_update_user_command() {
        let mut service = setup_service().await;
        let add_user_command = AddUserCommand {
            user: UserNewInput {
                username: "updateuser".to_string(),
                pin_hash: "updatepin".to_string(),
                full_name: "Update User".to_string(),
                state: UserState::Active,
                last_login_at: None,
            },
        };
        let user = add_user_command.exec(&mut service).await.unwrap();

        let update_user_command = UpdateUserCommand {
            user: UserUpdateInput {
                id: user.id,
                full_name: Some("Updated User".to_string()),
                state: Some(UserState::Inactive),
                username: None,
                pin_hash: None,
                last_login_at: None,
            },
        };

        let result = update_user_command.exec(&mut service).await;
        assert!(result.is_ok());
        let updated_user = result.unwrap();
        assert_eq!(updated_user.id, user.id);
        assert_eq!(updated_user.full_name, "Updated User");
        assert_eq!(updated_user.state, UserState::Inactive);
    }

    #[tokio::test]
    async fn test_add_user_command_duplicate_username() {
        let mut service = setup_service().await;
        let add_user_command = AddUserCommand {
            user: UserNewInput {
                username: "testuser".to_string(),
                pin_hash: "testpin".to_string(),
                full_name: "Test User".to_string(),
                state: UserState::Active,
                last_login_at: None,
            },
        };

        let result = add_user_command.exec(&mut service).await;
        assert!(result.is_ok());

        // Try to add user with same username
        let duplicate_user_command = AddUserCommand {
            user: UserNewInput {
                username: "testuser".to_string(),
                pin_hash: "anotherpin".to_string(),
                full_name: "Another User".to_string(),
                state: UserState::Active,
                last_login_at: None,
            },
        };

        let result = duplicate_user_command.exec(&mut service).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_user_command_non_existent_user() {
        let mut service = setup_service().await;
        let non_existent_id = Uuid::now_v7().into();
        let update_user_command = UpdateUserCommand {
            user: UserUpdateInput {
                id: non_existent_id,
                full_name: Some("Updated User".to_string()),
                state: Some(UserState::Inactive),
                username: None,
                pin_hash: None,
                last_login_at: None,
            },
        };

        let result = update_user_command.exec(&mut service).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_user_command_partial_update() {
        let mut service = setup_service().await;
        let add_user_command = AddUserCommand {
            user: UserNewInput {
                username: "partialupdate".to_string(),
                pin_hash: "initialpin".to_string(),
                full_name: "Initial Name".to_string(),
                state: UserState::Active,
                last_login_at: None,
            },
        };
        let user = add_user_command.exec(&mut service).await.unwrap();

        let update_user_command = UpdateUserCommand {
            user: UserUpdateInput {
                id: user.id,
                full_name: Some("Updated Name".to_string()),
                state: None,
                username: None,
                pin_hash: None,
                last_login_at: None,
            },
        };

        let result = update_user_command.exec(&mut service).await;
        assert!(result.is_ok());
        let updated_user = result.unwrap();
        println!("Updated user: {:#?}", updated_user);
        assert_eq!(updated_user.id, user.id);
        assert_eq!(updated_user.full_name, "Updated Name");
        assert_eq!(updated_user.state, UserState::Active); // State should remain unchanged
        assert_eq!(updated_user.username, "partialupdate"); // Username should remain unchanged
    }

    #[tokio::test]
    async fn test_delete_user_command() {
        let mut service = setup_service().await;
        let add_user_command = AddUserCommand {
            user: UserNewInput {
                username: "deleteuser".to_string(),
                pin_hash: "deletepin".to_string(),
                full_name: "Delete User".to_string(),
                state: UserState::Active,
                last_login_at: None,
            },
        };
        let user = add_user_command.exec(&mut service).await.unwrap();

        let delete_user_command = DeleteUserCommand { id: user.id };
        let result = delete_user_command.exec(&mut service).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1); // 1 row affected

        // Attempt to delete the same user again
        let result = delete_user_command.exec(&mut service).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0); // 0 rows affected, user no longer exists
    }

    #[tokio::test]
    async fn test_delete_non_existent_user() {
        let mut service = setup_service().await;
        let non_existent_id = Uuid::now_v7().into();
        let delete_user_command = DeleteUserCommand {
            id: non_existent_id,
        };

        let result = delete_user_command.exec(&mut service).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0); // 0 rows affected, user doesn't exist
    }
}
