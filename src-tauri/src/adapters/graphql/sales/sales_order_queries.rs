use diesel::{dsl::count, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use juniper::FieldResult;

use crate::{
    core::{models::sales::sales_order_model::SalesOrder, types::db_uuid::DbUuid},
    schema::sales_orders,
    AppState,
};

pub fn sales_orders(
    first: Option<i32>,
    offset: Option<i32>,
    context: &AppState,
) -> FieldResult<Vec<SalesOrder>> {
    let mut service = context.service.lock().unwrap();
    let mut query = sales_orders::table.into_boxed();
    if let Some(limit) = first {
        query = query.limit(limit as i64);
    }
    if let Some(off) = offset {
        query = query.offset(off as i64);
    }
    let result = query
        .select(SalesOrder::as_select())
        .load::<SalesOrder>(&mut service.conn)?;
    Ok(result)
}

pub fn total_sales_orders(context: &AppState) -> FieldResult<i32> {
    let mut service = context.service.lock().unwrap();
    let result: i64 = sales_orders::table
        .select(count(sales_orders::id))
        .get_result(&mut service.conn)?;
    Ok(result as i32)
}

pub fn sales_order(id: DbUuid, context: &AppState) -> FieldResult<SalesOrder> {
    let mut service = context.service.lock().unwrap();
    let result = sales_orders::table
        .filter(sales_orders::id.eq(id))
        .select(SalesOrder::as_select())
        .get_result(&mut service.conn)?;
    Ok(result)
}
