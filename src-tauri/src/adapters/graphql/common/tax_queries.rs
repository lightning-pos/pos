use diesel::{dsl::count, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use juniper::FieldResult;

use crate::{
    core::{models::common::tax_model::Tax, types::db_uuid::DbUuid},
    schema::taxes,
    AppState,
};

pub fn taxes(first: Option<i32>, offset: Option<i32>, context: &AppState) -> FieldResult<Vec<Tax>> {
    let mut service = context.service.lock().unwrap();

    let mut query = taxes::table.order(taxes::created_at.desc()).into_boxed();

    // Apply pagination if parameters are provided
    if let Some(limit) = first {
        query = query.limit(limit as i64);
    }
    if let Some(off) = offset {
        query = query.offset(off as i64);
    }

    let result = query
        .select(Tax::as_select())
        .load::<Tax>(&mut service.conn)?;

    Ok(result)
}

pub fn tax(id: DbUuid, context: &AppState) -> FieldResult<Tax> {
    let mut service = context.service.lock().unwrap();
    let result = taxes::table
        .find(id)
        .select(Tax::as_select())
        .get_result::<Tax>(&mut service.conn)?;
    Ok(result)
}

pub fn total_taxes(context: &AppState) -> FieldResult<i32> {
    let mut service = context.service.lock().unwrap();
    let result: i64 = taxes::table
        .select(count(taxes::id))
        .get_result(&mut service.conn)?;
    Ok(result as i32)
}
