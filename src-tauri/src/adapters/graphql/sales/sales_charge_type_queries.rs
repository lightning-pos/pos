use diesel::{dsl::count, QueryDsl, RunQueryDsl, SelectableHelper};
use juniper::FieldResult;

use crate::{
    core::{models::sales::sales_charge_type_model::SalesChargeType, types::db_uuid::DbUuid},
    schema::sales_charge_types,
    AppState,
};

pub fn sales_charge_types(
    first: Option<i32>,
    offset: Option<i32>,
    context: &AppState,
) -> FieldResult<Vec<SalesChargeType>> {
    let mut service = context.service.lock().unwrap();
    let mut query = sales_charge_types::table.into_boxed();

    if let Some(limit) = first {
        query = query.limit(limit as i64);
    }
    if let Some(off) = offset {
        query = query.offset(off as i64);
    }

    let result = query
        .select(SalesChargeType::as_select())
        .load::<SalesChargeType>(&mut service.conn)?;

    Ok(result)
}

pub fn sales_charge_type(id: DbUuid, context: &AppState) -> FieldResult<SalesChargeType> {
    let mut service = context.service.lock().unwrap();
    let result = sales_charge_types::table
        .find(id)
        .select(SalesChargeType::as_select())
        .first::<SalesChargeType>(&mut service.conn)?;

    Ok(result)
}

pub fn sales_charge_types_count(context: &AppState) -> FieldResult<i32> {
    let mut service = context.service.lock().unwrap();
    let result = sales_charge_types::table
        .select(count(sales_charge_types::id))
        .first::<i64>(&mut service.conn)?;

    Ok(result as i32)
}
