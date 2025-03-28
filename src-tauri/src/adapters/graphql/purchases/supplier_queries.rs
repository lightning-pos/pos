use diesel::{dsl::count, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use juniper::FieldResult;

use crate::{
    core::{models::purchases::supplier_model::Supplier, types::db_uuid::DbUuid},
    schema::suppliers,
    AppState,
};

pub fn suppliers(
    first: Option<i32>,
    offset: Option<i32>,
    context: &AppState,
) -> FieldResult<Vec<Supplier>> {
    let mut service = context.service.lock().unwrap();
    let mut query = suppliers::table.into_boxed();
    if let Some(limit) = first {
        query = query.limit(limit as i64);
    }
    if let Some(off) = offset {
        query = query.offset(off as i64);
    }
    let result = query
        .select(Supplier::as_select())
        .load::<Supplier>(&mut service.conn)?;
    Ok(result)
}

pub fn total_suppliers(context: &AppState) -> FieldResult<i32> {
    let mut service = context.service.lock().unwrap();
    let result: i64 = suppliers::table
        .select(count(suppliers::id))
        .get_result(&mut service.conn)?;
    Ok(result as i32)
}

pub fn supplier(id: DbUuid, context: &AppState) -> FieldResult<Supplier> {
    let mut service = context.service.lock().unwrap();
    let result = suppliers::table
        .filter(suppliers::id.eq(id))
        .select(Supplier::as_select())
        .get_result(&mut service.conn)?;
    Ok(result)
}
