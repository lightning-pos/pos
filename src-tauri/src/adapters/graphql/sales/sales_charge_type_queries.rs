use sea_query::{Alias, Expr, Query};
use juniper::FieldResult;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        models::sales::sales_charge_type_model::{SalesChargeType, SalesChargeTypes},
        types::db_uuid::DbUuid,
    },
    AppState,
};

pub async fn sales_charge_types(
    first: Option<i32>,
    offset: Option<i32>,
    context: &AppState,
) -> FieldResult<Vec<SalesChargeType>> {
    let service = context.service.lock().await;

    // Build the query with SeaQuery
    let mut query_builder = Query::select();
    let stmt = query_builder
        .from(SalesChargeTypes::Table)
        .columns([
            SalesChargeTypes::Id,
            SalesChargeTypes::Name,
            SalesChargeTypes::Description,
            SalesChargeTypes::CreatedAt,
            SalesChargeTypes::UpdatedAt,
        ]);

    // Apply pagination if parameters are provided
    if let Some(limit) = first {
        stmt.limit(limit as u64);
    }
    if let Some(off) = offset {
        stmt.offset(off as u64);
    }

    // Execute the query
    let result = service.db_adapter.query_many::<SalesChargeType>(&stmt).await?;

    Ok(result)
}

pub async fn sales_charge_type(id: DbUuid, context: &AppState) -> FieldResult<SalesChargeType> {
    let service = context.service.lock().await;

    // Build the query with SeaQuery
    let mut query = Query::select();
    let stmt = query
        .from(SalesChargeTypes::Table)
        .columns([
            SalesChargeTypes::Id,
            SalesChargeTypes::Name,
            SalesChargeTypes::Description,
            SalesChargeTypes::CreatedAt,
            SalesChargeTypes::UpdatedAt,
        ])
        .and_where(Expr::col(SalesChargeTypes::Id).eq(id.to_string()));

    // Execute the query
    let result = service.db_adapter.query_one::<SalesChargeType>(&stmt).await?;

    Ok(result)
}

pub async fn sales_charge_types_count(context: &AppState) -> FieldResult<i32> {
    let service = context.service.lock().await;

    // Build the count query with SeaQuery
    let mut query = Query::select();
    let stmt = query
        .from(SalesChargeTypes::Table)
        .expr_as(Expr::col(SalesChargeTypes::Id).count(), Alias::new("count"));

    // Execute the query
    let result = service.db_adapter.query_one::<i64>(&stmt).await?;

    Ok(result as i32)
}
