use bigdecimal::BigDecimal;
use chrono::{Duration, Utc};
use sea_query::{Alias, Expr, Query, SqliteQueryBuilder};
use juniper::FieldResult;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::models::{
        catalog::item_model::Items,
        sales::{
            customer_model::Customers,
            sales_order_model::SalesOrders,
        },
    },
    AppState,
};

use super::analytics_overview_model::AnalyticsOverview;

pub fn analytics_overview(days: Option<i32>, context: &AppState) -> FieldResult<AnalyticsOverview> {
    let service = context.service.lock().unwrap();

    // Calculate the start date based on the days parameter
    let start_date = match days {
        Some(d) => Utc::now() - Duration::days(d as i64),
        None => Utc::now() - Duration::days(365 * 10), // Default to 10 years if no days specified
    };

    // Get total sales with date filter
    let total_sales_query = Query::select()
        .from(SalesOrders::Table)
        .expr_as(Expr::col(SalesOrders::TotalAmount).sum(), Alias::new("total_sales"))
        .and_where(Expr::col(SalesOrders::CreatedAt).gte(start_date.naive_utc().to_string()))
        .to_string(SqliteQueryBuilder);

    let total_sales_result = service.db_adapter.query_optional::<BigDecimal>(&total_sales_query)?;
    let total_sales = total_sales_result.unwrap_or_default();

    // Get total orders with date filter
    let total_orders_query = Query::select()
        .from(SalesOrders::Table)
        .expr_as(Expr::col(SalesOrders::Id).count(), Alias::new("total_orders"))
        .and_where(Expr::col(SalesOrders::CreatedAt).gte(start_date.naive_utc().to_string()))
        .to_string(SqliteQueryBuilder);

    let total_orders = service.db_adapter.query_one::<i64>(&total_orders_query, vec![])?;

    // Get total customers (not filtered by date since it's current inventory)
    let total_customers_query = Query::select()
        .from(Customers::Table)
        .expr_as(Expr::col(Customers::Id).count(), Alias::new("total_customers"))
        .to_string(SqliteQueryBuilder);

    let total_customers = service.db_adapter.query_one::<i64>(&total_customers_query, vec![])?;

    // Get total products (not filtered by date since it's current inventory)
    let total_products_query = Query::select()
        .from(Items::Table)
        .expr_as(Expr::col(Items::Id).count(), Alias::new("total_products"))
        .to_string(SqliteQueryBuilder);

    let total_products = service.db_adapter.query_one::<i64>(&total_products_query, vec![])?;

    Ok(AnalyticsOverview {
        total_sales: total_sales.into(),
        total_orders: total_orders as i32,
        total_customers: total_customers as i32,
        total_products: total_products as i32,
    })
}
