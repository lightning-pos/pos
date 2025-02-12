use diesel::{dsl::count, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use juniper::FieldResult;

use crate::{
    core::{models::sales::customer_model::Customer, types::db_uuid::DbUuid},
    schema::customers,
    AppState,
};

pub fn customers(
    first: Option<i32>,
    offset: Option<i32>,
    context: &AppState,
) -> FieldResult<Vec<Customer>> {
    let mut service = context.service.lock().unwrap();
    let mut query = customers::table.into_boxed();
    if let Some(limit) = first {
        query = query.limit(limit as i64);
    }
    if let Some(off) = offset {
        query = query.offset(off as i64);
    }
    let result = query
        .select(Customer::as_select())
        .load::<Customer>(&mut service.conn)?;
    Ok(result)
}

pub fn total_customers(context: &AppState) -> FieldResult<i32> {
    let mut service = context.service.lock().unwrap();
    let result: i64 = customers::table
        .select(count(customers::id))
        .get_result(&mut service.conn)?;
    Ok(result as i32)
}

pub fn customer(id: DbUuid, context: &AppState) -> FieldResult<Customer> {
    let mut service = context.service.lock().unwrap();
    let result = customers::table
        .filter(customers::id.eq(id))
        .select(Customer::as_select())
        .get_result(&mut service.conn)?;
    Ok(result)
}

pub fn customer_by_phone(phone: String, context: &AppState) -> FieldResult<Customer> {
    let mut service = context.service.lock().unwrap();
    let result = customers::table
        .filter(customers::phone.eq(phone))
        .select(Customer::as_select())
        .get_result(&mut service.conn)?;
    Ok(result)
}
