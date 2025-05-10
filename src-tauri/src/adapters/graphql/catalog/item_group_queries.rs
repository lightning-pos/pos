use sea_query::{Expr, Query};
use juniper::FieldResult;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        models::catalog::item_group_model::{ItemCategories, ItemCategory},
        types::db_uuid::DbUuid,
    },
    AppState,
};

pub async fn item_categories(
    first: Option<i32>,
    offset: Option<i32>,
    context: &AppState,
) -> FieldResult<Vec<ItemCategory>> {
    let service = context.service.lock().await;

    // Build the query with SeaQuery
    let mut query_builder = Query::select();
    let query = query_builder
        .from(ItemCategories::Table)
        .columns([
            ItemCategories::Id,
            ItemCategories::Name,
            ItemCategories::Description,
            ItemCategories::State,
            ItemCategories::CreatedAt,
            ItemCategories::UpdatedAt,
        ]);

    // Apply pagination if parameters are provided
    if let Some(limit) = first {
        query.limit(limit as u64);
    }
    if let Some(off) = offset {
        query.offset(off as u64);
    }

    // Execute the query
    let result = service.db_adapter.query_many::<ItemCategory>(&&query).await?;

    Ok(result)
}

pub async fn items_category(id: DbUuid, context: &AppState) -> FieldResult<ItemCategory> {
    let service = context.service.lock().await;

    // Build the query with SeaQuery
    let mut query_builder = Query::select();
    let query = query_builder
        .from(ItemCategories::Table)
        .columns([
            ItemCategories::Id,
            ItemCategories::Name,
            ItemCategories::Description,
            ItemCategories::State,
            ItemCategories::CreatedAt,
            ItemCategories::UpdatedAt,
        ])
        .and_where(Expr::col(ItemCategories::Id).eq(id.to_string()));

    // Execute the query
    let result = service.db_adapter.query_one::<ItemCategory>(&query).await?;

    Ok(result)
}
