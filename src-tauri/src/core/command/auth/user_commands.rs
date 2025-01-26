use chrono::Utc;
use diesel::{
    query_dsl::methods::FilterDsl, ExpressionMethods, OptionalExtension, RunQueryDsl,
    SelectableHelper,
};
use uuid::Uuid;

use crate::{
    core::{
        command::{app_service::AppService, Command},
        entities::auth::user::{
            User, UserNewInput, UserState, UserUpdateChangeset, UserUpdateInput,
        },
        types::db_uuid::DbUuid,
    },
    error::{Error, Result},
    schema::users,
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
        // check if username exists
        let username = &self.user.username;
        let user = users::table
            .filter(users::username.eq(username))
            .get_result::<User>(&mut service.conn)
            .optional()?;

        if user.is_some() {
            return Err(Error::UniqueConstraintError);
        }

        let pin_hash = self.user.pin.clone();
        let new_user = User {
            id: Uuid::now_v7().into(),
            username: self.user.username.clone(),
            pin_hash,
            full_name: self.user.full_name.clone(),
            state: UserState::Active,
            last_login_at: None,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        let user = diesel::insert_into(users::table)
            .values(&new_user)
            .get_result::<User>(&mut service.conn)?;

        Ok(user)
    }
}

impl Command for UpdateUserCommand {
    type Output = User;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        users::table
            .filter(users::id.eq(&self.user.id))
            .get_result::<User>(&mut service.conn)?;

        let user = UserUpdateChangeset {
            id: self.user.id,
            full_name: self.user.full_name.clone(),
            state: self.user.state.clone(),
            username: self.user.username.clone(),
            pin_hash: self.user.pin.clone(),
            updated_at: Utc::now().naive_utc(),
        };

        let user = diesel::update(users::table.filter(users::id.eq(&self.user.id)))
            .set(&user)
            .returning(User::as_returning())
            .get_result(&mut service.conn)?;

        Ok(user)
    }
}

impl Command for DeleteUserCommand {
    type Output = i32;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let res = diesel::delete(users::table.filter(users::id.eq(&self.id)))
            .execute(&mut service.conn)?;
        Ok(res as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_user_command() {
        let mut service = AppService::new(":memory:");
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
        let mut service = AppService::new(":memory:");
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
        let mut service = AppService::new(":memory:");
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
        let mut service = AppService::new(":memory:");
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
        let mut service = AppService::new(":memory:");
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
        let mut service = AppService::new(":memory:");
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
        let mut service = AppService::new(":memory:");
        let non_existent_id = Uuid::now_v7().into();
        let delete_user_command = DeleteUserCommand {
            id: non_existent_id,
        };

        let result = delete_user_command.exec(&mut service);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0); // 0 rows affected, user doesn't exist
    }
}
