use crate::{
    core::{
        commands::{
            catalog::item_group_commands::{
                CreateItemGroupCommand, DeleteItemGroupCommand, UpdateItemGroupCommand,
            },
            Command,
        },
        models::catalog::item_group_model::{ItemGroup, ItemGroupNew, ItemGroupUpdate},
        types::db_uuid::DbUuid,
    },
    AppState,
};
use juniper::FieldResult;

pub async fn create_item_category(category: ItemGroupNew, context: &AppState) -> FieldResult<ItemGroup> {
    let mut service = context.service.lock().await;
    let res = CreateItemGroupCommand { category }.exec(&mut service).await?;
    Ok(res)
}

pub async fn update_item_category(
    category: ItemGroupUpdate,
    context: &AppState,
) -> FieldResult<ItemGroup> {
    let mut service = context.service.lock().await;
    let res = UpdateItemGroupCommand { category }.exec(&mut service).await?;
    Ok(res)
}

pub async fn delete_item_category(id: DbUuid, context: &AppState) -> FieldResult<i32> {
    let mut service = context.service.lock().await;
    let res = DeleteItemGroupCommand { id }.exec(&mut service).await?;
    Ok(res)
}
