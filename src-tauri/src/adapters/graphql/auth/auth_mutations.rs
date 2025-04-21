use crate::{
    core::commands::{
        auth::auth_commands::{LoginCommand, LogoutCommand},
        Command,
    },
    AppState,
};
use juniper::FieldResult;

pub async fn login(username: String, password: String, context: &AppState) -> FieldResult<()> {
    let cmd = LoginCommand { username, password };
    let mut service = context.service.lock().await;
    let res = cmd.exec(&mut service).await?;
    Ok(res)
}

pub async fn logout(context: &AppState) -> FieldResult<()> {
    let mut service = context.service.lock().await;
    let res = LogoutCommand.exec(&mut service).await?;
    Ok(res)
}
