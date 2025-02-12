use bigdecimal::BigDecimal;
use chrono::{Duration, Utc};
use diesel::{
    dsl::{count, sum},
    query_dsl::methods::{FilterDsl, SelectDsl},
    ExpressionMethods, RunQueryDsl,
};
use juniper::FieldResult;

use crate::{
    schema::{customers, items, sales_orders},
    AppState,
};

use super::analytics_overview_model::AnalyticsOverview;

pub fn analytics_overview(days: Option<i32>, context: &AppState) -> FieldResult<AnalyticsOverview> {
    let mut service = context.service.lock().unwrap();

    // Calculate the start date based on the days parameter
    let start_date = match days {
        Some(d) => Utc::now() - Duration::days(d as i64),
        None => Utc::now() - Duration::days(365 * 10), // Default to 10 years if no days specified
    };

    // Get total sales with date filter
    let total_sales = sales_orders::table
        .select(sum(sales_orders::total_amount))
        .filter(sales_orders::created_at.ge(start_date.naive_utc()))
        .first::<Option<BigDecimal>>(&mut service.conn)
        .unwrap()
        .unwrap_or_default();

    // Get total orders with date filter
    let total_orders = sales_orders::table
        .select(count(sales_orders::id))
        .filter(sales_orders::created_at.ge(start_date.naive_utc()))
        .first::<i64>(&mut service.conn)
        .unwrap();

    // Get total customers (not filtered by date since it's current inventory)
    let total_customers = customers::table
        .select(count(customers::id))
        .first::<i64>(&mut service.conn)
        .unwrap();

    // Get total products (not filtered by date since it's current inventory)
    let total_products = items::table
        .select(count(items::id))
        .first::<i64>(&mut service.conn)
        .unwrap();

    Ok(AnalyticsOverview {
        total_sales: total_sales.into(),
        total_orders: total_orders as i32,
        total_customers: total_customers as i32,
        total_products: total_products as i32,
    })
}
