use juniper::FieldResult;

use crate::core::commands::catalog::item_variant_commands::{
    GetItemVariantCommand, ListItemVariantsCommand,
};
use crate::core::commands::Command;
use crate::core::models::catalog::item_variant_model::ItemVariant;
use crate::core::types::db_uuid::DbUuid;
use crate::AppState;

pub fn get_item_variant(id: DbUuid, context: &AppState) -> FieldResult<ItemVariant> {
    let mut service = context.service.lock().unwrap();
    
    let command = GetItemVariantCommand { id };
    let item_variant = command.exec(&mut service)?;
    Ok(item_variant)
}

pub fn get_item_variants(
    item_id: Option<DbUuid>,
    first: Option<i32>,
    offset: Option<i32>,
    context: &AppState,
) -> FieldResult<Vec<ItemVariant>> {
    let mut service = context.service.lock().unwrap();
    
    let command = ListItemVariantsCommand { item_id };
    let item_variants = command.exec(&mut service)?;
    
    // Apply pagination in memory
    let offset_val = offset.unwrap_or(0) as usize;
    let limit = first.map(|f| f as usize);
    
    let paginated_variants = if let Some(limit) = limit {
        item_variants
            .into_iter()
            .skip(offset_val)
            .take(limit)
            .collect()
    } else {
        item_variants.into_iter().skip(offset_val).collect()
    };
    
    Ok(paginated_variants)
}
