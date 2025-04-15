use sea_query::{Alias, Expr, Query, SqliteQueryBuilder};
use juniper::FieldResult;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        models::sales::sales_order_model::{SalesOrder, SalesOrders},
        types::db_uuid::DbUuid,
    },
    AppState,
};

pub fn sales_orders(
    first: Option<i32>,
    offset: Option<i32>,
    context: &AppState,
) -> FieldResult<Vec<SalesOrder>> {
    let service = context.service.lock().unwrap();

    // Build the query with SeaQuery
    let mut query_builder = Query::select();
    let query = query_builder
        .from(SalesOrders::Table)
        .columns([
            SalesOrders::Id,
            SalesOrders::OrderReadableId,
            SalesOrders::OrderDate,
            SalesOrders::CustomerId,
            SalesOrders::CustomerName,
            SalesOrders::CustomerPhoneNumber,
            SalesOrders::BillingAddress,
            SalesOrders::ShippingAddress,
            SalesOrders::NetAmount,
            SalesOrders::DiscAmount,
            SalesOrders::TaxableAmount,
            SalesOrders::TaxAmount,
            SalesOrders::TotalAmount,
            SalesOrders::OrderState,
            SalesOrders::PaymentState,
            SalesOrders::Notes,
            SalesOrders::ChannelId,
            SalesOrders::LocationId,
            SalesOrders::CostCenterId,
            SalesOrders::CreatedBy,
            SalesOrders::UpdatedBy,
            SalesOrders::DiscountId,
            SalesOrders::CreatedAt,
            SalesOrders::UpdatedAt,
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
    let result = service.db_adapter.query_many::<SalesOrder>(&sql, vec![])?;

    Ok(result)
}

pub fn total_sales_orders(context: &AppState) -> FieldResult<i32> {
    let service = context.service.lock().unwrap();

    // Build the count query with SeaQuery
    let query = Query::select()
        .from(SalesOrders::Table)
        .expr_as(Expr::col(SalesOrders::Id).count(), Alias::new("count"))
        .to_string(SqliteQueryBuilder);

    // Execute the query
    let result = service.db_adapter.query_one::<i64>(&query, vec![])?;

    Ok(result as i32)
}

pub fn sales_order(id: DbUuid, context: &AppState) -> FieldResult<SalesOrder> {
    let service = context.service.lock().unwrap();

    // Build the query with SeaQuery
    let query = Query::select()
        .from(SalesOrders::Table)
        .columns([
            SalesOrders::Id,
            SalesOrders::OrderReadableId,
            SalesOrders::OrderDate,
            SalesOrders::CustomerId,
            SalesOrders::CustomerName,
            SalesOrders::CustomerPhoneNumber,
            SalesOrders::BillingAddress,
            SalesOrders::ShippingAddress,
            SalesOrders::NetAmount,
            SalesOrders::DiscAmount,
            SalesOrders::TaxableAmount,
            SalesOrders::TaxAmount,
            SalesOrders::TotalAmount,
            SalesOrders::OrderState,
            SalesOrders::PaymentState,
            SalesOrders::Notes,
            SalesOrders::ChannelId,
            SalesOrders::LocationId,
            SalesOrders::CostCenterId,
            SalesOrders::CreatedBy,
            SalesOrders::UpdatedBy,
            SalesOrders::DiscountId,
            SalesOrders::CreatedAt,
            SalesOrders::UpdatedAt,
        ])
        .and_where(Expr::col(SalesOrders::Id).eq(id.to_string()))
        .to_string(SqliteQueryBuilder);

    // Execute the query
    let result = service.db_adapter.query_one::<SalesOrder>(&query, vec![])?;

    Ok(result)
}
