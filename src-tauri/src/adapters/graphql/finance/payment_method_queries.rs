use sea_query::{Alias, Expr, Order, Query, SqliteQueryBuilder};
use juniper::FieldResult;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        models::finance::payment_method_model::{PaymentMethod, PaymentMethodState, PaymentMethods},
        types::db_uuid::DbUuid,
    },
    AppState,
};

pub fn payment_methods(
    first: Option<i32>,
    offset: Option<i32>,
    context: &AppState,
) -> FieldResult<Vec<PaymentMethod>> {
    let service = context.service.lock().unwrap();

    // Build the query with SeaQuery
    let mut query_builder = Query::select();
    let query = query_builder
        .from(PaymentMethods::Table)
        .columns([
            PaymentMethods::Id,
            PaymentMethods::Name,
            PaymentMethods::Code,
            PaymentMethods::Description,
            PaymentMethods::State,
            PaymentMethods::CreatedAt,
            PaymentMethods::UpdatedAt,
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
    let result = service.db_adapter.query_many::<PaymentMethod>(&sql, vec![])?;

    Ok(result)
}

pub fn payment_method(id: DbUuid, context: &AppState) -> FieldResult<PaymentMethod> {
    let service = context.service.lock().unwrap();
    
    // Build the query with SeaQuery
    let query = Query::select()
        .from(PaymentMethods::Table)
        .columns([
            PaymentMethods::Id,
            PaymentMethods::Name,
            PaymentMethods::Code,
            PaymentMethods::Description,
            PaymentMethods::State,
            PaymentMethods::CreatedAt,
            PaymentMethods::UpdatedAt,
        ])
        .and_where(Expr::col(PaymentMethods::Id).eq(id.to_string()))
        .to_string(SqliteQueryBuilder);
    
    // Execute the query
    let result = service.db_adapter.query_one::<PaymentMethod>(&query, vec![])?;
    
    Ok(result)
}

pub fn all_payment_methods(context: &AppState) -> FieldResult<Vec<PaymentMethod>> {
    let service = context.service.lock().unwrap();
    
    // Build the query with SeaQuery
    let query = Query::select()
        .from(PaymentMethods::Table)
        .columns([
            PaymentMethods::Id,
            PaymentMethods::Name,
            PaymentMethods::Code,
            PaymentMethods::Description,
            PaymentMethods::State,
            PaymentMethods::CreatedAt,
            PaymentMethods::UpdatedAt,
        ])
        .and_where(Expr::col(PaymentMethods::State).eq(PaymentMethodState::Active.to_string()))
        .to_string(SqliteQueryBuilder);
    
    // Execute the query
    let result = service.db_adapter.query_many::<PaymentMethod>(&query, vec![])?;
    
    Ok(result)
}

pub fn total_payment_methods(context: &AppState) -> FieldResult<i32> {
    let service = context.service.lock().unwrap();
    
    // Build the count query with SeaQuery
    let query = Query::select()
        .from(PaymentMethods::Table)
        .expr_as(Expr::col(PaymentMethods::Id).count(), Alias::new("count"))
        .to_string(SqliteQueryBuilder);
    
    // Execute the query
    let result = service.db_adapter.query_one::<i64>(&query, vec![])?;
    
    Ok(result as i32)
}