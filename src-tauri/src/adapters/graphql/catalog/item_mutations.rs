use crate::{
    core::{
        commands::{
            catalog::item_commands::{CreateItemCommand, DeleteItemCommand, UpdateItemCommand},
            Command,
        },
        models::catalog::item_model::{Item, NewItem, UpdateItem},
        types::db_uuid::DbUuid,
    },
    AppState,
};
use juniper::FieldResult;

pub async fn create_item(item: NewItem, context: &AppState) -> FieldResult<Item> {
    let mut service = context.service.lock().await;
    let res = CreateItemCommand { item }.exec(&mut service).await?;
    Ok(res)
}

pub async fn update_item(item: UpdateItem, context: &AppState) -> FieldResult<Item> {
    let mut service = context.service.lock().await;
    let res = UpdateItemCommand { item }.exec(&mut service).await?;
    Ok(res)
}

pub async fn delete_item(id: DbUuid, context: &AppState) -> FieldResult<i32> {
    let mut service = context.service.lock().await;
    let res = DeleteItemCommand { id }.exec(&mut service).await?;
    Ok(res)
}
