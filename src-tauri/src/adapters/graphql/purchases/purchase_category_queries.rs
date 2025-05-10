use sea_query::{Expr, Query};
use juniper::FieldResult;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        models::purchases::purchase_category_model::{PurchaseCategory, PurchaseCategoryState, PurchaseCategories},
        types::db_uuid::DbUuid,
    },
    AppState,
};

pub async fn purchase_categories(
    first: Option<i32>,
    offset: Option<i32>,
    context: &AppState,
) -> FieldResult<Vec<PurchaseCategory>> {
    let service = context.service.lock().await;

    // Build the query with SeaQuery
    let mut query_builder = Query::select();
    let query = query_builder
        .from(PurchaseCategories::Table)
        .columns([
            PurchaseCategories::Id,
            PurchaseCategories::Name,
            PurchaseCategories::Description,
            PurchaseCategories::State,
            PurchaseCategories::CreatedAt,
            PurchaseCategories::UpdatedAt,
        ]);

    // Apply pagination if parameters are provided
    if let Some(limit) = first {
        query.limit(limit as u64);
    }
    if let Some(off) = offset {
        query.offset(off as u64);
    }

    let result = service.db_adapter.query_many::<PurchaseCategory>(&query).await?;

    Ok(result)
}

pub async fn purchase_category(id: DbUuid, context: &AppState) -> FieldResult<PurchaseCategory> {
    let service = context.service.lock().await;

    // Build the query with SeaQuery
    let mut query = Query::select();
    let query = query
        .from(PurchaseCategories::Table)
        .columns([
            PurchaseCategories::Id,
            PurchaseCategories::Name,
            PurchaseCategories::Description,
            PurchaseCategories::State,
            PurchaseCategories::CreatedAt,
            PurchaseCategories::UpdatedAt,
        ])
        .and_where(Expr::col(PurchaseCategories::Id).eq(id.to_string()));

    // Execute the query
    let result = service.db_adapter.query_one::<PurchaseCategory>(&query).await?;

    Ok(result)
}

pub async fn all_purchase_categories(context: &AppState) -> FieldResult<Vec<PurchaseCategory>> {
    let service = context.service.lock().await;

    // Build the query with SeaQuery
    let mut query = Query::select();
    let query = query
        .from(PurchaseCategories::Table)
        .columns([
            PurchaseCategories::Id,
            PurchaseCategories::Name,
            PurchaseCategories::Description,
            PurchaseCategories::State,
            PurchaseCategories::CreatedAt,
            PurchaseCategories::UpdatedAt,
        ])
        .and_where(Expr::col(PurchaseCategories::State).eq(PurchaseCategoryState::Active.to_string()));

    // Execute the query
    let result = service.db_adapter.query_many::<PurchaseCategory>(&query).await?;

    Ok(result)
}
