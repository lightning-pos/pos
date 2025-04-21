use chrono::NaiveDateTime;
use sea_query::{Alias, Expr, Order, Query};
use juniper::FieldResult;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        models::purchases::expense_model::{Expense, Expenses},
        types::db_uuid::DbUuid,
    },
    AppState,
};

pub async fn expenses(
    first: Option<i32>,
    offset: Option<i32>,
    cost_center_id: Option<DbUuid>,
    start_date: Option<String>,
    end_date: Option<String>,
    context: &AppState,
) -> FieldResult<Vec<Expense>> {
    let service = context.service.lock().await;

    // Build the query with SeaQuery
    let mut query_builder = Query::select();
    let mut query = query_builder
        .from(Expenses::Table)
        .columns([
            Expenses::Id,
            Expenses::Title,
            Expenses::Amount,
            Expenses::ExpenseDate,
            Expenses::CategoryId,
            Expenses::CostCenterId,
            Expenses::Description,
            Expenses::CreatedAt,
            Expenses::UpdatedAt,
        ]);

    // Apply cost center filter if provided
    if let Some(cc_id) = cost_center_id {
        query = query.and_where(Expr::col(Expenses::CostCenterId).eq(cc_id.to_string()));
    }

    // Apply date range filters if provided
    if let Some(start) = start_date {
        if let Ok(date) = NaiveDateTime::parse_from_str(&start, "%Y-%m-%dT%H:%M:%S%.f%z") {
            query = query.and_where(Expr::col(Expenses::ExpenseDate).gte(date.to_string()));
        } else if let Ok(date) = NaiveDateTime::parse_from_str(&start, "%Y-%m-%dT%H:%M:%S%.f") {
            query = query.and_where(Expr::col(Expenses::ExpenseDate).gte(date.to_string()));
        }
    }

    if let Some(end) = end_date {
        if let Ok(date) = NaiveDateTime::parse_from_str(&end, "%Y-%m-%dT%H:%M:%S%.f%z") {
            query = query.and_where(Expr::col(Expenses::ExpenseDate).lte(date.to_string()));
        } else if let Ok(date) = NaiveDateTime::parse_from_str(&end, "%Y-%m-%dT%H:%M:%S%.f") {
            query = query.and_where(Expr::col(Expenses::ExpenseDate).lte(date.to_string()));
        }
    }

    // Order by expense date descending (newest first)
    query = query.order_by(Expenses::ExpenseDate, Order::Desc);

    // Apply pagination if parameters are provided
    if let Some(limit) = first {
        query = query.limit(limit as u64);
    }
    if let Some(off) = offset {
        query = query.offset(off as u64);
    }

    // Execute the query
    let result = service.db_adapter.query_many::<Expense>(&query).await?;

    Ok(result)
}

pub async fn total_expenses(
    cost_center_id: Option<DbUuid>,
    start_date: Option<String>,
    end_date: Option<String>,
    context: &AppState,
) -> FieldResult<i32> {
    let service = context.service.lock().await;

    // Build the query with SeaQuery
    let mut query_builder = Query::select();
    let mut query = query_builder
        .from(Expenses::Table)
        .expr_as(Expr::col(Expenses::Id).count(), Alias::new("count"));

    // Apply cost center filter if provided
    if let Some(cc_id) = cost_center_id {
        query = query.and_where(Expr::col(Expenses::CostCenterId).eq(cc_id.to_string()));
    }

    // Apply date range filters if provided
    if let Some(start) = start_date {
        if let Ok(date) = NaiveDateTime::parse_from_str(&start, "%Y-%m-%dT%H:%M:%S%.f%z") {
            query = query.and_where(Expr::col(Expenses::ExpenseDate).gte(date.to_string()));
        } else if let Ok(date) = NaiveDateTime::parse_from_str(&start, "%Y-%m-%dT%H:%M:%S%.f") {
            query = query.and_where(Expr::col(Expenses::ExpenseDate).gte(date.to_string()));
        }
    }

    if let Some(end) = end_date {
        if let Ok(date) = NaiveDateTime::parse_from_str(&end, "%Y-%m-%dT%H:%M:%S%.f%z") {
            query = query.and_where(Expr::col(Expenses::ExpenseDate).lte(date.to_string()));
        } else if let Ok(date) = NaiveDateTime::parse_from_str(&end, "%Y-%m-%dT%H:%M:%S%.f") {
            query = query.and_where(Expr::col(Expenses::ExpenseDate).lte(date.to_string()));
        }
    }

    // Execute the query
    let result = service.db_adapter.query_one::<i64>(&query).await?;

    Ok(result as i32)
}

pub async fn expense(id: DbUuid, context: &AppState) -> FieldResult<Expense> {
    let service = context.service.lock().await;

    // Build the query with SeaQuery
    let mut query = Query::select();
    let query = query
        .from(Expenses::Table)
        .columns([
            Expenses::Id,
            Expenses::Title,
            Expenses::Amount,
            Expenses::ExpenseDate,
            Expenses::CategoryId,
            Expenses::CostCenterId,
            Expenses::Description,
            Expenses::CreatedAt,
            Expenses::UpdatedAt,
        ])
        .and_where(Expr::col(Expenses::Id).eq(id.to_string()));

    // Execute the query
    let result = service.db_adapter.query_one::<Expense>(&query).await?;

    Ok(result)
}

pub async fn expenses_by_category(
    category_id: DbUuid,
    first: Option<i32>,
    offset: Option<i32>,
    context: &AppState,
) -> FieldResult<Vec<Expense>> {
    let service = context.service.lock().await;

    // Build the query with SeaQuery
    let mut query_builder = Query::select();
    let mut query = query_builder
        .from(Expenses::Table)
        .columns([
            Expenses::Id,
            Expenses::Title,
            Expenses::Amount,
            Expenses::ExpenseDate,
            Expenses::CategoryId,
            Expenses::CostCenterId,
            Expenses::Description,
            Expenses::CreatedAt,
            Expenses::UpdatedAt,
        ])
        .and_where(Expr::col(Expenses::CategoryId).eq(category_id.to_string()))
        .order_by(Expenses::ExpenseDate, Order::Desc);

    // Apply pagination if parameters are provided
    if let Some(limit) = first {
        query = query.limit(limit as u64);
    }
    if let Some(off) = offset {
        query = query.offset(off as u64);
    }

    // Execute the query
    let result = service.db_adapter.query_many::<Expense>(&query).await?;

    Ok(result)
}
