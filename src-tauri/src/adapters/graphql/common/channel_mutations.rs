use juniper::FieldResult;

use crate::{
    core::{
        commands::{
            common::channel_commands::{
                CreateChannelCommand, DeleteChannelCommand, UpdateChannelCommand,
            },
            Command,
        },
        models::common::channel_model::{Channel, ChannelNewInput, ChannelUpdateInput},
        types::db_uuid::DbUuid,
    },
    AppState,
};

pub async fn create_channel(input: ChannelNewInput, context: &AppState) -> FieldResult<Channel> {
    let mut service = context.service.lock().await;
    let res = CreateChannelCommand { channel: input }.exec(&mut service).await?;
    Ok(res)
}

pub async fn update_channel(input: ChannelUpdateInput, context: &AppState) -> FieldResult<Channel> {
    let mut service = context.service.lock().await;
    let res = UpdateChannelCommand { channel: input }.exec(&mut service).await?;
    Ok(res)
}

pub async fn delete_channel(id: DbUuid, context: &AppState) -> FieldResult<i32> {
    let mut service = context.service.lock().await;
    let res = DeleteChannelCommand { id }.exec(&mut service).await?;
    Ok(res)
}
