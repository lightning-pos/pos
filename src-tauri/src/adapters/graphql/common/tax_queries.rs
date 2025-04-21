use sea_query::{Alias, Expr, Order, Query, Func};
use juniper::FieldResult;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        models::common::tax_model::{Tax, Taxes},
        types::db_uuid::DbUuid,
    },
    AppState,
};

pub async fn taxes(first: Option<i32>, offset: Option<i32>, context: &AppState) -> FieldResult<Vec<Tax>> {
    let service = context.service.lock().await;

    // Build the query with SeaQuery
    let mut query_builder = Query::select();
    let mut query = query_builder
        .from(Taxes::Table)
        .columns([
            Taxes::Id,
            Taxes::Name,
            Taxes::Rate,
            Taxes::Description,
            Taxes::CreatedAt,
            Taxes::UpdatedAt,
        ])
        .order_by(Taxes::CreatedAt, Order::Desc);

    // Apply pagination if parameters are provided
    if let Some(limit) = first {
        query = query.limit(limit as u64);
    }
    if let Some(off) = offset {
        query = query.offset(off as u64);
    }

    // Execute the query
    let result = service.db_adapter.query_many::<Tax>(&query).await?;

    Ok(result)
}

pub async fn tax(id: DbUuid, context: &AppState) -> FieldResult<Tax> {
    let service = context.service.lock().await;

    // Build the query with SeaQuery
    let mut query_builder = Query::select();
    let query = query_builder
        .from(Taxes::Table)
        .columns([
            Taxes::Id,
            Taxes::Name,
            Taxes::Rate,
            Taxes::Description,
            Taxes::CreatedAt,
            Taxes::UpdatedAt,
        ])
        .and_where(Expr::col(Taxes::Id).eq(id.to_string()));

    // Execute the query
    let result = service.db_adapter.query_one::<Tax>(&query).await?;

    Ok(result)
}

pub async fn total_taxes(context: &AppState) -> FieldResult<i32> {
    let service = context.service.lock().await;

    // Build the count query with SeaQuery
    let mut query_builder = Query::select();
    let query = query_builder
        .from(Taxes::Table)
        .expr_as(Func::count(Expr::col(Taxes::Id)), Alias::new("count"));

    // Execute the query
    let result = service.db_adapter.query_one::<i64>(&query).await?;

    Ok(result as i32)
}
