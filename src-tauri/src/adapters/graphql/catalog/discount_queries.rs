use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use juniper::FieldResult;

use crate::{
    core::{
        commands::{catalog::discount_commands::GetDiscountCommand, Command},
        models::catalog::discount_model::{Discount, DiscountState},
        types::db_uuid::DbUuid,
    },
    schema::discounts,
    AppState,
};

/// Fetch a list of all discounts, with optional pagination
pub fn discounts(
    first: Option<i32>,
    offset: Option<i32>,
    state: Option<DiscountState>,
    context: &AppState,
) -> FieldResult<Vec<Discount>> {
    let mut service = context.service.lock().unwrap();

    // If we want to apply additional filtering (like by state), we should use this query approach
    let mut query = discounts::table.into_boxed();

    // Filter by state if provided
    if let Some(filter_state) = state {
        query = query.filter(discounts::state.eq(filter_state));
    }

    // Apply pagination if parameters are provided
    if let Some(limit) = first {
        query = query.limit(limit as i64);
    }
    if let Some(off) = offset {
        query = query.offset(off as i64);
    }

    let result = query
        .select(Discount::as_select())
        .load::<Discount>(&mut service.conn)?;

    Ok(result)

    // Alternative implementation using the command pattern
    // let result = ListDiscountsCommand.exec(&mut service)?;
    // Ok(result)
}

/// Fetch a single discount by its ID
pub fn discount(id: DbUuid, context: &AppState) -> FieldResult<Discount> {
    let mut service = context.service.lock().unwrap();

    // Using our command pattern
    let result = GetDiscountCommand { id }.exec(&mut service)?;

    Ok(result)
}
