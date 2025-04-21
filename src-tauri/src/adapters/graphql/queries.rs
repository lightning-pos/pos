use juniper::{graphql_object, FieldResult};

use crate::{
    adapters::graphql::{catalog::variants::{item_variant_queries, variant_type_queries, variant_value_queries}, Query},
    core::{
        models::{
            auth::user_model::User,
            catalog::{
                discount_model::{Discount, DiscountState}, item_discount_model::ItemDiscount, item_group_model::ItemGroup, item_model::Item, item_variant_model::ItemVariant, variant_type_model::VariantType, variant_value_model::VariantValue
            },
            common::{
                brand_model::Brand, channel_model::Channel, tax_group_model::TaxGroup,
                tax_model::Tax,
            },
            finance::{cost_center_model::CostCenter, payment_method_model::PaymentMethod},
            purchases::{
                expense_model::Expense, purchase_category_model::PurchaseCategory,
                supplier_model::Supplier,
            },
            sales::{
                cart_model::Cart, customer_model::Customer,
                sales_charge_type_model::SalesChargeType, sales_order_model::SalesOrder,
            },
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

    async fn item_categories(
        &self,
        first: Option<i32>,
        offset: Option<i32>,
        context: &AppState,
    ) -> FieldResult<Vec<ItemGroup>> {
        super::catalog::item_group_queries::item_categories(first, offset, context).await
    }

    async fn items_category(&self, id: DbUuid, context: &AppState) -> FieldResult<ItemGroup> {
        super::catalog::item_group_queries::items_category(id, context).await
    }

    async fn items(
        &self,
        first: Option<i32>,
        offset: Option<i32>,
        context: &AppState,
    ) -> FieldResult<Vec<Item>> {
        super::catalog::item_queries::items(first, offset, context).await
    }

    async fn item(&self, id: DbUuid, context: &AppState) -> FieldResult<Item> {
        super::catalog::item_queries::item(id, context).await
    }

    async fn purchase_categories(
        &self,
        first: Option<i32>,
        offset: Option<i32>,
        context: &AppState,
    ) -> FieldResult<Vec<PurchaseCategory>> {
        super::purchases::purchase_category_queries::purchase_categories(first, offset, context).await
    }

    async fn purchase_category(&self, id: DbUuid, context: &AppState) -> FieldResult<PurchaseCategory> {
        super::purchases::purchase_category_queries::purchase_category(id, context).await
    }

    async fn all_purchase_categories(&self, context: &AppState) -> FieldResult<Vec<PurchaseCategory>> {
        super::purchases::purchase_category_queries::all_purchase_categories(context).await
    }

    async fn users(
        &self,
        first: Option<i32>,
        offset: Option<i32>,
        context: &AppState,
    ) -> FieldResult<Vec<User>> {
        super::auth::user_queries::users(first, offset, context).await
    }

    async fn user(&self, id: DbUuid, context: &AppState) -> FieldResult<User> {
        super::auth::user_queries::user(id, context).await
    }

    async fn customers(
        &self,
        first: Option<i32>,
        offset: Option<i32>,
        context: &AppState,
    ) -> FieldResult<Vec<Customer>> {
        super::sales::customer_queries::customers(first, offset, context).await
    }

    async fn total_customers(&self, context: &AppState) -> FieldResult<i32> {
        super::sales::customer_queries::total_customers(context).await
    }

    async fn customer(&self, id: DbUuid, context: &AppState) -> FieldResult<Customer> {
        super::sales::customer_queries::customer(id, context).await
    }

    async fn customer_by_phone(&self, phone: String, context: &AppState) -> FieldResult<Customer> {
        super::sales::customer_queries::customer_by_phone(phone, context).await
    }

    async fn sales_orders(
        &self,
        first: Option<i32>,
        offset: Option<i32>,
        context: &AppState,
    ) -> FieldResult<Vec<SalesOrder>> {
        super::sales::sales_order_queries::sales_orders(first, offset, context).await
    }

    async fn total_sales_orders(&self, context: &AppState) -> FieldResult<i32> {
        super::sales::sales_order_queries::total_sales_orders(context).await
    }

    async fn sales_order(&self, id: DbUuid, context: &AppState) -> FieldResult<SalesOrder> {
        super::sales::sales_order_queries::sales_order(id, context).await
    }

    async fn carts(
        &self,
        first: Option<i32>,
        offset: Option<i32>,
        context: &AppState,
    ) -> FieldResult<Vec<Cart>> {
        super::sales::cart_queries::carts(first, offset, context).await
    }

    async fn total_carts(&self, context: &AppState) -> FieldResult<i32> {
        super::sales::cart_queries::total_carts(context).await
    }

    async fn cart(&self, id: DbUuid, context: &AppState) -> FieldResult<Cart> {
        super::sales::cart_queries::cart(id, context).await
    }

    async fn taxes(
        &self,
        first: Option<i32>,
        offset: Option<i32>,
        context: &AppState,
    ) -> FieldResult<Vec<Tax>> {
        super::common::tax_queries::taxes(first, offset, context).await
    }

    async fn total_taxes(&self, context: &AppState) -> FieldResult<i32> {
        super::common::tax_queries::total_taxes(context).await
    }

    async fn tax(&self, id: DbUuid, context: &AppState) -> FieldResult<Tax> {
        super::common::tax_queries::tax(id, context).await
    }

    async fn tax_groups(
        &self,
        first: Option<i32>,
        offset: Option<i32>,
        context: &AppState,
    ) -> FieldResult<Vec<TaxGroup>> {
        super::common::tax_group_queries::tax_groups(first, offset, context).await
    }

    async fn total_tax_groups(&self, context: &AppState) -> FieldResult<i32> {
        super::common::tax_group_queries::total_tax_groups(context).await
    }

    async fn tax_group(&self, id: DbUuid, context: &AppState) -> FieldResult<TaxGroup> {
        super::common::tax_group_queries::tax_group(id, context).await
    }

    async fn channels(&self, context: &AppState) -> FieldResult<Vec<Channel>> {
        super::common::channel_queries::get_channels(context).await
    }

    async fn active_channels(&self, context: &AppState) -> FieldResult<Vec<Channel>> {
        super::common::channel_queries::get_active_channels(context).await
    }

    async fn channel(&self, id: DbUuid, context: &AppState) -> FieldResult<Channel> {
        super::common::channel_queries::get_channel(id, context).await
    }

    async fn brands(&self, context: &AppState) -> FieldResult<Vec<Brand>> {
        super::common::brand_queries::get_brands(context).await
    }

    async fn active_brands(&self, context: &AppState) -> FieldResult<Vec<Brand>> {
        super::common::brand_queries::get_active_brands(context).await
    }

    async fn brand(&self, id: DbUuid, context: &AppState) -> FieldResult<Brand> {
        super::common::brand_queries::get_brand(id, context).await
    }

    async fn analytics_overview(
        &self,
        days: Option<i32>,
        context: &AppState,
    ) -> FieldResult<AnalyticsOverview> {
        super::analytics::analytics_queries::analytics_overview(days, context).await
    }

    async fn suppliers(
        &self,
        first: Option<i32>,
        offset: Option<i32>,
        context: &AppState,
    ) -> FieldResult<Vec<Supplier>> {
        super::purchases::supplier_queries::suppliers(first, offset, context).await
    }

    async fn total_suppliers(&self, context: &AppState) -> FieldResult<i32> {
        super::purchases::supplier_queries::total_suppliers(context).await
    }

    async fn supplier(&self, id: DbUuid, context: &AppState) -> FieldResult<Supplier> {
        super::purchases::supplier_queries::supplier(id, context).await
    }

    async fn expenses(
        &self,
        first: Option<i32>,
        offset: Option<i32>,
        cost_center_id: Option<DbUuid>,
        start_date: Option<String>,
        end_date: Option<String>,
        context: &AppState,
    ) -> FieldResult<Vec<Expense>> {
        super::purchases::expense_queries::expenses(
            first,
            offset,
            cost_center_id,
            start_date,
            end_date,
            context,
        ).await
    }

    async fn total_expenses(
        &self,
        cost_center_id: Option<DbUuid>,
        start_date: Option<String>,
        end_date: Option<String>,
        context: &AppState,
    ) -> FieldResult<i32> {
        super::purchases::expense_queries::total_expenses(
            cost_center_id,
            start_date,
            end_date,
            context,
        ).await
    }

    async fn expense(&self, id: DbUuid, context: &AppState) -> FieldResult<Expense> {
        super::purchases::expense_queries::expense(id, context).await
    }

    async fn expenses_by_category(
        &self,
        category_id: DbUuid,
        first: Option<i32>,
        offset: Option<i32>,
        context: &AppState,
    ) -> FieldResult<Vec<Expense>> {
        super::purchases::expense_queries::expenses_by_category(category_id, first, offset, context).await
    }

    // Cost Center Queries
    async fn cost_centers(
        &self,
        first: Option<i32>,
        offset: Option<i32>,
        context: &AppState,
    ) -> FieldResult<Vec<CostCenter>> {
        super::finance::cost_center_queries::cost_centers(first, offset, context).await
    }

    async fn cost_center(&self, id: DbUuid, context: &AppState) -> FieldResult<CostCenter> {
        super::finance::cost_center_queries::cost_center(id, context).await
    }

    async fn all_cost_centers(&self, context: &AppState) -> FieldResult<Vec<CostCenter>> {
        super::finance::cost_center_queries::all_cost_centers(context).await
    }

    async fn total_cost_centers(&self, context: &AppState) -> FieldResult<i32> {
        super::finance::cost_center_queries::total_cost_centers(context).await
    }

    // Payment Method Queries
    async fn payment_methods(
        &self,
        first: Option<i32>,
        offset: Option<i32>,
        context: &AppState,
    ) -> FieldResult<Vec<PaymentMethod>> {
        super::finance::payment_method_queries::payment_methods(first, offset, context).await
    }

    async fn payment_method(&self, id: DbUuid, context: &AppState) -> FieldResult<PaymentMethod> {
        super::finance::payment_method_queries::payment_method(id, context).await
    }

    async fn all_payment_methods(&self, context: &AppState) -> FieldResult<Vec<PaymentMethod>> {
        super::finance::payment_method_queries::all_payment_methods(context).await
    }

    async fn total_payment_methods(&self, context: &AppState) -> FieldResult<i32> {
        super::finance::payment_method_queries::total_payment_methods(context).await
    }

    async fn sales_order_payments(
        &self,
        order_id: DbUuid,
        context: &AppState,
    ) -> FieldResult<Vec<crate::core::models::finance::sales_order_payment_model::SalesOrderPayment>>
    {
        super::finance::sales_order_payment_queries::sales_order_payments(context, order_id).await
    }

    // Add new discount queries
    async fn discounts(
        &self,
        first: Option<i32>,
        offset: Option<i32>,
        state: Option<DiscountState>,
        context: &AppState,
    ) -> FieldResult<Vec<Discount>> {
        super::catalog::discount_queries::discounts(first, offset, state, context).await
    }

    async fn discount(&self, id: DbUuid, context: &AppState) -> FieldResult<Discount> {
        super::catalog::discount_queries::discount(id, context).await
    }

    // Sales Charge Type Queries
    async fn sales_charge_types(
        &self,
        first: Option<i32>,
        offset: Option<i32>,
        context: &AppState,
    ) -> FieldResult<Vec<SalesChargeType>> {
        super::sales::sales_charge_type_queries::sales_charge_types(first, offset, context).await
    }

    async fn sales_charge_type(&self, id: DbUuid, context: &AppState) -> FieldResult<SalesChargeType> {
        super::sales::sales_charge_type_queries::sales_charge_type(id, context).await
    }

    async fn sales_charge_types_count(&self, context: &AppState) -> FieldResult<i32> {
        super::sales::sales_charge_type_queries::sales_charge_types_count(context).await
    }

    // Variant Type Queries
    async fn variant_types(
        &self,
        first: Option<i32>,
        offset: Option<i32>,
        context: &AppState,
    ) -> FieldResult<Vec<VariantType>> {
        variant_type_queries::get_variant_types(first, offset, context).await
    }

    async fn variant_type(&self, id: DbUuid, context: &AppState) -> FieldResult<VariantType> {
        variant_type_queries::get_variant_type(id, context).await
    }

    async fn total_variant_types(&self, context: &AppState) -> FieldResult<i32> {
        variant_type_queries::get_total_variant_types(context).await
    }

    // Variant Value Queries
    async fn variant_values(
        &self,
        variant_type_id: Option<DbUuid>,
        first: Option<i32>,
        offset: Option<i32>,
        context: &AppState,
    ) -> FieldResult<Vec<VariantValue>> {
        variant_value_queries::get_variant_values(variant_type_id, first, offset, context).await
    }

    async fn variant_value(&self, id: DbUuid, context: &AppState) -> FieldResult<VariantValue> {
        variant_value_queries::get_variant_value(id, context).await
    }

    // Item Variant Queries
    async fn item_variants(
        &self,
        item_id: Option<DbUuid>,
        first: Option<i32>,
        offset: Option<i32>,
        context: &AppState,
    ) -> FieldResult<Vec<ItemVariant>> {
        item_variant_queries::get_item_variants(item_id, first, offset, context).await
    }

    async fn item_variant(&self, id: DbUuid, context: &AppState) -> FieldResult<ItemVariant> {
        item_variant_queries::get_item_variant(id, context).await
    }

    // Item Discount Queries
    async fn item_discounts(
        &self,
        item_id: DbUuid,
        context: &AppState,
    ) -> FieldResult<Vec<ItemDiscount>> {
        super::catalog::item_discount::ItemDiscountQuery::item_discounts(context, item_id).await
    }

    async fn discount_items(
        &self,
        discount_id: DbUuid,
        context: &AppState,
    ) -> FieldResult<Vec<ItemDiscount>> {
        super::catalog::item_discount::ItemDiscountQuery::discount_items(context, discount_id).await
    }
}
