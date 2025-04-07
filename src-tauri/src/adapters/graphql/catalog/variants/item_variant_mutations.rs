use juniper::FieldResult;

use crate::core::commands::catalog::item_variant_commands::{
    AssignVariantValueCommand, CreateItemVariantCommand, DeleteItemVariantCommand,
    RemoveVariantValueCommand, UpdateItemVariantCommand,
};
use crate::core::commands::Command;
use crate::core::models::catalog::item_variant_model::{
    ItemVariant, ItemVariantNewInput, ItemVariantUpdateInput,
};
use crate::core::types::db_uuid::DbUuid;
use crate::AppState;

pub fn create_item_variant(
    input: ItemVariantNewInput,
    context: &AppState,
) -> FieldResult<ItemVariant> {
    let mut service = context.service.lock().unwrap();
    
    let command = CreateItemVariantCommand {
        item_variant: input,
    };
    
    let item_variant = command.exec(&mut service)?;
    Ok(item_variant)
}

pub fn update_item_variant(
    input: ItemVariantUpdateInput,
    context: &AppState,
) -> FieldResult<ItemVariant> {
    let mut service = context.service.lock().unwrap();
    
    let command = UpdateItemVariantCommand {
        item_variant: input,
    };
    
    let updated_item_variant = command.exec(&mut service)?;
    Ok(updated_item_variant)
}

pub fn delete_item_variant(id: DbUuid, context: &AppState) -> FieldResult<i32> {
    let mut service = context.service.lock().unwrap();
    
    let command = DeleteItemVariantCommand { id };
    
    match command.exec(&mut service) {
        Ok(count) => Ok(count as i32),
        Err(e) => Err(juniper::FieldError::new(
            format!("Failed to delete item variant: {}", e),
            juniper::Value::null(),
        )),
    }
}

pub fn assign_variant_value_to_item_variant(
    item_variant_id: DbUuid,
    variant_value_id: DbUuid,
    context: &AppState,
) -> FieldResult<i32> {
    let mut service = context.service.lock().unwrap();
    
    let command = AssignVariantValueCommand {
        item_variant_id,
        variant_value_id,
    };
    
    let result = command.exec(&mut service)?;
    Ok(result as i32)
}

pub fn remove_variant_value_from_item_variant(
    item_variant_id: DbUuid,
    variant_value_id: DbUuid,
    context: &AppState,
) -> FieldResult<i32> {
    let mut service = context.service.lock().unwrap();
    
    let command = RemoveVariantValueCommand {
        item_variant_id,
        variant_value_id,
    };
    
    let result = command.exec(&mut service)?;
    Ok(result as i32)
}
