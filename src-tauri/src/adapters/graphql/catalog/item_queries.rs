use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use juniper::FieldResult;

use crate::{
    core::{models::catalog::item_model::Item, types::db_uuid::DbUuid},
    schema::items,
    AppState,
};

pub fn items(
    first: Option<i32>,
    offset: Option<i32>,
    context: &AppState,
) -> FieldResult<Vec<Item>> {
    let mut service = context.service.lock().unwrap();

    let mut query = items::table.into_boxed();

    if let Some(limit) = first {
        query = query.limit(limit as i64);
    }

    if let Some(off) = offset {
        query = query.offset(off as i64);
    }

    let result = query
        .select(Item::as_select())
        .load::<Item>(&mut service.conn)?;
    Ok(result)
}

pub fn item(id: DbUuid, context: &AppState) -> FieldResult<Item> {
    let mut service = context.service.lock().unwrap();
    let result = items::table
        .filter(items::id.eq(id))
        .select(Item::as_select())
        .get_result(&mut service.conn)?;
    Ok(result)
}
