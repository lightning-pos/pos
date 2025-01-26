use crate::{
    core::{
        commands::{
            auth::user_commands::{AddUserCommand, DeleteUserCommand, UpdateUserCommand},
            Command,
        },
        models::auth::user_model::{User, UserNewInput, UserUpdateInput},
        types::db_uuid::DbUuid,
    },
    AppState,
};
use juniper::FieldResult;

pub fn add_user(user: UserNewInput, context: &AppState) -> FieldResult<User> {
    let mut service = context.service.lock().unwrap();
    let res = AddUserCommand { user }.exec(&mut service)?;
    Ok(res)
}

pub fn update_user(user: UserUpdateInput, context: &AppState) -> FieldResult<User> {
    let mut service = context.service.lock().unwrap();
    let res = UpdateUserCommand { user }.exec(&mut service)?;
    Ok(res)
}

pub fn delete_user(id: DbUuid, context: &AppState) -> FieldResult<i32> {
    let mut service = context.service.lock().unwrap();
    let res = DeleteUserCommand { id }.exec(&mut service)?;
    Ok(res)
}
