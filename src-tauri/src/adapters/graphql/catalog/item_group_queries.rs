use sea_query::{Expr, Query, SqliteQueryBuilder};
use juniper::FieldResult;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        models::catalog::item_group_model::{ItemGroup, ItemCategories},
        types::db_uuid::DbUuid,
    },
    AppState,
};

pub fn item_categories(
    first: Option<i32>,
    offset: Option<i32>,
    context: &AppState,
) -> FieldResult<Vec<ItemGroup>> {
    let service = context.service.lock().unwrap();

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

    let sql = query.to_string(SqliteQueryBuilder);

    // Execute the query
    let result = service.db_adapter.query_many::<ItemGroup>(&sql, vec![])?;

    Ok(result)
}

pub fn items_category(id: DbUuid, context: &AppState) -> FieldResult<ItemGroup> {
    let service = context.service.lock().unwrap();

    // Build the query with SeaQuery
    let query = Query::select()
        .from(ItemCategories::Table)
        .columns([
            ItemCategories::Id,
            ItemCategories::Name,
            ItemCategories::Description,
            ItemCategories::State,
            ItemCategories::CreatedAt,
            ItemCategories::UpdatedAt,
        ])
        .and_where(Expr::col(ItemCategories::Id).eq(id.to_string()))
        .to_string(SqliteQueryBuilder);

    // Execute the query
    let result = service.db_adapter.query_one::<ItemGroup>(&query, vec![])?;

    Ok(result)
}
