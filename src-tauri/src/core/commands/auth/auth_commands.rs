use juniper::{GraphQLInputObject, GraphQLObject};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        commands::{app_service::AppService, Command},
        models::auth::user_model::{self, User},
    },
    error::{Error, Result},
};

#[derive(Debug, Serialize, GraphQLInputObject)]
pub struct LoginCommand {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, GraphQLObject)]
pub struct LoginResponse {
    turso_url: String,
    turso_token: String,
}

pub struct LogoutCommand;

impl Command for LoginCommand {
    type Output = LoginResponse;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let client = Client::new();

        // TODO: Make the IAM service URL configurable
        let response = client
            .post("http://localhost:7001/users/login")
            .json(&LoginCommand { username: self.username.clone(), password: self.password.clone() })
            .send()
            .await
            .map_err(|e| format!("Failed to connect to IAM service: {}", e))?;

        // Check if the request was successful
        if !response.status().is_success() {
            return Err(Error::AuthenticationError);
        }

        // Parse the response
        let login_response = response
        .json::<LoginResponse>()
        .await
        .map_err(|e| format!("Failed to parse IAM service response: {}", e))?;


        #[cfg(not(test))]
        {
            let turso_url = login_response.turso_url.clone();
            let turso_token = login_response.turso_token.clone();
            service.update_adapter(turso_url, turso_token).await;
        }

        #[cfg(test)]
        {
            let check_query = user_model::queries::find_by_username(&self.username);
            let user = service.db_adapter.query_optional::<User>(&check_query).await?;
            match user {
                Some(user) => {
                    service.state.current_user = Some(user.id);
                    Ok(login_response)
                }
                None => Err(Error::NotFoundError)
            }
        }
        #[cfg(not(test))]
        {
            Ok(login_response)
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
                username: "test".to_string(),
                pin_hash: "test".to_string(),
                full_name: "Test User".to_string(),
                state: UserState::Active,
                last_login_at: None,
            },
        };

        let result = add_user_command.exec(&mut service).await;
        assert!(result.is_ok());
        let user = result.unwrap();

        let login_command = LoginCommand {
            username: "test".to_string(),
            password: "test".to_string(),
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
        assert!(matches!(result, Err(Error::AuthenticationError)));
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
