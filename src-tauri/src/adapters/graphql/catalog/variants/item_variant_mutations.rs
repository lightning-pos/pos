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

pub async fn create_item_variant(
    input: ItemVariantNewInput,
    context: &AppState,
) -> FieldResult<ItemVariant> {
    let mut service = context.service.lock().await;

    let command = CreateItemVariantCommand {
        item_variant: input,
    };

    let item_variant = command.exec(&mut service).await?;
    Ok(item_variant)
}

pub async fn update_item_variant(
    input: ItemVariantUpdateInput,
    context: &AppState,
) -> FieldResult<ItemVariant> {
    let mut service = context.service.lock().await;

    let command = UpdateItemVariantCommand {
        item_variant: input,
    };

    let updated_item_variant = command.exec(&mut service).await?;
    Ok(updated_item_variant)
}

pub async fn delete_item_variant(id: DbUuid, context: &AppState) -> FieldResult<i32> {
    let mut service = context.service.lock().await;

    let command = DeleteItemVariantCommand { id };

    match command.exec(&mut service).await {
        Ok(count) => Ok(count as i32),
        Err(e) => Err(juniper::FieldError::new(
            format!("Failed to delete item variant: {}", e),
            juniper::Value::null(),
        )),
    }
}

pub async fn assign_variant_value_to_item_variant(
    item_variant_id: DbUuid,
    variant_value_id: DbUuid,
    context: &AppState,
) -> FieldResult<i32> {
    let mut service = context.service.lock().await;

    let command = AssignVariantValueCommand {
        item_variant_id,
        variant_value_id,
    };

    let result = command.exec(&mut service).await?;
    Ok(result as i32)
}

pub async fn remove_variant_value_from_item_variant(
    item_variant_id: DbUuid,
    variant_value_id: DbUuid,
    context: &AppState,
) -> FieldResult<i32> {
    let mut service = context.service.lock().await;

    let command = RemoveVariantValueCommand {
        item_variant_id,
        variant_value_id,
    };

    let result = command.exec(&mut service).await?;
    Ok(result as i32)
}
