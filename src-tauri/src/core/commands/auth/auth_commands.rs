use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        commands::{app_service::AppService, Command},
        models::auth::user_model::{self, User},
    },
    error::{Error, Result},
};

pub struct LoginCommand {
    pub username: String,
    pub password: String,
}

pub struct LogoutCommand;

impl Command for LoginCommand {
    type Output = ();

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let check_query = user_model::queries::find_by_username(&self.username);
        let user = service.db_adapter.query_optional::<User>(&check_query).await?;

        match user {
            Some(user) => {
                service.state.current_user = Some(user.id);
                Ok(())
            }
            None => Err(Error::NotFoundError)
        }
    }
}

impl Command for LogoutCommand {
    type Output = ();

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.state.current_user = None;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::{
        core::{
            commands::{
                auth::{
                    auth_commands::{LoginCommand, LogoutCommand},
                    user_commands::AddUserCommand,
                }, tests::setup_service, Command
            },
            models::auth::user_model::{UserNewInput, UserState},
        },
        error::Error,
    };

    #[tokio::test]
    async fn test_login_command() {
        let mut service = setup_service().await;
        let add_user_command = AddUserCommand {
            user: UserNewInput {
                username: "testuser".to_string(),
                pin_hash: "password".to_string(),
                full_name: "Test User".to_string(),
                state: UserState::Active,
                last_login_at: None,
            },
        };

        let result = add_user_command.exec(&mut service).await;
        assert!(result.is_ok());
        let user = result.unwrap();

        let login_command = LoginCommand {
            username: "testuser".to_string(),
            password: "password".to_string(),
        };

        let result = login_command.exec(&mut service).await;
        println!("result: {:#?}", result);
        assert!(result.is_ok());
        assert!(service.state.current_user.is_some());
        assert_eq!(service.state.current_user.unwrap(), user.id);

        // Test login with non-existent user
        let invalid_login = LoginCommand {
            username: "nonexistent".to_string(),
            password: "password".to_string(),
        };
        let result = invalid_login.exec(&mut service).await;
        assert!(matches!(result, Err(Error::NotFoundError)));
    }

    #[tokio::test]
    async fn test_logout_command() {
        let mut service = setup_service().await;
        service.state.current_user = Some(Uuid::now_v7().into());

        let logout_command = LogoutCommand;
        let result = logout_command.exec(&mut service).await;

        assert!(result.is_ok());
        assert_eq!(service.state.current_user, None);
    }
}
