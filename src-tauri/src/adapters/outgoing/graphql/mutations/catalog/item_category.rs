use crate::{
    core::{
        command::{
            catalog::item_category::{
                CreateItemCategoryCommand, DeleteItemCategoryCommand, UpdateItemCategoryCommand,
            },
            Command,
        },
        entities::catalog::item_category::{ItemCategory, NewItemCategory, UpdateItemCategory},
    },
    AppState,
};
use juniper::FieldResult;

pub fn create_item_category(
    new_category: NewItemCategory,
    context: &AppState,
) -> FieldResult<ItemCategory> {
    let mut service = context.service.lock().unwrap();
    let res = CreateItemCategoryCommand {
        category: new_category,
    }
    .exec(&mut service)?;
    Ok(res)
}

pub fn update_item_category(
    category: UpdateItemCategory,
    context: &AppState,
) -> FieldResult<ItemCategory> {
    let mut service = context.service.lock().unwrap();
    let res = UpdateItemCategoryCommand { category }.exec(&mut service)?;
    Ok(res)
}

pub fn delete_item_category(id: String, context: &AppState) -> FieldResult<i32> {
    let mut service = context.service.lock().unwrap();
    let res = DeleteItemCategoryCommand { id }.exec(&mut service)?;
    Ok(res)
}
