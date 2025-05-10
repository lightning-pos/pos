use sea_query::{Alias, Expr, Query};
use juniper::FieldResult;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        models::sales::sales_order_model::{SalesOrder, SalesOrders},
        types::db_uuid::DbUuid,
    },
    AppState,
};

pub async fn sales_orders(
    first: Option<i32>,
    offset: Option<i32>,
    context: &AppState,
) -> FieldResult<Vec<SalesOrder>> {
    let service = context.service.lock().await;

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

    // Execute the query
    let result = service.db_adapter.query_many::<SalesOrder>(&query).await?;

    Ok(result)
}

pub async fn total_sales_orders(context: &AppState) -> FieldResult<i32> {
    let service = context.service.lock().await;

    // Build the count query with SeaQuery
    let mut query = Query::select();
    let query = query
        .from(SalesOrders::Table)
        .expr_as(Expr::col(SalesOrders::Id).count(), Alias::new("count"));

    // Execute the query
    let result = service.db_adapter.query_one::<i64>(&query).await?;

    Ok(result as i32)
}

pub async fn sales_order(id: DbUuid, context: &AppState) -> FieldResult<SalesOrder> {
    let service = context.service.lock().await;

    // Build the query with SeaQuery
    let mut query = Query::select();
    let query = query
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
        .and_where(Expr::col(SalesOrders::Id).eq(id.to_string()));

    // Execute the query
    let result = service.db_adapter.query_one::<SalesOrder>(&query).await?;

    Ok(result)
}
