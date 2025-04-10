use juniper::{graphql_object, FieldResult};

use crate::{
    adapters::graphql::catalog::{
        item_discount::ItemDiscountObject,
        variants::{item_variant_queries, variant_type_queries, variant_value_queries},
    },
    adapters::graphql::Query,
    core::{
        models::{
            auth::user_model::User,
            catalog::{
                discount_model::{Discount, DiscountState},
                item_group_model::ItemGroup,
                item_model::Item,
                item_variant_model::ItemVariant,
                variant_type_model::VariantType,
                variant_value_model::VariantValue,
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

    fn all_purchase_categories(&self, context: &AppState) -> FieldResult<Vec<PurchaseCategory>> {
        super::purchases::purchase_category_queries::all_purchase_categories(context)
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

    fn tax_groups(
        &self,
        first: Option<i32>,
        offset: Option<i32>,
        context: &AppState,
    ) -> FieldResult<Vec<TaxGroup>> {
        super::common::tax_group_queries::tax_groups(first, offset, context)
    }

    fn total_tax_groups(&self, context: &AppState) -> FieldResult<i32> {
        super::common::tax_group_queries::total_tax_groups(context)
    }

    fn tax_group(&self, id: DbUuid, context: &AppState) -> FieldResult<TaxGroup> {
        super::common::tax_group_queries::tax_group(id, context)
    }

    fn channels(&self, context: &AppState) -> FieldResult<Vec<Channel>> {
        super::common::channel_queries::get_channels(context)
    }

    fn active_channels(&self, context: &AppState) -> FieldResult<Vec<Channel>> {
        super::common::channel_queries::get_active_channels(context)
    }

    fn channel(&self, id: DbUuid, context: &AppState) -> FieldResult<Channel> {
        super::common::channel_queries::get_channel(id, context)
    }

    fn brands(&self, context: &AppState) -> FieldResult<Vec<Brand>> {
        super::common::brand_queries::get_brands(context)
    }

    fn active_brands(&self, context: &AppState) -> FieldResult<Vec<Brand>> {
        super::common::brand_queries::get_active_brands(context)
    }

    fn brand(&self, id: DbUuid, context: &AppState) -> FieldResult<Brand> {
        super::common::brand_queries::get_brand(id, context)
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
        )
    }

    fn total_expenses(
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
        )
    }

    fn expense(&self, id: DbUuid, context: &AppState) -> FieldResult<Expense> {
        super::purchases::expense_queries::expense(id, context)
    }

    fn expenses_by_category(
        &self,
        category_id: DbUuid,
        first: Option<i32>,
        offset: Option<i32>,
        context: &AppState,
    ) -> FieldResult<Vec<Expense>> {
        super::purchases::expense_queries::expenses_by_category(category_id, first, offset, context)
    }

    // Cost Center Queries
    fn cost_centers(
        &self,
        first: Option<i32>,
        offset: Option<i32>,
        context: &AppState,
    ) -> FieldResult<Vec<CostCenter>> {
        super::finance::cost_center_queries::cost_centers(first, offset, context)
    }

    fn cost_center(&self, id: DbUuid, context: &AppState) -> FieldResult<CostCenter> {
        super::finance::cost_center_queries::cost_center(id, context)
    }

    fn all_cost_centers(&self, context: &AppState) -> FieldResult<Vec<CostCenter>> {
        super::finance::cost_center_queries::all_cost_centers(context)
    }

    fn total_cost_centers(&self, context: &AppState) -> FieldResult<i32> {
        super::finance::cost_center_queries::total_cost_centers(context)
    }

    // Payment Method Queries
    fn payment_methods(
        &self,
        first: Option<i32>,
        offset: Option<i32>,
        context: &AppState,
    ) -> FieldResult<Vec<PaymentMethod>> {
        super::finance::payment_method_queries::payment_methods(first, offset, context)
    }

    fn payment_method(&self, id: DbUuid, context: &AppState) -> FieldResult<PaymentMethod> {
        super::finance::payment_method_queries::payment_method(id, context)
    }

    fn all_payment_methods(&self, context: &AppState) -> FieldResult<Vec<PaymentMethod>> {
        super::finance::payment_method_queries::all_payment_methods(context)
    }

    fn total_payment_methods(&self, context: &AppState) -> FieldResult<i32> {
        super::finance::payment_method_queries::total_payment_methods(context)
    }

    fn sales_order_payments(
        &self,
        order_id: DbUuid,
        context: &AppState,
    ) -> FieldResult<Vec<crate::core::models::finance::sales_order_payment_model::SalesOrderPayment>>
    {
        super::finance::sales_order_payment_queries::sales_order_payments(context, order_id)
    }

    // Add new discount queries
    fn discounts(
        &self,
        first: Option<i32>,
        offset: Option<i32>,
        state: Option<DiscountState>,
        context: &AppState,
    ) -> FieldResult<Vec<Discount>> {
        super::catalog::discount_queries::discounts(first, offset, state, context)
    }

    fn discount(&self, id: DbUuid, context: &AppState) -> FieldResult<Discount> {
        super::catalog::discount_queries::discount(id, context)
    }

    // Sales Charge Type Queries
    fn sales_charge_types(
        &self,
        first: Option<i32>,
        offset: Option<i32>,
        context: &AppState,
    ) -> FieldResult<Vec<SalesChargeType>> {
        super::sales::sales_charge_type_queries::sales_charge_types(first, offset, context)
    }

    fn sales_charge_type(&self, id: DbUuid, context: &AppState) -> FieldResult<SalesChargeType> {
        super::sales::sales_charge_type_queries::sales_charge_type(id, context)
    }

    fn sales_charge_types_count(&self, context: &AppState) -> FieldResult<i32> {
        super::sales::sales_charge_type_queries::sales_charge_types_count(context)
    }

    // Variant Type Queries
    fn variant_types(
        &self,
        first: Option<i32>,
        offset: Option<i32>,
        context: &AppState,
    ) -> FieldResult<Vec<VariantType>> {
        variant_type_queries::get_variant_types(first, offset, context)
    }

    fn variant_type(&self, id: DbUuid, context: &AppState) -> FieldResult<VariantType> {
        variant_type_queries::get_variant_type(id, context)
    }

    fn total_variant_types(&self, context: &AppState) -> FieldResult<i32> {
        variant_type_queries::get_total_variant_types(context)
    }

    // Variant Value Queries
    fn variant_values(
        &self,
        variant_type_id: Option<DbUuid>,
        first: Option<i32>,
        offset: Option<i32>,
        context: &AppState,
    ) -> FieldResult<Vec<VariantValue>> {
        variant_value_queries::get_variant_values(variant_type_id, first, offset, context)
    }

    fn variant_value(&self, id: DbUuid, context: &AppState) -> FieldResult<VariantValue> {
        variant_value_queries::get_variant_value(id, context)
    }

    // Item Variant Queries
    fn item_variants(
        &self,
        item_id: Option<DbUuid>,
        first: Option<i32>,
        offset: Option<i32>,
        context: &AppState,
    ) -> FieldResult<Vec<ItemVariant>> {
        item_variant_queries::get_item_variants(item_id, first, offset, context)
    }

    fn item_variant(&self, id: DbUuid, context: &AppState) -> FieldResult<ItemVariant> {
        item_variant_queries::get_item_variant(id, context)
    }

    // Item Discount Queries
    fn item_discounts(
        &self,
        item_id: DbUuid,
        context: &AppState,
    ) -> FieldResult<Vec<ItemDiscountObject>> {
        super::catalog::item_discount::ItemDiscountQuery::item_discounts(context, item_id)
    }

    fn discount_items(
        &self,
        discount_id: DbUuid,
        context: &AppState,
    ) -> FieldResult<Vec<ItemDiscountObject>> {
        super::catalog::item_discount::ItemDiscountQuery::discount_items(context, discount_id)
    }
}
