use juniper::FieldResult;

use crate::core::commands::catalog::variant_value_commands::{
    GetVariantValueCommand, ListVariantValuesCommand,
};
use crate::core::commands::Command;
use crate::core::models::catalog::variant_value_model::VariantValue;
use crate::core::types::db_uuid::DbUuid;
use crate::AppState;

pub async fn get_variant_value(id: DbUuid, context: &AppState) -> FieldResult<VariantValue> {
    let mut service = context.service.lock().await;

    let command = GetVariantValueCommand { id };
    let variant_value = command.exec(&mut service).await?;
    Ok(variant_value)
}

pub async fn get_variant_values(
    variant_type_id: Option<DbUuid>,
    first: Option<i32>,
    offset: Option<i32>,
    context: &AppState,
) -> FieldResult<Vec<VariantValue>> {
    let mut service = context.service.lock().await;

    let command = ListVariantValuesCommand { variant_type_id };
    let variant_values = command.exec(&mut service).await?;

    // Apply pagination in memory
    let offset_val = offset.unwrap_or(0) as usize;
    let limit = first.map(|f| f as usize);

    let paginated_values = if let Some(limit) = limit {
        variant_values
            .into_iter()
            .skip(offset_val)
            .take(limit)
            .collect()
    } else {
        variant_values.into_iter().skip(offset_val).collect()
    };

    Ok(paginated_values)
}
