use sea_query::{Expr, Query};

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        commands::{app_service::AppService, Command},
        models::auth::user_model::{User, Users},
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
        // Build the query using sea-query
        let mut query_builder = Query::select();
        let query = query_builder
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
            .and_where(Expr::col(Users::Username).eq(self.username.clone()));

        // Execute the query using the database adapter
        let user = service.db_adapter.query_optional::<User>(&query).await?;

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
            models::auth::user_model::UserNewInput,
        },
        error::Error,
    };

    #[tokio::test]
    async fn test_login_command() {
        let mut service = setup_service();
        let add_user_command = AddUserCommand {
            user: UserNewInput {
                username: "testuser".to_string(),
                pin: "password".to_string(),
                full_name: "Test User".to_string(),
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
        let mut service = setup_service();
        service.state.current_user = Some(Uuid::now_v7().into());

        let logout_command = LogoutCommand;
        let result = logout_command.exec(&mut service).await;

        assert!(result.is_ok());
        assert_eq!(service.state.current_user, None);
    }
}
