use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods, OptionalExtension, RunQueryDsl};

use crate::{
    core::{
        command::{app_service::AppService, Command},
        entities::auth::user::User,
    },
    error::{Error, Result},
    schema::users,
};

pub struct LoginCommand {
    pub username: String,
    pub password: String,
}

pub struct LogoutCommand;

impl Command for LoginCommand {
    type Output = ();

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let user = users::table
            .filter(users::username.eq(&self.username))
            .get_result::<User>(&mut service.conn)
            .optional()?;

        match user {
            Some(user) => {
                service.state.current_user = Some(user.id);
                return Ok(());
            }
            None => return Err(Error::NotFoundError),
        }
    }
}

impl Command for LogoutCommand {
    type Output = ();

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.state.current_user = None;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::{
        core::{
            command::{
                app_service::AppService,
                auth::{
                    auth_commands::{LoginCommand, LogoutCommand},
                    user_commands::AddUserCommand,
                },
                Command,
            },
            entities::auth::user::UserNewInput,
        },
        error::Error,
    };

    #[test]
    fn test_login_command() {
        let mut service = AppService::new(":memory:");
        let add_user_command = AddUserCommand {
            user: UserNewInput {
                username: "testuser".to_string(),
                pin: "password".to_string(),
                full_name: "Test User".to_string(),
            },
        };

        let result = add_user_command.exec(&mut service);
        assert!(result.is_ok());
        let user = result.unwrap();

        let login_command = LoginCommand {
            username: "testuser".to_string(),
            password: "password".to_string(),
        };

        let result = login_command.exec(&mut service);
        assert!(result.is_ok());
        assert!(service.state.current_user.is_some());
        assert_eq!(service.state.current_user.unwrap(), user.id);

        // Test login with non-existent user
        let invalid_login = LoginCommand {
            username: "nonexistent".to_string(),
            password: "password".to_string(),
        };
        let result = invalid_login.exec(&mut service);
        assert!(matches!(result, Err(Error::NotFoundError)));
    }

    #[test]
    fn test_logout_command() {
        let mut service = AppService::new(":memory:");
        service.state.current_user = Some(Uuid::now_v7().into());

        let logout_command = LogoutCommand;
        let result = logout_command.exec(&mut service);

        assert!(result.is_ok());
        assert_eq!(service.state.current_user, None);
    }
}
