use juniper::FieldResult;

use crate::core::commands::catalog::variant_value_commands::{
    CreateVariantValueCommand, DeleteVariantValueCommand, UpdateVariantValueCommand,
};
use crate::core::commands::Command;
use crate::core::models::catalog::variant_value_model::{
    VariantValue, VariantValueNewInput, VariantValueUpdateInput,
};
use crate::core::types::db_uuid::DbUuid;
use crate::AppState;

pub fn create_variant_value(
    input: VariantValueNewInput,
    context: &AppState,
) -> FieldResult<VariantValue> {
    let mut service = context.service.lock().unwrap();
    
    let command = CreateVariantValueCommand {
        variant_value: input,
    };
    
    let variant_value = command.exec(&mut service)?;
    Ok(variant_value)
}

pub fn update_variant_value(
    input: VariantValueUpdateInput,
    context: &AppState,
) -> FieldResult<VariantValue> {
    let mut service = context.service.lock().unwrap();
    
    let command = UpdateVariantValueCommand {
        variant_value: input,
    };
    
    let updated_variant_value = command.exec(&mut service)?;
    Ok(updated_variant_value)
}

pub fn delete_variant_value(id: DbUuid, context: &AppState) -> FieldResult<i32> {
    let mut service = context.service.lock().unwrap();
    
    let command = DeleteVariantValueCommand { id };
    
    match command.exec(&mut service) {
        Ok(count) => Ok(count as i32),
        Err(crate::error::Error::HasChildrenError) => Err(juniper::FieldError::new(
            "Cannot delete variant value that is used by item variants",
            juniper::Value::null(),
        )),
        Err(e) => Err(juniper::FieldError::new(
            format!("Failed to delete variant value: {}", e),
            juniper::Value::null(),
        )),
    }
}
