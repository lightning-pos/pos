use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use juniper::FieldResult;

use crate::{
    core::{models::common::brand_model::Brand, types::db_uuid::DbUuid},
    schema::brands,
    AppState,
};

pub fn get_brand(id: DbUuid, context: &AppState) -> FieldResult<Brand> {
    let conn = &mut context.service.lock().unwrap().conn;
    let brand = brands::table
        .filter(brands::id.eq(id))
        .select(Brand::as_select())
        .get_result(conn)?;

    Ok(brand)
}

pub fn get_brands(context: &AppState) -> FieldResult<Vec<Brand>> {
    let conn = &mut context.service.lock().unwrap().conn;
    let brands_list = brands::table.select(Brand::as_select()).load(conn)?;

    Ok(brands_list)
}

pub fn get_active_brands(context: &AppState) -> FieldResult<Vec<Brand>> {
    let conn = &mut context.service.lock().unwrap().conn;
    let brands_list = brands::table
        .filter(brands::is_active.eq(true))
        .select(Brand::as_select())
        .load(conn)?;

    Ok(brands_list)
}
