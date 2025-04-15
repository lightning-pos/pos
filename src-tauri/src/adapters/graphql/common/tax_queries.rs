use sea_query::{Alias, Expr, Order, Query, SqliteQueryBuilder};
use juniper::FieldResult;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        models::common::tax_model::{Tax, Taxes},
        types::db_uuid::DbUuid,
    },
    AppState,
};

pub fn taxes(first: Option<i32>, offset: Option<i32>, context: &AppState) -> FieldResult<Vec<Tax>> {
    let service = context.service.lock().unwrap();

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
        .order_by(Taxes::CreatedAt, Order::Desc);

    // Apply pagination if parameters are provided
    if let Some(limit) = first {
        query.limit(limit as u64);
    }
    if let Some(off) = offset {
        query.offset(off as u64);
    }

    let sql = query.to_string(SqliteQueryBuilder);

    // Execute the query
    let result = service.db_adapter.query_many::<Tax>(&sql, vec![])?;

    Ok(result)
}

pub fn tax(id: DbUuid, context: &AppState) -> FieldResult<Tax> {
    let service = context.service.lock().unwrap();
    
    // Build the query with SeaQuery
    let query = Query::select()
        .from(Taxes::Table)
        .columns([
            Taxes::Id,
            Taxes::Name,
            Taxes::Rate,
            Taxes::Description,
            Taxes::CreatedAt,
            Taxes::UpdatedAt,
        ])
        .and_where(Expr::col(Taxes::Id).eq(id.to_string()))
        .to_string(SqliteQueryBuilder);
    
    // Execute the query
    let result = service.db_adapter.query_one::<Tax>(&query, vec![])?;
    
    Ok(result)
}

pub fn total_taxes(context: &AppState) -> FieldResult<i32> {
    let service = context.service.lock().unwrap();
    
    // Build the count query with SeaQuery
    let query = Query::select()
        .from(Taxes::Table)
        .expr_as(Expr::col(Taxes::Id).count(), Alias::new("count"))
        .to_string(SqliteQueryBuilder);
    
    // Execute the query
    let result = service.db_adapter.query_one::<i64>(&query, vec![])?;
    
    Ok(result as i32)
}