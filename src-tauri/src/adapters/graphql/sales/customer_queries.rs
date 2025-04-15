use sea_query::{Alias, Expr, Query, SqliteQueryBuilder};
use juniper::FieldResult;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        models::sales::customer_model::{Customer, Customers},
        types::db_uuid::DbUuid,
    },
    AppState,
};

pub fn customers(
    first: Option<i32>,
    offset: Option<i32>,
    context: &AppState,
) -> FieldResult<Vec<Customer>> {
    let service = context.service.lock().unwrap();

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

    let sql = query.to_string(SqliteQueryBuilder);

    // Execute the query
    let result = service.db_adapter.query_many::<Customer>(&sql, vec![])?;

    Ok(result)
}

pub fn total_customers(context: &AppState) -> FieldResult<i32> {
    let service = context.service.lock().unwrap();

    // Build the count query with SeaQuery
    let query = Query::select()
        .from(Customers::Table)
        .expr_as(Expr::col(Customers::Id).count(), Alias::new("count"))
        .to_string(SqliteQueryBuilder);

    // Execute the query
    let result = service.db_adapter.query_one::<i64>(&query, vec![])?;

    Ok(result as i32)
}

pub fn customer(id: DbUuid, context: &AppState) -> FieldResult<Customer> {
    let service = context.service.lock().unwrap();

    // Build the query with SeaQuery
    let query = Query::select()
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
        .and_where(Expr::col(Customers::Id).eq(id.to_string()))
        .to_string(SqliteQueryBuilder);

    // Execute the query
    let result = service.db_adapter.query_one::<Customer>(&query, vec![])?;

    Ok(result)
}

pub fn customer_by_phone(phone: String, context: &AppState) -> FieldResult<Customer> {
    let service = context.service.lock().unwrap();

    // Build the query with SeaQuery
    let query = Query::select()
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
        .and_where(Expr::col(Customers::Phone).eq(phone))
        .to_string(SqliteQueryBuilder);

    // Execute the query
    let result = service.db_adapter.query_one::<Customer>(&query, vec![])?;

    Ok(result)
}
