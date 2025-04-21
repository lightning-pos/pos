use juniper::FieldResult;

use crate::{
    core::{
        commands::{catalog::discount_commands::{GetDiscountCommand, ListDiscountsCommand}, Command},
        models::catalog::discount_model::{Discount, DiscountState},
        types::db_uuid::DbUuid,
    },
    AppState,
};

/// Fetch a list of all discounts, with optional pagination
pub async fn discounts(
    first: Option<i32>,
    offset: Option<i32>,
    state: Option<DiscountState>,
    context: &AppState,
) -> FieldResult<Vec<Discount>> {
    let mut service = context.service.lock().await;

    // Using the command pattern
    let result = ListDiscountsCommand.exec(&mut service).await?;

    // Filter by state if provided
    let filtered_result = match state {
        Some(filter_state) => result.into_iter().filter(|d| d.state == filter_state).collect(),
        None => result,
    };

    // Apply pagination if parameters are provided
    let paginated_result = match (first, offset) {
        (Some(limit), Some(off)) => filtered_result
            .into_iter()
            .skip(off as usize)
            .take(limit as usize)
            .collect(),
        (Some(limit), None) => filtered_result
            .into_iter()
            .take(limit as usize)
            .collect(),
        (None, Some(off)) => filtered_result
            .into_iter()
            .skip(off as usize)
            .collect(),
        (None, None) => filtered_result,
    };

    Ok(paginated_result)
}

/// Fetch a single discount by its ID
pub async fn discount(id: DbUuid, context: &AppState) -> FieldResult<Discount> {
    let mut service = context.service.lock().await;

    // Using our command pattern
    let result = GetDiscountCommand { id }.exec(&mut service).await?;

    Ok(result)
}
