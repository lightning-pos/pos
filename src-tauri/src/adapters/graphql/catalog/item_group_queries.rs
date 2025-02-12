use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use juniper::FieldResult;

use crate::{
    core::{models::catalog::item_group_model::ItemGroup, types::db_uuid::DbUuid},
    schema::item_categories,
    AppState,
};

pub fn item_categories(
    first: Option<i32>,
    offset: Option<i32>,
    context: &AppState,
) -> FieldResult<Vec<ItemGroup>> {
    let mut service = context.service.lock().unwrap();

    let mut query = item_categories::table.into_boxed();

    // Apply pagination if parameters are provided
    if let Some(limit) = first {
        query = query.limit(limit as i64);
    }
    if let Some(off) = offset {
        query = query.offset(off as i64);
    }

    let result = query
        .select(ItemGroup::as_select())
        .load::<ItemGroup>(&mut service.conn)?;

    Ok(result)
}

pub fn items_category(id: DbUuid, context: &AppState) -> FieldResult<ItemGroup> {
    let mut service = context.service.lock().unwrap();
    let result = item_categories::table
        .filter(item_categories::id.eq(id))
        .select(ItemGroup::as_select())
        .get_result(&mut service.conn)?;
    Ok(result)
}
