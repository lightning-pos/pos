use juniper::FieldResult;

use crate::{
    core::{
        commands::{
            catalog::discount_commands::{
                CreateDiscountCommand, DeleteDiscountCommand, UpdateDiscountCommand,
            },
            Command,
        },
        models::catalog::discount_model::{Discount, DiscountNewInput, DiscountUpdateInput},
        types::db_uuid::DbUuid,
    },
    AppState,
};

/// Create a new discount
pub async fn create_discount(discount: DiscountNewInput, context: &AppState) -> FieldResult<Discount> {
    let mut service = context.service.lock().await;
    let result = CreateDiscountCommand { discount }.exec(&mut service).await?;
    Ok(result)
}

/// Update an existing discount
pub async fn update_discount(discount: DiscountUpdateInput, context: &AppState) -> FieldResult<Discount> {
    let mut service = context.service.lock().await;
    let result = UpdateDiscountCommand { discount }.exec(&mut service).await?;
    Ok(result)
}

/// Delete a discount by its ID
pub async fn delete_discount(id: DbUuid, context: &AppState) -> FieldResult<i32> {
    let mut service = context.service.lock().await;
    let result = DeleteDiscountCommand { id }.exec(&mut service).await?;
    Ok(result as i32)
}
