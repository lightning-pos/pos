use juniper::{graphql_object, FieldResult};

use crate::{
    adapters::graphql::Query,
    core::{
        models::{
            auth::user_model::User,
            catalog::{item_group_model::ItemGroup, item_model::Item},
            common::tax_model::Tax,
            purchases::{
                expense_model::Expense, purchase_category_model::PurchaseCategory,
                supplier_model::Supplier,
            },
            sales::{cart_model::Cart, customer_model::Customer, sales_order_model::SalesOrder},
        },
        types::db_uuid::DbUuid,
    },
    AppState,
};

use super::analytics::analytics_overview_model::AnalyticsOverview;

#[graphql_object(context = AppState)]
impl Query {
    fn api_version() -> &'static str {
        "1.0.0"
    }

    fn item_categories(
        &self,
        first: Option<i32>,
        offset: Option<i32>,
        context: &AppState,
    ) -> FieldResult<Vec<ItemGroup>> {
        super::catalog::item_group_queries::item_categories(first, offset, context)
    }

    fn items_category(&self, id: DbUuid, context: &AppState) -> FieldResult<ItemGroup> {
        super::catalog::item_group_queries::items_category(id, context)
    }

    fn items(
        &self,
        first: Option<i32>,
        offset: Option<i32>,
        context: &AppState,
    ) -> FieldResult<Vec<Item>> {
        super::catalog::item_queries::items(first, offset, context)
    }

    fn item(&self, id: DbUuid, context: &AppState) -> FieldResult<Item> {
        super::catalog::item_queries::item(id, context)
    }

    fn purchase_categories(
        &self,
        first: Option<i32>,
        offset: Option<i32>,
        context: &AppState,
    ) -> FieldResult<Vec<PurchaseCategory>> {
        super::purchases::purchase_category_queries::purchase_categories(first, offset, context)
    }

    fn purchase_category(&self, id: DbUuid, context: &AppState) -> FieldResult<PurchaseCategory> {
        super::purchases::purchase_category_queries::purchase_category(id, context)
    }

    fn users(
        &self,
        first: Option<i32>,
        offset: Option<i32>,
        context: &AppState,
    ) -> FieldResult<Vec<User>> {
        super::auth::user_queries::users(first, offset, context)
    }

    fn user(&self, id: DbUuid, context: &AppState) -> FieldResult<User> {
        super::auth::user_queries::user(id, context)
    }

    fn customers(
        &self,
        first: Option<i32>,
        offset: Option<i32>,
        context: &AppState,
    ) -> FieldResult<Vec<Customer>> {
        super::sales::customer_queries::customers(first, offset, context)
    }

    fn total_customers(&self, context: &AppState) -> FieldResult<i32> {
        super::sales::customer_queries::total_customers(context)
    }

    fn customer(&self, id: DbUuid, context: &AppState) -> FieldResult<Customer> {
        super::sales::customer_queries::customer(id, context)
    }

    fn customer_by_phone(&self, phone: String, context: &AppState) -> FieldResult<Customer> {
        super::sales::customer_queries::customer_by_phone(phone, context)
    }

    fn sales_orders(
        &self,
        first: Option<i32>,
        offset: Option<i32>,
        context: &AppState,
    ) -> FieldResult<Vec<SalesOrder>> {
        super::sales::sales_order_queries::sales_orders(first, offset, context)
    }

    fn total_sales_orders(&self, context: &AppState) -> FieldResult<i32> {
        super::sales::sales_order_queries::total_sales_orders(context)
    }

    fn sales_order(&self, id: DbUuid, context: &AppState) -> FieldResult<SalesOrder> {
        super::sales::sales_order_queries::sales_order(id, context)
    }

    fn carts(
        &self,
        first: Option<i32>,
        offset: Option<i32>,
        context: &AppState,
    ) -> FieldResult<Vec<Cart>> {
        super::sales::cart_queries::carts(first, offset, context)
    }

    fn total_carts(&self, context: &AppState) -> FieldResult<i32> {
        super::sales::cart_queries::total_carts(context)
    }

    fn cart(&self, id: DbUuid, context: &AppState) -> FieldResult<Cart> {
        super::sales::cart_queries::cart(id, context)
    }

    fn taxes(
        &self,
        first: Option<i32>,
        offset: Option<i32>,
        context: &AppState,
    ) -> FieldResult<Vec<Tax>> {
        super::common::tax_queries::taxes(first, offset, context)
    }

    fn total_taxes(&self, context: &AppState) -> FieldResult<i32> {
        super::common::tax_queries::total_taxes(context)
    }

    fn tax(&self, id: DbUuid, context: &AppState) -> FieldResult<Tax> {
        super::common::tax_queries::tax(id, context)
    }

    fn analytics_overview(
        &self,
        days: Option<i32>,
        context: &AppState,
    ) -> FieldResult<AnalyticsOverview> {
        super::analytics::analytics_queries::analytics_overview(days, context)
    }

    fn suppliers(
        &self,
        first: Option<i32>,
        offset: Option<i32>,
        context: &AppState,
    ) -> FieldResult<Vec<Supplier>> {
        super::purchases::supplier_queries::suppliers(first, offset, context)
    }

    fn total_suppliers(&self, context: &AppState) -> FieldResult<i32> {
        super::purchases::supplier_queries::total_suppliers(context)
    }

    fn supplier(&self, id: DbUuid, context: &AppState) -> FieldResult<Supplier> {
        super::purchases::supplier_queries::supplier(id, context)
    }

    fn expenses(
        &self,
        first: Option<i32>,
        offset: Option<i32>,
        context: &AppState,
    ) -> FieldResult<Vec<Expense>> {
        super::purchases::expense_queries::expenses(first, offset, context)
    }

    fn total_expenses(&self, context: &AppState) -> FieldResult<i32> {
        super::purchases::expense_queries::total_expenses(context)
    }

    fn expense(&self, id: DbUuid, context: &AppState) -> FieldResult<Expense> {
        super::purchases::expense_queries::expense(id, context)
    }

    fn expenses_by_category(
        &self,
        category: String,
        first: Option<i32>,
        offset: Option<i32>,
        context: &AppState,
    ) -> FieldResult<Vec<Expense>> {
        super::purchases::expense_queries::expenses_by_category(category, first, offset, context)
    }
}
