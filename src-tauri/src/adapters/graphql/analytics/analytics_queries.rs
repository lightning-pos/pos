use bigdecimal::BigDecimal;
use chrono::{Duration, Utc};
use sea_query::{Alias, Expr, Func, Query};
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

pub async fn analytics_overview(days: Option<i32>, context: &AppState) -> FieldResult<AnalyticsOverview> {
    let service = context.service.lock().await;

    // Calculate the start date based on the days parameter
    let start_date = match days {
        Some(d) => Utc::now() - Duration::days(d as i64),
        None => Utc::now() - Duration::days(365 * 10), // Default to 10 years if no days specified
    };

    // Get total sales with date filter
    let mut total_sales_query = Query::select();
    let total_sales_stmt = total_sales_query
        .from(SalesOrders::Table)
        .expr_as(Func::sum(Expr::col(SalesOrders::TotalAmount)), Alias::new("total_sales"))
        .and_where(Expr::col(SalesOrders::CreatedAt).gte(start_date.naive_utc().to_string()));

    let total_sales_result = service.db_adapter.query_optional::<BigDecimal>(&total_sales_stmt).await?;
    let total_sales = total_sales_result.unwrap_or_default();

    // Get total orders with date filter
    let mut total_orders_query = Query::select();
    let total_orders_stmt = total_orders_query
        .from(SalesOrders::Table)
        .expr_as(Func::count(Expr::col(SalesOrders::Id)), Alias::new("total_orders"))
        .and_where(Expr::col(SalesOrders::CreatedAt).gte(start_date.naive_utc().to_string()));

    let total_orders = service.db_adapter.query_one::<i64>(&total_orders_stmt).await?;

    // Get total customers (not filtered by date since it's current inventory)
    let mut total_customers_query = Query::select();
    let total_customers_stmt = total_customers_query
        .from(Customers::Table)
        .expr_as(Func::count(Expr::col(Customers::Id)), Alias::new("total_customers"));

    let total_customers = service.db_adapter.query_one::<i64>(&total_customers_stmt).await?;

    // Get total products (not filtered by date since it's current inventory)
    let mut total_products_query = Query::select();
    let total_products_stmt = total_products_query
        .from(Items::Table)
        .expr_as(Func::count(Expr::col(Items::Id)), Alias::new("total_products"));

    let total_products = service.db_adapter.query_one::<i64>(&total_products_stmt).await?;

    Ok(AnalyticsOverview {
        total_sales: total_sales.into(),
        total_orders: total_orders as i32,
        total_customers: total_customers as i32,
        total_products: total_products as i32,
    })
}
