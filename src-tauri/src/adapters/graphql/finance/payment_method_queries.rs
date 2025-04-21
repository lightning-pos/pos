use sea_query::{Alias, Expr, Query};
use juniper::FieldResult;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        models::finance::payment_method_model::{PaymentMethod, PaymentMethodState, PaymentMethods},
        types::db_uuid::DbUuid,
    },
    AppState,
};

pub async fn payment_methods(
    first: Option<i32>,
    offset: Option<i32>,
    context: &AppState,
) -> FieldResult<Vec<PaymentMethod>> {
    let service = context.service.lock().await;

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

    // Execute the query
    let result = service.db_adapter.query_many::<PaymentMethod>(&query).await?;

    Ok(result)
}

pub async fn payment_method(id: DbUuid, context: &AppState) -> FieldResult<PaymentMethod> {
    let service = context.service.lock().await;

    // Build the query with SeaQuery
    let mut query = Query::select();
    let query = query
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
        .and_where(Expr::col(PaymentMethods::Id).eq(id.to_string()));

    // Execute the query
    let result = service.db_adapter.query_one::<PaymentMethod>(&query).await?;

    Ok(result)
}

pub async fn all_payment_methods(context: &AppState) -> FieldResult<Vec<PaymentMethod>> {
    let service = context.service.lock().await;

    // Build the query with SeaQuery
    let mut query = Query::select();
    let query = query
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
        .and_where(Expr::col(PaymentMethods::State).eq(PaymentMethodState::Active.to_string()));

    // Execute the query
    let result = service.db_adapter.query_many::<PaymentMethod>(&query).await?;

    Ok(result)
}

pub async fn total_payment_methods(context: &AppState) -> FieldResult<i32> {
    let service = context.service.lock().await;

    // Build the count query with SeaQuery
    let mut query = Query::select();
    let query = query
        .from(PaymentMethods::Table)
        .expr_as(Expr::col(PaymentMethods::Id).count(), Alias::new("count"));

    // Execute the query
    let result = service.db_adapter.query_one::<i64>(&query).await?;

    Ok(result as i32)
}
