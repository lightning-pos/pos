use sea_query::{Alias, Expr, Query, SqliteQueryBuilder};
use juniper::FieldResult;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        models::sales::sales_charge_type_model::{SalesChargeType, SalesChargeTypes},
        types::db_uuid::DbUuid,
    },
    AppState,
};

pub fn sales_charge_types(
    first: Option<i32>,
    offset: Option<i32>,
    context: &AppState,
) -> FieldResult<Vec<SalesChargeType>> {
    let service = context.service.lock().unwrap();
    
    // Build the query with SeaQuery
    let mut query_builder = Query::select();
    let query = query_builder
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
        query.limit(limit as u64);
    }
    if let Some(off) = offset {
        query.offset(off as u64);
    }

    let sql = query.to_string(SqliteQueryBuilder);

    // Execute the query
    let result = service.db_adapter.query_many::<SalesChargeType>(&sql, vec![])?;

    Ok(result)
}

pub fn sales_charge_type(id: DbUuid, context: &AppState) -> FieldResult<SalesChargeType> {
    let service = context.service.lock().unwrap();
    
    // Build the query with SeaQuery
    let query = Query::select()
        .from(SalesChargeTypes::Table)
        .columns([
            SalesChargeTypes::Id,
            SalesChargeTypes::Name,
            SalesChargeTypes::Description,
            SalesChargeTypes::CreatedAt,
            SalesChargeTypes::UpdatedAt,
        ])
        .and_where(Expr::col(SalesChargeTypes::Id).eq(id.to_string()))
        .to_string(SqliteQueryBuilder);
    
    // Execute the query
    let result = service.db_adapter.query_one::<SalesChargeType>(&query, vec![])?;

    Ok(result)
}

pub fn sales_charge_types_count(context: &AppState) -> FieldResult<i32> {
    let service = context.service.lock().unwrap();
    
    // Build the count query with SeaQuery
    let query = Query::select()
        .from(SalesChargeTypes::Table)
        .expr_as(Expr::col(SalesChargeTypes::Id).count(), Alias::new("count"))
        .to_string(SqliteQueryBuilder);
    
    // Execute the query
    let result = service.db_adapter.query_one::<i64>(&query, vec![])?;

    Ok(result as i32)
}