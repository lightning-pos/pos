use sea_query::{Alias, Expr, Query};
use juniper::FieldResult;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        models::finance::cost_center_model::{CostCenter, CostCenterState, CostCenters},
        types::db_uuid::DbUuid,
    },
    AppState,
};

pub async fn cost_centers(
    first: Option<i32>,
    offset: Option<i32>,
    context: &AppState,
) -> FieldResult<Vec<CostCenter>> {
    let service = context.service.lock().await;

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

    // Execute the query
    let result = service.db_adapter.query_many::<CostCenter>(&query).await?;

    Ok(result)
}

pub async fn cost_center(id: DbUuid, context: &AppState) -> FieldResult<CostCenter> {
    let service = context.service.lock().await;

    // Build the query with SeaQuery
    let mut query = Query::select();
    let query = query
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
        .and_where(Expr::col(CostCenters::Id).eq(id.to_string()));

    // Execute the query
    let result = service.db_adapter.query_one::<CostCenter>(&query).await?;

    Ok(result)
}

pub async fn all_cost_centers(context: &AppState) -> FieldResult<Vec<CostCenter>> {
    let service = context.service.lock().await;

    // Build the query with SeaQuery
    let mut query = Query::select();
    let query = query
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
        .and_where(Expr::col(CostCenters::State).eq(CostCenterState::Active.to_string()));

    // Execute the query
    let result = service.db_adapter.query_many::<CostCenter>(&query).await?;

    Ok(result)
}

pub async fn total_cost_centers(context: &AppState) -> FieldResult<i32> {
    let service = context.service.lock().await;

    // Build the count query with SeaQuery
    let mut query = Query::select();
    let query = query
        .from(CostCenters::Table)
        .expr_as(Expr::col(CostCenters::Id).count(), Alias::new("count"));

    // Execute the query
    let result = service.db_adapter.query_one::<i64>(&query).await?;

    Ok(result as i32)
}
