use chrono::Utc;
// Diesel imports no longer needed
use sea_query::{Expr, Query, SqliteQueryBuilder};
use uuid::Uuid;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        commands::{app_service::AppService, Command},
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

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Check if username exists using SeaQuery
        let username = &self.user.username;

        // Build the query to check for existing username
        let check_query = Query::select()
            .from(Users::Table)
            .column(Users::Id)
            .and_where(Expr::col(Users::Username).eq(username.clone()))
            .to_string(SqliteQueryBuilder);

        // Execute the query
        let user = service.db_adapter.query_optional::<User>(&check_query, vec![])?;

        if user.is_some() {
            return Err(Error::UniqueConstraintError);
        }

        // Create new user
        let now = Utc::now().naive_utc();
        let user_id: DbUuid = Uuid::now_v7().into();
        let pin_hash = self.user.pin.clone();

        // Build the insert query
        let insert_query = Query::insert()
            .into_table(Users::Table)
            .columns([
                Users::Id,
                Users::Username,
                Users::PinHash,
                Users::FullName,
                Users::State,
                Users::CreatedAt,
                Users::UpdatedAt,
            ])
            .values_panic([
                user_id.to_string().into(),
                self.user.username.clone().into(),
                pin_hash.clone().into(),
                self.user.full_name.clone().into(),
                UserState::Active.to_string().into(),
                now.to_string().into(),
                now.to_string().into(),
            ])
            .to_string(SqliteQueryBuilder);

        // Execute the insert query
        service.db_adapter.execute(&insert_query, vec![])?;

        // Create and return the new user object
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

        Ok(new_user)
    }
}

impl Command for UpdateUserCommand {
    type Output = User;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Check if user exists using SeaQuery
        let check_query = Query::select()
            .from(Users::Table)
            .columns([Users::Id])
            .and_where(Expr::col(Users::Id).eq(self.user.id.to_string()))
            .to_string(SqliteQueryBuilder);

        let existing_user = service.db_adapter.query_optional::<User>(&check_query, vec![])?;

        if existing_user.is_none() {
            return Err(Error::NotFoundError);
        }

        // Build update query with SeaQuery
        let now = Utc::now().naive_utc();

        // Create the update query
        let mut query = Query::update();
        query.table(Users::Table)
            .value(Users::UpdatedAt, now.to_string());

        // Add optional fields if they exist
        if let Some(full_name) = &self.user.full_name {
            query.value(Users::FullName, full_name.clone());
        }

        if let Some(state) = &self.user.state {
            query.value(Users::State, state.to_string());
        }

        if let Some(username) = &self.user.username {
            query.value(Users::Username, username.clone());
        }

        if let Some(pin) = &self.user.pin {
            query.value(Users::PinHash, pin.clone());
        }

        // Add WHERE condition
        query.and_where(Expr::col(Users::Id).eq(self.user.id.to_string()));

        // Generate the SQL query
        let sql = query.to_string(SqliteQueryBuilder);

        // Execute the update
        service.db_adapter.execute(&sql, vec![])?;

        // Retrieve the updated user
        let select_query = Query::select()
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
            .and_where(Expr::col(Users::Id).eq(self.user.id.to_string()))
            .to_string(SqliteQueryBuilder);

        let user = service.db_adapter.query_one::<User>(&select_query, vec![])?;

        Ok(user)
    }
}

impl Command for DeleteUserCommand {
    type Output = i32;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Build delete query with SeaQuery
        let delete_query = Query::delete()
            .from_table(Users::Table)
            .and_where(Expr::col(Users::Id).eq(self.id.to_string()))
            .to_string(SqliteQueryBuilder);

        // Execute the delete query
        let affected_rows = service.db_adapter.execute(&delete_query, vec![])?;

        Ok(affected_rows as i32)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::commands::tests::setup_service;

    use super::*;

