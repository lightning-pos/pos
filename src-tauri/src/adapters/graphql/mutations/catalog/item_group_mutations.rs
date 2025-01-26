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

pub fn create_item_category(category: ItemGroupNew, context: &AppState) -> FieldResult<ItemGroup> {
    let mut service = context.service.lock().unwrap();
    let res = CreateItemGroupCommand { category }.exec(&mut service)?;
    Ok(res)
}

pub fn update_item_category(
    category: ItemGroupUpdate,
    context: &AppState,
) -> FieldResult<ItemGroup> {
    let mut service = context.service.lock().unwrap();
    let res = UpdateItemGroupCommand { category }.exec(&mut service)?;
    Ok(res)
}

pub fn delete_item_category(id: DbUuid, context: &AppState) -> FieldResult<i32> {
    let mut service = context.service.lock().unwrap();
    let res = DeleteItemGroupCommand { id }.exec(&mut service)?;
    Ok(res)
}
