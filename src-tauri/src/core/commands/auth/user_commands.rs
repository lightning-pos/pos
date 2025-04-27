use chrono::Utc;
use sea_query::{Expr, Query};
use uuid::Uuid;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        commands::{app_service::AppService, Command},
        db::SeaQueryCrudTrait,
        models::auth::user_model::{
            User, UserNewInput, UserState, UserUpdateInput, Users,
        },
        types::db_uuid::DbUuid,
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
        let mut query_builder = Query::select();
        let check_query = query_builder
            .from(Users::Table)
            .column(Users::Id)
            .and_where(Expr::col(Users::Username).eq(username.clone()));

        let user = service.db_adapter.query_optional::<User>(&check_query).await?;

        if user.is_some() {
            return Err(Error::UniqueConstraintError);
        }

        // Create new user
        let now = Utc::now().naive_utc();
        let user_id: DbUuid = Uuid::now_v7().into();
        let pin_hash = self.user.pin.clone();

        // Create a new user instance
        let new_user = User {
            id: user_id,
            username: self.user.username.clone(),
            pin_hash,
            full_name: self.user.full_name.clone(),
            state: UserState::Active,
            last_login_at: None,
            created_at: now,
            updated_at: now,
        };

        // Generate and execute the insert query
        let insert_stmt = new_user.insert();
        service.db_adapter.insert_one::<User>(&insert_stmt).await?;

        Ok(new_user)
    }
}

impl Command for UpdateUserCommand {
    type Output = User;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Check if user exists using SeaQuery
        let mut query_builder = Query::select();
        let check_query = query_builder
            .from(Users::Table)
            .columns([Users::Id, Users::Username, Users::PinHash, Users::FullName, Users::State, Users::LastLoginAt, Users::CreatedAt, Users::UpdatedAt])
            .and_where(Expr::col(Users::Id).eq(self.user.id.to_string()));

        let existing_user = service.db_adapter.query_optional::<User>(&check_query).await?;

        if existing_user.is_none() {
            return Err(Error::NotFoundError);
        }

        // Get the existing user data and update it
        let mut user = existing_user.unwrap();
        let now = Utc::now().naive_utc();

        // Update fields if they exist
        if let Some(full_name) = &self.user.full_name {
            user.full_name = full_name.clone();
        }

        if let Some(state) = &self.user.state {
            user.state = state.clone();
        }

        if let Some(username) = &self.user.username {
            user.username = username.clone();
        }

        if let Some(pin) = &self.user.pin {
            user.pin_hash = pin.clone();
        }

        // Always update the updated_at timestamp
        user.updated_at = now;

        // Generate and execute the update query
        let update_stmt = user.update();
        service.db_adapter.update_one::<User>(&update_stmt).await?;

        // Retrieve the updated user
        let mut query_builder = Query::select();
        let select_query = query_builder
            .from(Users::Table)
            .columns([
                Users::Id,
                Users::Username,
                Users::PinHash,
                Users::FullName,
                Users::State,
                Users::LastLoginAt,
                Users::CreatedAt,
                Users::UpdatedAt,
            ])
            .and_where(Expr::col(Users::Id).eq(self.user.id.to_string()));

        let user = service.db_adapter.query_one::<User>(&select_query).await?;

        Ok(user)
    }
}

impl Command for DeleteUserCommand {
    type Output = i32;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let delete_stmt = User::delete_by_id(self.id);
        let affected_rows = service.db_adapter.delete(&delete_stmt).await?;

        Ok(affected_rows as i32)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::commands::tests::setup_service;

    use super::*;

    #[tokio::test]
    async fn test_add_user_command() {
        let mut service = setup_service().await;
        let add_user_command = AddUserCommand {
            user: UserNewInput {
                username: "newuser".to_string(),
                pin: "newpin".to_string(),
                full_name: "New User".to_string(),
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
                pin: "updatepin".to_string(),
                full_name: "Update User".to_string(),
            },
        };
        let user = add_user_command.exec(&mut service).await.unwrap();

        let update_user_command = UpdateUserCommand {
            user: UserUpdateInput {
                id: user.id,
                full_name: Some("Updated User".to_string()),
                state: Some(UserState::Inactive),
                username: None,
                pin: None,
            },
        };

        let result = update_user_command.exec(&mut service).await;
        println!("Update result: {:?}", result);
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
                pin: "testpin".to_string(),
                full_name: "Test User".to_string(),
            },
        };

        let result = add_user_command.exec(&mut service).await;
        assert!(result.is_ok());

        // Try to add user with same username
        let duplicate_user_command = AddUserCommand {
            user: UserNewInput {
                username: "testuser".to_string(),
                pin: "anotherpin".to_string(),
                full_name: "Another User".to_string(),
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
                pin: None,
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
                pin: "initialpin".to_string(),
                full_name: "Initial Name".to_string(),
            },
        };
        let user = add_user_command.exec(&mut service).await.unwrap();

        let update_user_command = UpdateUserCommand {
            user: UserUpdateInput {
                id: user.id,
                full_name: Some("Updated Name".to_string()),
                state: None,
                username: None,
                pin: None,
            },
        };

        let result = update_user_command.exec(&mut service).await;
        assert!(result.is_ok());
        let updated_user = result.unwrap();
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
                pin: "deletepin".to_string(),
                full_name: "Delete User".to_string(),
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
