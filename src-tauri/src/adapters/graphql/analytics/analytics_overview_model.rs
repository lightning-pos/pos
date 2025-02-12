use juniper::GraphQLObject;

use crate::core::types::money::Money;

#[derive(GraphQLObject)]
pub struct AnalyticsOverview {
    pub total_sales: Money,
    pub total_orders: i32,
    pub total_customers: i32,
    pub total_products: i32,
}