    #[test]
    fn test_add_user_command() {
        let mut service = setup_service();
        let add_user_command = AddUserCommand {
            user: UserNewInput {
                username: "newuser".to_string(),
                pin: "newpin".to_string(),
                full_name: "New User".to_string(),
            },
        };

        let result = add_user_command.exec(&mut service);
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.username, "newuser");
        assert_eq!(user.full_name, "New User");
        assert_eq!(user.state, UserState::Active);
    }

    #[test]
    fn test_update_user_command() {
        let mut service = setup_service();
        let add_user_command = AddUserCommand {
            user: UserNewInput {
                username: "updateuser".to_string(),
                pin: "updatepin".to_string(),
                full_name: "Update User".to_string(),
            },
        };
        let user = add_user_command.exec(&mut service).unwrap();

        let update_user_command = UpdateUserCommand {
            user: UserUpdateInput {
                id: user.id,
                full_name: Some("Updated User".to_string()),
                state: Some(UserState::Inactive),
                username: None,
                pin: None,
            },
        };

        let result = update_user_command.exec(&mut service);
        assert!(result.is_ok());
        let updated_user = result.unwrap();
        assert_eq!(updated_user.id, user.id);
        assert_eq!(updated_user.full_name, "Updated User");
        assert_eq!(updated_user.state, UserState::Inactive);
    }

    #[test]
    fn test_add_user_command_duplicate_username() {
        let mut service = setup_service();
        let add_user_command = AddUserCommand {
            user: UserNewInput {
                username: "testuser".to_string(),
                pin: "testpin".to_string(),
                full_name: "Test User".to_string(),
            },
        };

        let result = add_user_command.exec(&mut service);
        assert!(result.is_ok());

        // Try to add user with same username
        let duplicate_user_command = AddUserCommand {
            user: UserNewInput {
                username: "testuser".to_string(),
                pin: "anotherpin".to_string(),
                full_name: "Another User".to_string(),
            },
        };

        let result = duplicate_user_command.exec(&mut service);
        assert!(result.is_err());
    }

    #[test]
    fn test_update_user_command_non_existent_user() {
        let mut service = setup_service();
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

        let result = update_user_command.exec(&mut service);
        assert!(result.is_err());
    }

    #[test]
    fn test_update_user_command_partial_update() {
        let mut service = setup_service();
        let add_user_command = AddUserCommand {
            user: UserNewInput {
                username: "partialupdate".to_string(),
                pin: "initialpin".to_string(),
                full_name: "Initial Name".to_string(),
            },
        };
        let user = add_user_command.exec(&mut service).unwrap();

        let update_user_command = UpdateUserCommand {
            user: UserUpdateInput {
                id: user.id,
                full_name: Some("Updated Name".to_string()),
                state: None,
                username: None,
                pin: None,
            },
        };

        let result = update_user_command.exec(&mut service);
        assert!(result.is_ok());
        let updated_user = result.unwrap();
        assert_eq!(updated_user.id, user.id);
        assert_eq!(updated_user.full_name, "Updated Name");
        assert_eq!(updated_user.state, UserState::Active); // State should remain unchanged
        assert_eq!(updated_user.username, "partialupdate"); // Username should remain unchanged
    }

    #[test]
    fn test_delete_user_command() {
        let mut service = setup_service();
        let add_user_command = AddUserCommand {
            user: UserNewInput {
                username: "deleteuser".to_string(),
                pin: "deletepin".to_string(),
                full_name: "Delete User".to_string(),
            },
        };
        let user = add_user_command.exec(&mut service).unwrap();

        let delete_user_command = DeleteUserCommand { id: user.id };
        let result = delete_user_command.exec(&mut service);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1); // 1 row affected

        // Attempt to delete the same user again
        let result = delete_user_command.exec(&mut service);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0); // 0 rows affected, user no longer exists
    }

    #[test]
    fn test_delete_non_existent_user() {
        let mut service = setup_service();
        let non_existent_id = Uuid::now_v7().into();
        let delete_user_command = DeleteUserCommand {
            id: non_existent_id,
        };

        let result = delete_user_command.exec(&mut service);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0); // 0 rows affected, user doesn't exist
    }
}
