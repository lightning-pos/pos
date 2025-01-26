use crate::{
    core::commands::{
        auth::auth_commands::{LoginCommand, LogoutCommand},
        Command,
    },
    AppState,
};
use juniper::FieldResult;

pub fn login(username: String, password: String, context: &AppState) -> FieldResult<()> {
    let mut service = context.service.lock().unwrap();
    let res = LoginCommand { username, password }.exec(&mut service)?;
    Ok(res)
}

pub fn logout(context: &AppState) -> FieldResult<()> {
    let mut service = context.service.lock().unwrap();
    let res = LogoutCommand.exec(&mut service)?;
    Ok(res)
}
