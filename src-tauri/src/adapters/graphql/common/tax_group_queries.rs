use sea_query::{Alias, Expr, Order, Query, Func};
use juniper::FieldResult;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        models::common::tax_group_model::{TaxGroup, TaxGroups},
        types::db_uuid::DbUuid,
    },
    AppState,
};

pub async fn tax_groups(
    first: Option<i32>,
    offset: Option<i32>,
    context: &AppState,
) -> FieldResult<Vec<TaxGroup>> {
    let service = context.service.lock().await;

    // Build the query with SeaQuery
    let mut query_builder = Query::select();
    let mut query = query_builder
        .from(TaxGroups::Table)
        .columns([
            TaxGroups::Id,
            TaxGroups::Name,
            TaxGroups::Description,
            TaxGroups::CreatedAt,
            TaxGroups::UpdatedAt,
        ])
        .order_by(TaxGroups::CreatedAt, Order::Desc);

    // Apply pagination if parameters are provided
    if let Some(limit) = first {
        query = query.limit(limit as u64);
    }
    if let Some(off) = offset {
        query = query.offset(off as u64);
    }

    // Execute the query
    let result = service.db_adapter.query_many::<TaxGroup>(&query).await?;

    Ok(result)
}

pub async fn tax_group(id: DbUuid, context: &AppState) -> FieldResult<TaxGroup> {
    let service = context.service.lock().await;

    // Build the query with SeaQuery
    let mut query_builder = Query::select();
    let query = query_builder
        .from(TaxGroups::Table)
        .columns([
            TaxGroups::Id,
            TaxGroups::Name,
            TaxGroups::Description,
            TaxGroups::CreatedAt,
            TaxGroups::UpdatedAt,
        ])
        .and_where(Expr::col(TaxGroups::Id).eq(id.to_string()));

    // Execute the query
    let result = service.db_adapter.query_one::<TaxGroup>(&query).await?;

    Ok(result)
}

pub async fn total_tax_groups(context: &AppState) -> FieldResult<i32> {
    let service = context.service.lock().await;

    // Build the count query with SeaQuery
    let mut query_builder = Query::select();
    let query = query_builder
        .from(TaxGroups::Table)
        .expr_as(Func::count(Expr::col(TaxGroups::Id)), Alias::new("count"));

    // Execute the query
    let result = service.db_adapter.query_one::<i64>(&query).await?;

    Ok(result as i32)
}
