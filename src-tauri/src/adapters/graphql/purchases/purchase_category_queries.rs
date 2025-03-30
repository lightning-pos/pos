use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use juniper::FieldResult;

use crate::{
    core::{models::purchases::purchase_category_model::PurchaseCategory, types::db_uuid::DbUuid},
    schema::purchase_categories,
    AppState,
};

pub fn purchase_categories(
    first: Option<i32>,
    offset: Option<i32>,
    context: &AppState,
) -> FieldResult<Vec<PurchaseCategory>> {
    println!("yoyo inside purchase_categories");
    let mut service = context.service.lock().unwrap();

    let mut query = purchase_categories::table.into_boxed();

    // Apply pagination if parameters are provided
    if let Some(limit) = first {
        query = query.limit(limit as i64);
    }
    if let Some(off) = offset {
        query = query.offset(off as i64);
    }

    let result = query
        .select(PurchaseCategory::as_select())
        .load::<PurchaseCategory>(&mut service.conn)?;

    println!("yoyo result: {:?}", result);

    Ok(result)
}

pub fn purchase_category(id: DbUuid, context: &AppState) -> FieldResult<PurchaseCategory> {
    let mut service = context.service.lock().unwrap();
    let result = purchase_categories::table
        .filter(purchase_categories::id.eq(id))
        .first::<PurchaseCategory>(&mut service.conn)?;
    Ok(result)
}

pub fn all_purchase_categories(context: &AppState) -> FieldResult<Vec<PurchaseCategory>> {
    let mut service = context.service.lock().unwrap();

    use crate::core::models::purchases::purchase_category_model::PurchaseCategoryState;

    let result = purchase_categories::table
        .filter(purchase_categories::state.eq(PurchaseCategoryState::Active))
        .load::<PurchaseCategory>(&mut service.conn)?;
    Ok(result)
}
