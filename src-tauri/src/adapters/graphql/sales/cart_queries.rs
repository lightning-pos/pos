use sea_query::{Alias, Expr, Query, SqliteQueryBuilder};
use juniper::FieldResult;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        models::sales::cart_model::{Cart, Carts},
        types::db_uuid::DbUuid,
    },
    AppState,
};

pub fn carts(
    first: Option<i32>,
    offset: Option<i32>,
    context: &AppState,
) -> FieldResult<Vec<Cart>> {
    let service = context.service.lock().unwrap();
    
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

    let sql = query.to_string(SqliteQueryBuilder);

    // Execute the query
    let result = service.db_adapter.query_many::<Cart>(&sql, vec![])?;
    
    Ok(result)
}

pub fn total_carts(context: &AppState) -> FieldResult<i32> {
    let service = context.service.lock().unwrap();
    
    // Build the count query with SeaQuery
    let query = Query::select()
        .from(Carts::Table)
        .expr_as(Expr::col(Carts::Id).count(), Alias::new("count"))
        .to_string(SqliteQueryBuilder);
    
    // Execute the query
    let result = service.db_adapter.query_one::<i64>(&query, vec![])?;
    
    Ok(result as i32)
}

pub fn cart(id: DbUuid, context: &AppState) -> FieldResult<Cart> {
    let service = context.service.lock().unwrap();
    
    // Build the query with SeaQuery
    let query = Query::select()
        .from(Carts::Table)
        .columns([
            Carts::Id,
            Carts::CartData,
            Carts::CustomerId,
            Carts::CreatedAt,
            Carts::UpdatedAt,
        ])
        .and_where(Expr::col(Carts::Id).eq(id.to_string()))
        .to_string(SqliteQueryBuilder);
    
    // Execute the query
    let result = service.db_adapter.query_one::<Cart>(&query, vec![])?;
    
    Ok(result)
}