use crate::{
    core::{
        commands::{
            catalog::item_group_commands::{
                CreateItemGroupCommand, DeleteItemGroupCommand, UpdateItemGroupCommand,
            },
            Command,
        },
        models::catalog::item_group_model::{ItemCategory, ItemCategoryNew, ItemCategoryUpdate},
        types::db_uuid::DbUuid,
    },
    AppState,
};
use juniper::FieldResult;

pub async fn create_item_category(category: ItemCategoryNew, context: &AppState) -> FieldResult<ItemCategory> {
    let mut service = context.service.lock().await;
    let res = CreateItemGroupCommand { category }.exec(&mut service).await?;
    Ok(res)
}

pub async fn update_item_category(
    category: ItemCategoryUpdate,
    context: &AppState,
) -> FieldResult<ItemCategory> {
    let mut service = context.service.lock().await;
    let res = UpdateItemGroupCommand { category }.exec(&mut service).await?;
    Ok(res)
}

pub async fn delete_item_category(id: DbUuid, context: &AppState) -> FieldResult<i32> {
    let mut service = context.service.lock().await;
    let res = DeleteItemGroupCommand { id }.exec(&mut service).await?;
    Ok(res)
}
