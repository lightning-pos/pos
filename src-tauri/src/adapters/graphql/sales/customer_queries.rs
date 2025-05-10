use sea_query::{Alias, Expr, Query};
use juniper::FieldResult;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        models::sales::customer_model::{Customer, Customers},
        types::db_uuid::DbUuid,
    },
    AppState,
};

pub async fn customers(
    first: Option<i32>,
    offset: Option<i32>,
    context: &AppState,
) -> FieldResult<Vec<Customer>> {
    let service = context.service.lock().await;

    // Build the query with SeaQuery
    let mut query_builder = Query::select();
    let query = query_builder
        .from(Customers::Table)
        .columns([
            Customers::Id,
            Customers::FullName,
            Customers::Email,
            Customers::Phone,
            Customers::Address,
            Customers::CreatedAt,
            Customers::UpdatedAt,
        ]);

    // Apply pagination if parameters are provided
    if let Some(limit) = first {
        query.limit(limit as u64);
    }
    if let Some(off) = offset {
        query.offset(off as u64);
    }

    // Execute the query
    let result = service.db_adapter.query_many::<Customer>(&query).await?;

    Ok(result)
}

pub async fn total_customers(context: &AppState) -> FieldResult<i32> {
    let service = context.service.lock().await;

    // Build the count query with SeaQuery
    let mut query = Query::select();
    let query = query
        .from(Customers::Table)
        .expr_as(Expr::col(Customers::Id).count(), Alias::new("count"));

    // Execute the query
    let result = service.db_adapter.query_one::<i64>(&query).await?;

    Ok(result as i32)
}

pub async fn customer(id: DbUuid, context: &AppState) -> FieldResult<Customer> {
    let service = context.service.lock().await;

    // Build the query with SeaQuery
    let mut query = Query::select();
    let query = query
        .from(Customers::Table)
        .columns([
            Customers::Id,
            Customers::FullName,
            Customers::Email,
            Customers::Phone,
            Customers::Address,
            Customers::CreatedAt,
            Customers::UpdatedAt,
        ])
        .and_where(Expr::col(Customers::Id).eq(id.to_string()));

    // Execute the query
    let result = service.db_adapter.query_one::<Customer>(&query).await?;

    Ok(result)
}

pub async fn customer_by_phone(phone: String, context: &AppState) -> FieldResult<Customer> {
    let service = context.service.lock().await;

    // Build the query with SeaQuery
    let mut query = Query::select();
    let query = query
        .from(Customers::Table)
        .columns([
            Customers::Id,
            Customers::FullName,
            Customers::Email,
            Customers::Phone,
            Customers::Address,
            Customers::CreatedAt,
            Customers::UpdatedAt,
        ])
        .and_where(Expr::col(Customers::Phone).eq(phone));

    // Execute the query
    let result = service.db_adapter.query_one::<Customer>(&query).await?;

    Ok(result)
}
