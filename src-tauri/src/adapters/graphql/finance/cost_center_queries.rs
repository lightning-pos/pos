use sea_query::{Alias, Expr, Order, Query, SqliteQueryBuilder};
use juniper::FieldResult;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        models::finance::cost_center_model::{CostCenter, CostCenterState, CostCenters},
        types::db_uuid::DbUuid,
    },
    AppState,
};

pub fn cost_centers(
    first: Option<i32>,
    offset: Option<i32>,
    context: &AppState,
) -> FieldResult<Vec<CostCenter>> {
    let service = context.service.lock().unwrap();

    // Build the query with SeaQuery
    let mut query_builder = Query::select();
    let query = query_builder
        .from(CostCenters::Table)
        .columns([
            CostCenters::Id,
            CostCenters::Name,
            CostCenters::Code,
            CostCenters::Description,
            CostCenters::State,
            CostCenters::CreatedAt,
            CostCenters::UpdatedAt,
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
    let result = service.db_adapter.query_many::<CostCenter>(&sql, vec![])?;

    Ok(result)
}

pub fn cost_center(id: DbUuid, context: &AppState) -> FieldResult<CostCenter> {
    let service = context.service.lock().unwrap();
    
    // Build the query with SeaQuery
    let query = Query::select()
        .from(CostCenters::Table)
        .columns([
            CostCenters::Id,
            CostCenters::Name,
            CostCenters::Code,
            CostCenters::Description,
            CostCenters::State,
            CostCenters::CreatedAt,
            CostCenters::UpdatedAt,
        ])
        .and_where(Expr::col(CostCenters::Id).eq(id.to_string()))
        .to_string(SqliteQueryBuilder);
    
    // Execute the query
    let result = service.db_adapter.query_one::<CostCenter>(&query, vec![])?;
    
    Ok(result)
}

pub fn all_cost_centers(context: &AppState) -> FieldResult<Vec<CostCenter>> {
    let service = context.service.lock().unwrap();
    
    // Build the query with SeaQuery
    let query = Query::select()
        .from(CostCenters::Table)
        .columns([
            CostCenters::Id,
            CostCenters::Name,
            CostCenters::Code,
            CostCenters::Description,
            CostCenters::State,
            CostCenters::CreatedAt,
            CostCenters::UpdatedAt,
        ])
        .and_where(Expr::col(CostCenters::State).eq(CostCenterState::Active.to_string()))
        .to_string(SqliteQueryBuilder);
    
    // Execute the query
    let result = service.db_adapter.query_many::<CostCenter>(&query, vec![])?;
    
    Ok(result)
}

pub fn total_cost_centers(context: &AppState) -> FieldResult<i32> {
    let service = context.service.lock().unwrap();
    
    // Build the count query with SeaQuery
    let query = Query::select()
        .from(CostCenters::Table)
        .expr_as(Expr::col(CostCenters::Id).count(), Alias::new("count"))
        .to_string(SqliteQueryBuilder);
    
    // Execute the query
    let result = service.db_adapter.query_one::<i64>(&query, vec![])?;
    
    Ok(result as i32)
}