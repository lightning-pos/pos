use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use juniper::FieldResult;

use crate::{
    core::{models::finance::cost_center_model::CostCenter, types::db_uuid::DbUuid},
    schema::cost_centers,
    AppState,
};

pub fn cost_centers(
    first: Option<i32>,
    offset: Option<i32>,
    context: &AppState,
) -> FieldResult<Vec<CostCenter>> {
    let mut service = context.service.lock().unwrap();

    let mut query = cost_centers::table.into_boxed();

    // Apply pagination if parameters are provided
    if let Some(limit) = first {
        query = query.limit(limit as i64);
    }
    if let Some(off) = offset {
        query = query.offset(off as i64);
    }

    let result = query
        .select(CostCenter::as_select())
        .load::<CostCenter>(&mut service.conn)?;

    Ok(result)
}

pub fn cost_center(id: DbUuid, context: &AppState) -> FieldResult<CostCenter> {
    let mut service = context.service.lock().unwrap();
    let result = cost_centers::table
        .filter(cost_centers::id.eq(id))
        .first::<CostCenter>(&mut service.conn)?;
    Ok(result)
}

pub fn all_cost_centers(context: &AppState) -> FieldResult<Vec<CostCenter>> {
    let mut service = context.service.lock().unwrap();

    use crate::core::models::finance::cost_center_model::CostCenterState;

    let result = cost_centers::table
        .filter(cost_centers::state.eq(CostCenterState::Active))
        .load::<CostCenter>(&mut service.conn)?;
    Ok(result)
}

pub fn total_cost_centers(context: &AppState) -> FieldResult<i32> {
    let mut service = context.service.lock().unwrap();
    let count: i64 = cost_centers::table.count().get_result(&mut service.conn)?;
    Ok(count as i32)
}
