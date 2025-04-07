use diesel::{dsl::count, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use juniper::FieldResult;

use crate::{
    core::{models::common::tax_group_model::TaxGroup, types::db_uuid::DbUuid},
    schema::tax_groups,
    AppState,
};

pub fn tax_groups(
    first: Option<i32>,
    offset: Option<i32>,
    context: &AppState,
) -> FieldResult<Vec<TaxGroup>> {
    let mut service = context.service.lock().unwrap();

    let mut query = tax_groups::table.order(tax_groups::created_at.desc()).into_boxed();

    // Apply pagination if parameters are provided
    if let Some(limit) = first {
        query = query.limit(limit as i64);
    }
    if let Some(off) = offset {
        query = query.offset(off as i64);
    }

    let result = query
        .select(TaxGroup::as_select())
        .load::<TaxGroup>(&mut service.conn)?;

    Ok(result)
}

pub fn tax_group(id: DbUuid, context: &AppState) -> FieldResult<TaxGroup> {
    let mut service = context.service.lock().unwrap();
    let result = tax_groups::table
        .find(id)
        .select(TaxGroup::as_select())
        .get_result::<TaxGroup>(&mut service.conn)?;
    Ok(result)
}

pub fn total_tax_groups(context: &AppState) -> FieldResult<i32> {
    let mut service = context.service.lock().unwrap();
    let result: i64 = tax_groups::table
        .select(count(tax_groups::id))
        .get_result(&mut service.conn)?;
    Ok(result as i32)
}
