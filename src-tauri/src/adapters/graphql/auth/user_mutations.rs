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

pub async fn add_user(user: UserNewInput, context: &AppState) -> FieldResult<User> {
    let cmd = AddUserCommand { user };
    let mut service = context.service.lock().await;
    let res = cmd.exec(&mut service).await?;
    Ok(res)
}

pub async fn update_user(user: UserUpdateInput, context: &AppState) -> FieldResult<User> {
    let cmd = UpdateUserCommand { user };
    let mut service = context.service.lock().await;
    let res = cmd.exec(&mut service).await?;
    Ok(res)
}

pub async fn delete_user(id: DbUuid, context: &AppState) -> FieldResult<i32> {
    let cmd = DeleteUserCommand { id };
    let mut service = context.service.lock().await;
    let res = cmd.exec(&mut service).await?;
    Ok(res)
}
