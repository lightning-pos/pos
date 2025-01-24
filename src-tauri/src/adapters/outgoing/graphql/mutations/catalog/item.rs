use crate::{
    core::{
        command::{
            catalog::item::{CreateItemCommand, DeleteItemCommand, UpdateItemCommand},
            Command,
        },
        entities::catalog::item::{Item, NewItem, UpdateItem},
        types::db_uuid::DbUuid,
    },
    AppState,
};
use juniper::FieldResult;

pub fn create_item(item: NewItem, context: &AppState) -> FieldResult<Item> {
    let mut service = context.service.lock().unwrap();
    let res = CreateItemCommand { item }.exec(&mut service)?;
    Ok(res)
}

pub fn update_item(item: UpdateItem, context: &AppState) -> FieldResult<Item> {
    let mut service = context.service.lock().unwrap();
    let res = UpdateItemCommand { item }.exec(&mut service)?;
    Ok(res)
}

pub fn delete_item(id: DbUuid, context: &AppState) -> FieldResult<i32> {
    let mut service = context.service.lock().unwrap();
    let res = DeleteItemCommand { id }.exec(&mut service)?;
    Ok(res)
}
