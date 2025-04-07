use juniper::FieldResult;

use crate::core::commands::catalog::variant_type_commands::{
    CreateVariantTypeCommand, DeleteVariantTypeCommand, UpdateVariantTypeCommand,
};
use crate::core::commands::Command;
use crate::core::models::catalog::variant_type_model::{VariantType, VariantTypeNewInput, VariantTypeUpdateInput};
use crate::core::types::db_uuid::DbUuid;
use crate::AppState;

pub fn create_variant_type(
    input: VariantTypeNewInput,
    context: &AppState,
) -> FieldResult<VariantType> {
    let mut service = context.service.lock().unwrap();

    let command = CreateVariantTypeCommand {
        variant_type: input,
    };

    let variant_type = command.exec(&mut service)?;
    Ok(variant_type)
}

pub fn update_variant_type(
    input: VariantTypeUpdateInput,
    context: &AppState,
) -> FieldResult<VariantType> {
    let mut service = context.service.lock().unwrap();

    let command = UpdateVariantTypeCommand {
        variant_type: input,
    };

    let updated_variant_type = command.exec(&mut service)?;
    Ok(updated_variant_type)
}

pub fn delete_variant_type(id: DbUuid, context: &AppState) -> FieldResult<i32> {
    let mut service = context.service.lock().unwrap();

    let command = DeleteVariantTypeCommand { id };

    match command.exec(&mut service) {
        Ok(count) => Ok(count as i32),
        Err(crate::error::Error::HasChildrenError) => Err(juniper::FieldError::new(
            "Cannot delete variant type with associated values",
            juniper::Value::null(),
        )),
        Err(e) => Err(juniper::FieldError::new(
            format!("Failed to delete variant type: {}", e),
            juniper::Value::null(),
        )),
    }
}
