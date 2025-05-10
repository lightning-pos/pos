use sea_query::{Alias, Expr, Query};
use juniper::FieldResult;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        models::sales::cart_model::{Cart, Carts},
        types::db_uuid::DbUuid,
    },
    AppState,
};

pub async fn carts(
    first: Option<i32>,
    offset: Option<i32>,
    context: &AppState,
) -> FieldResult<Vec<Cart>> {
    let service = context.service.lock().await;

    // Build the query with SeaQuery
    let mut query_builder = Query::select();
    let query = query_builder
        .from(Carts::Table)
        .columns([
            Carts::Id,
            Carts::CartData,
            Carts::CustomerId,
            Carts::CreatedAt,
            Carts::UpdatedAt,
        ]);

    // Apply pagination if parameters are provided
    if let Some(limit) = first {
        query.limit(limit as u64);
    }
    if let Some(off) = offset {
        query.offset(off as u64);
    }

    // Execute the query
    let result = service.db_adapter.query_many::<Cart>(&query).await?;

    Ok(result)
}

pub async fn total_carts(context: &AppState) -> FieldResult<i32> {
    let service = context.service.lock().await;

    // Build the count query with SeaQuery
    let mut query = Query::select();
    let query = query
        .from(Carts::Table)
        .expr_as(Expr::col(Carts::Id).count(), Alias::new("count"));

    // Execute the query
    let result = service.db_adapter.query_one::<i64>(&query).await?;

    Ok(result as i32)
}

pub async fn cart(id: DbUuid, context: &AppState) -> FieldResult<Cart> {
    let service = context.service.lock().await;

    // Build the query with SeaQuery
    let mut query = Query::select();
    let query = query
        .from(Carts::Table)
        .columns([
            Carts::Id,
            Carts::CartData,
            Carts::CustomerId,
            Carts::CreatedAt,
            Carts::UpdatedAt,
        ])
        .and_where(Expr::col(Carts::Id).eq(id.to_string()));

    // Execute the query
    let result = service.db_adapter.query_one::<Cart>(&query).await?;

    Ok(result)
}
