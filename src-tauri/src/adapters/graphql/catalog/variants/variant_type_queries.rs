use juniper::FieldResult;

use crate::core::commands::catalog::variant_type_commands::{
    GetVariantTypeCommand, ListVariantTypesCommand,
};
use crate::core::commands::Command;
use crate::core::models::catalog::variant_type_model::VariantType;
use crate::core::types::db_uuid::DbUuid;
use crate::AppState;

pub fn get_variant_type(id: DbUuid, context: &AppState) -> FieldResult<VariantType> {
    let mut service = context.service.lock().unwrap();

    let command = GetVariantTypeCommand { id };
    let variant_type = command.exec(&mut service)?;
    Ok(variant_type)
}

pub fn get_variant_types(
    first: Option<i32>,
    offset: Option<i32>,
    context: &AppState,
) -> FieldResult<Vec<VariantType>> {
    let mut service = context.service.lock().unwrap();

    let command = ListVariantTypesCommand;
    let variant_types = command.exec(&mut service)?;

    // Apply pagination in memory since our command doesn't support it directly
    let offset_val = offset.unwrap_or(0) as usize;
    let limit = first.map(|f| f as usize);

    let paginated_types = if let Some(limit) = limit {
        variant_types
            .into_iter()
            .skip(offset_val)
            .take(limit)
            .collect()
    } else {
        variant_types.into_iter().skip(offset_val).collect()
    };

    Ok(paginated_types)
}

pub fn get_total_variant_types(context: &AppState) -> FieldResult<i32> {
    let mut service = context.service.lock().unwrap();

    let command = ListVariantTypesCommand;
    let variant_types = command.exec(&mut service)?;
    Ok(variant_types.len() as i32)
}
