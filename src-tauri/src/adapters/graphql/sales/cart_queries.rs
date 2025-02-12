use diesel::{dsl::count, QueryDsl, RunQueryDsl, SelectableHelper};
use juniper::FieldResult;

use crate::{
    core::{models::sales::cart_model::Cart, types::db_uuid::DbUuid},
    schema::carts,
    AppState,
};

pub fn carts(
    first: Option<i32>,
    offset: Option<i32>,
    context: &AppState,
) -> FieldResult<Vec<Cart>> {
    let mut service = context.service.lock().unwrap();
    let mut query = carts::table.into_boxed();
    if let Some(limit) = first {
        query = query.limit(limit as i64);
    }
    if let Some(off) = offset {
        query = query.offset(off as i64);
    }
    let result = query
        .select(Cart::as_select())
        .load::<Cart>(&mut service.conn)?;
    Ok(result)
}

pub fn total_carts(context: &AppState) -> FieldResult<i32> {
    let mut service = context.service.lock().unwrap();
    let result: i64 = carts::table
        .select(count(carts::id))
        .get_result(&mut service.conn)?;
    Ok(result as i32)
}

pub fn cart(id: DbUuid, context: &AppState) -> FieldResult<Cart> {
    let mut service = context.service.lock().unwrap();
    let result = carts::table
        .find(id)
        .select(Cart::as_select())
        .get_result::<Cart>(&mut service.conn)?;
    Ok(result)
}
