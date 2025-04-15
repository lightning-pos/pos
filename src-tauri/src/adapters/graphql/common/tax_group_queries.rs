use sea_query::{Alias, Expr, Order, Query, SqliteQueryBuilder};
use juniper::FieldResult;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        models::common::tax_group_model::{TaxGroup, TaxGroups},
        types::db_uuid::DbUuid,
    },
    AppState,
};

pub fn tax_groups(
    first: Option<i32>,
    offset: Option<i32>,
    context: &AppState,
) -> FieldResult<Vec<TaxGroup>> {
    let service = context.service.lock().unwrap();

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
        .order_by(TaxGroups::CreatedAt, Order::Desc);

    // Apply pagination if parameters are provided
    if let Some(limit) = first {
        query.limit(limit as u64);
    }
    if let Some(off) = offset {
        query.offset(off as u64);
    }

    let sql = query.to_string(SqliteQueryBuilder);

    // Execute the query
    let result = service.db_adapter.query_many::<TaxGroup>(&sql, vec![])?;

    Ok(result)
}

pub fn tax_group(id: DbUuid, context: &AppState) -> FieldResult<TaxGroup> {
    let service = context.service.lock().unwrap();
    
    // Build the query with SeaQuery
    let query = Query::select()
        .from(TaxGroups::Table)
        .columns([
            TaxGroups::Id,
            TaxGroups::Name,
            TaxGroups::Description,
            TaxGroups::CreatedAt,
            TaxGroups::UpdatedAt,
        ])
        .and_where(Expr::col(TaxGroups::Id).eq(id.to_string()))
        .to_string(SqliteQueryBuilder);
    
    // Execute the query
    let result = service.db_adapter.query_one::<TaxGroup>(&query, vec![])?;
    
    Ok(result)
}

pub fn total_tax_groups(context: &AppState) -> FieldResult<i32> {
    let service = context.service.lock().unwrap();
    
    // Build the count query with SeaQuery
    let query = Query::select()
        .from(TaxGroups::Table)
        .expr_as(Expr::col(TaxGroups::Id).count(), Alias::new("count"))
        .to_string(SqliteQueryBuilder);
    
    // Execute the query
    let result = service.db_adapter.query_one::<i64>(&query, vec![])?;
    
    Ok(result as i32)
}