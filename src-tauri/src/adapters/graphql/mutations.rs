use juniper::{graphql_object, FieldResult};

use crate::{
    adapters::graphql::catalog::{
        item_discount::ItemDiscountObject,
        variants::{item_variant_mutations, variant_type_mutations, variant_value_mutations},
    },
    core::{
        models::{
            auth::user_model::{User, UserNewInput, UserUpdateInput},
            catalog::{
                discount_model::{Discount, DiscountNewInput, DiscountUpdateInput},
                item_discount_model::ItemDiscountNewInput,
                item_group_model::{ItemGroup, ItemGroupNew, ItemGroupUpdate},
                item_model::{Item, NewItem, UpdateItem},
                item_variant_model::{ItemVariant, ItemVariantNewInput, ItemVariantUpdateInput},
                variant_type_model::{VariantType, VariantTypeNewInput, VariantTypeUpdateInput},
                variant_value_model::{
                    VariantValue, VariantValueNewInput, VariantValueUpdateInput,
                },
            },
            common::{
                brand_model::{Brand, BrandNewInput, BrandUpdateInput},
                channel_model::{Channel, ChannelNewInput, ChannelUpdateInput},
                tax_group_model::{TaxGroup, TaxGroupNewInput, TaxGroupUpdateInput},
                tax_model::{ItemTaxNewInput, Tax, TaxNewInput, TaxUpdateInput},
            },
            finance::{
                cost_center_model::{CostCenter, CostCenterState},
                payment_method_model::{PaymentMethod, PaymentMethodState},
            },
            purchases::{
                expense_model::{Expense, ExpenseNewInput, ExpenseUpdateInput},
                purchase_category_model::{PurchaseCategory, PurchaseCategoryState},
                supplier_model::{Supplier, SupplierNewInput, SupplierUpdateInput},
            },
            sales::{
                cart_model::{Cart, CartNewInput, CartUpdateInput},
                customer_model::{Customer, CustomerNewInput, CustomerUpdateInput},
                sales_charge_type_model::{
                    SalesChargeType, SalesChargeTypeNewInput, SalesChargeTypeUpdateInput,
                },
                sales_order_model::{SalesOrder, SalesOrderNewInput},
            },
        },
        types::db_uuid::DbUuid,
    },
    AppState,
};

use super::Mutation;

#[graphql_object(context = AppState)]
impl Mutation {
    async fn login(username: String, password: String, context: &AppState) -> FieldResult<bool> {
        super::auth::auth_mutations::login(username, password, context).await?;
        Ok(true)
    }

    async fn logout(context: &AppState) -> FieldResult<bool> {
        super::auth::auth_mutations::logout(context).await?;
        Ok(true)
    }

    async fn add_user(user: UserNewInput, context: &AppState) -> FieldResult<User> {
        super::auth::user_mutations::add_user(user, context).await
    }

    async fn update_user(user: UserUpdateInput, context: &AppState) -> FieldResult<User> {
        super::auth::user_mutations::update_user(user, context).await
    }

    async fn delete_user(id: DbUuid, context: &AppState) -> FieldResult<i32> {
        super::auth::user_mutations::delete_user(id, context).await
    }

    async fn create_item(item: NewItem, context: &AppState) -> FieldResult<Item> {
        super::catalog::item_mutations::create_item(item, context).await
    }

    async fn update_item(item: UpdateItem, context: &AppState) -> FieldResult<Item> {
        super::catalog::item_mutations::update_item(item, context).await
    }

    async fn delete_item(id: DbUuid, context: &AppState) -> FieldResult<i32> {
        super::catalog::item_mutations::delete_item(id, context).await
    }

    async fn create_item_category(
        new_category: ItemGroupNew,
        context: &AppState,
    ) -> FieldResult<ItemGroup> {
        super::catalog::item_group_mutations::create_item_category(new_category, context).await
    }

    async fn update_item_category(
        category: ItemGroupUpdate,
        context: &AppState,
    ) -> FieldResult<ItemGroup> {
        super::catalog::item_group_mutations::update_item_category(category, context).await
    }

    async fn delete_item_category(id: DbUuid, context: &AppState) -> FieldResult<i32> {
        super::catalog::item_group_mutations::delete_item_category(id, context).await
    }

    async fn create_purchase_category(
        &self,
        name: String,
        description: Option<String>,
        state: Option<PurchaseCategoryState>,
        context: &AppState,
    ) -> FieldResult<PurchaseCategory> {
        super::purchases::purchase_category_mutations::create_purchase_category(
            name,
            description,
            state,
            context,
        ).await
    }

    async fn update_purchase_category(
        &self,
        id: DbUuid,
        name: Option<String>,
        description: Option<Option<String>>,
        state: Option<PurchaseCategoryState>,
        context: &AppState,
    ) -> FieldResult<PurchaseCategory> {
        super::purchases::purchase_category_mutations::update_purchase_category(
            id,
            name,
            description,
            state,
            context,
        ).await
    }

    async fn delete_purchase_category(&self, id: DbUuid, context: &AppState) -> FieldResult<DbUuid> {
        super::purchases::purchase_category_mutations::delete_purchase_category(id, context).await
    }

    // Sales Order Mutations
    async fn create_sales_order(
        sales_order: SalesOrderNewInput,
        context: &AppState,
    ) -> FieldResult<SalesOrder> {
        super::sales::sales_order_mutations::create_sales_order(sales_order, context).await
    }

    async fn void_sales_order(id: DbUuid, context: &AppState) -> FieldResult<SalesOrder> {
        super::sales::sales_order_mutations::void_sales_order(id, context).await
    }

    // Cart Mutations
    async fn create_cart(cart: CartNewInput, context: &AppState) -> FieldResult<Cart> {
        super::sales::cart_mutations::create_cart(cart, context).await
    }

    async fn update_cart(cart: CartUpdateInput, context: &AppState) -> FieldResult<Cart> {
        super::sales::cart_mutations::update_cart(cart, context).await
    }

    async fn delete_cart(id: DbUuid, context: &AppState) -> FieldResult<i32> {
        super::sales::cart_mutations::delete_cart(id, context).await
    }

    // Customer Mutations
    async fn create_customer(customer: CustomerNewInput, context: &AppState) -> FieldResult<Customer> {
        super::sales::customer_mutations::create_customer(customer, context).await
    }

    async fn update_customer(customer: CustomerUpdateInput, context: &AppState) -> FieldResult<Customer> {
        super::sales::customer_mutations::update_customer(customer, context).await
    }

    async fn delete_customer(id: DbUuid, context: &AppState) -> FieldResult<i32> {
        super::sales::customer_mutations::delete_customer(id, context).await
    }

    // Tax Mutations
    async fn create_tax(input: TaxNewInput, context: &AppState) -> FieldResult<Tax> {
        super::common::tax_mutations::create_tax(input, context).await
    }

    async fn update_tax(input: TaxUpdateInput, context: &AppState) -> FieldResult<Tax> {
        super::common::tax_mutations::update_tax(input, context).await
    }

    async fn delete_tax(id: DbUuid, context: &AppState) -> FieldResult<i32> {
        super::common::tax_mutations::delete_tax(id, context).await
    }

    async fn assign_tax_to_item(input: ItemTaxNewInput, context: &AppState) -> FieldResult<i32> {
        super::common::tax_mutations::assign_tax_to_item(input, context).await
    }

    async fn remove_tax_from_item(
        item_id: DbUuid,
        tax_id: DbUuid,
        context: &AppState,
    ) -> FieldResult<i32> {
        super::common::tax_mutations::remove_tax_from_item(item_id, tax_id, context).await
    }

    // Tax Group Mutations
    async fn create_tax_group(input: TaxGroupNewInput, context: &AppState) -> FieldResult<TaxGroup> {
        super::common::tax_group_mutations::create_tax_group(input, context).await
    }

    async fn update_tax_group(input: TaxGroupUpdateInput, context: &AppState) -> FieldResult<TaxGroup> {
        super::common::tax_group_mutations::update_tax_group(input, context).await
    }

    async fn delete_tax_group(id: DbUuid, context: &AppState) -> FieldResult<i32> {
        super::common::tax_group_mutations::delete_tax_group(id, context).await
    }

    async fn assign_tax_to_group(
        tax_group_id: DbUuid,
        tax_id: DbUuid,
        context: &AppState,
    ) -> FieldResult<i32> {
        super::common::tax_group_mutations::assign_tax_to_group(tax_group_id, tax_id, context).await
    }

    async fn remove_tax_from_group(
        tax_group_id: DbUuid,
        tax_id: DbUuid,
        context: &AppState,
    ) -> FieldResult<i32> {
        super::common::tax_group_mutations::remove_tax_from_group(tax_group_id, tax_id, context).await
    }

    // Supplier Mutations
    async fn create_supplier(supplier: SupplierNewInput, context: &AppState) -> FieldResult<Supplier> {
        super::purchases::supplier_mutations::create_supplier(supplier, context).await
    }

    async fn update_supplier(supplier: SupplierUpdateInput, context: &AppState) -> FieldResult<Supplier> {
        super::purchases::supplier_mutations::update_supplier(supplier, context).await
    }

    async fn delete_supplier(id: DbUuid, context: &AppState) -> FieldResult<i32> {
        super::purchases::supplier_mutations::delete_supplier(id, context).await
    }

    // Expense Mutations
    async fn create_expense(expense: ExpenseNewInput, context: &AppState) -> FieldResult<Expense> {
        super::purchases::expense_mutations::create_expense(expense, context).await
    }

    async fn update_expense(expense: ExpenseUpdateInput, context: &AppState) -> FieldResult<Expense> {
        super::purchases::expense_mutations::update_expense(expense, context).await
    }

    async fn delete_expense(id: DbUuid, context: &AppState) -> FieldResult<i32> {
        super::purchases::expense_mutations::delete_expense(id, context).await
    }

    // Channel Mutations
    async fn create_channel(input: ChannelNewInput, context: &AppState) -> FieldResult<Channel> {
        super::common::channel_mutations::create_channel(input, context).await
    }

    async fn update_channel(input: ChannelUpdateInput, context: &AppState) -> FieldResult<Channel> {
        super::common::channel_mutations::update_channel(input, context).await
    }

    async fn delete_channel(id: DbUuid, context: &AppState) -> FieldResult<i32> {
        super::common::channel_mutations::delete_channel(id, context).await
    }

    // Brand Mutations
    async fn create_brand(input: BrandNewInput, context: &AppState) -> FieldResult<Brand> {
        super::common::brand_mutations::create_brand(input, context).await
    }

    async fn update_brand(input: BrandUpdateInput, context: &AppState) -> FieldResult<Brand> {
        super::common::brand_mutations::update_brand(input, context).await
    }

    async fn delete_brand(id: DbUuid, context: &AppState) -> FieldResult<i32> {
        super::common::brand_mutations::delete_brand(id, context).await
    }

    // Cost Center Mutations
    async fn create_cost_center(
        &self,
        name: String,
        code: String,
        description: Option<String>,
        state: Option<CostCenterState>,
        context: &AppState,
    ) -> FieldResult<CostCenter> {
        super::finance::cost_center_mutations::create_cost_center(
            name,
            code,
            description,
            state,
            context,
        ).await
    }

    async fn update_cost_center(
        &self,
        id: DbUuid,
        name: Option<String>,
        code: Option<String>,
        description: Option<Option<String>>,
        state: Option<CostCenterState>,
        context: &AppState,
    ) -> FieldResult<CostCenter> {
        super::finance::cost_center_mutations::update_cost_center(
            id,
            name,
            code,
            description,
            state,
            context,
        ).await
    }

    async fn delete_cost_center(&self, id: DbUuid, context: &AppState) -> FieldResult<DbUuid> {
        super::finance::cost_center_mutations::delete_cost_center(id, context).await
    }

    // Payment Method Mutations
    async fn create_payment_method(
        &self,
        name: String,
        code: String,
        description: Option<String>,
        state: Option<PaymentMethodState>,
        context: &AppState,
    ) -> FieldResult<PaymentMethod> {
        super::finance::payment_method_mutations::create_payment_method(
            name,
            code,
            description,
            state,
            context,
        ).await
    }

    async fn update_payment_method(
        &self,
        id: DbUuid,
        name: Option<String>,
        code: Option<String>,
        description: Option<Option<String>>,
        state: Option<PaymentMethodState>,
        context: &AppState,
    ) -> FieldResult<PaymentMethod> {
        super::finance::payment_method_mutations::update_payment_method(
            id,
            name,
            code,
            description,
            state,
            context,
        ).await
    }

    async fn delete_payment_method(&self, id: DbUuid, context: &AppState) -> FieldResult<DbUuid> {
        super::finance::payment_method_mutations::delete_payment_method(id, context).await
    }

    // Sales Order Payment Mutations
    async fn create_sales_order_payment(
        &self,
        payment: crate::core::models::finance::sales_order_payment_model::SalesOrderPaymentNewInput,
        context: &AppState,
    ) -> FieldResult<crate::core::models::finance::sales_order_payment_model::SalesOrderPayment>
    {
        super::finance::sales_order_payment_mutations::create_sales_order_payment(context, payment).await
    }

    async fn update_sales_order_payment(
        &self,
        payment: crate::core::models::finance::sales_order_payment_model::SalesOrderPaymentUpdateInput,
        context: &AppState,
    ) -> FieldResult<crate::core::models::finance::sales_order_payment_model::SalesOrderPayment>
    {
        super::finance::sales_order_payment_mutations::update_sales_order_payment(context, payment).await
    }

    async fn void_sales_order_payment(
        &self,
        id: DbUuid,
        context: &AppState,
    ) -> FieldResult<crate::core::models::finance::sales_order_payment_model::SalesOrderPayment>
    {
        super::finance::sales_order_payment_mutations::void_sales_order_payment(context, id).await
    }

    // Discount Mutations
    async fn create_discount(discount: DiscountNewInput, context: &AppState) -> FieldResult<Discount> {
        super::catalog::discount_mutations::create_discount(discount, context).await
    }

    async fn update_discount(discount: DiscountUpdateInput, context: &AppState) -> FieldResult<Discount> {
        super::catalog::discount_mutations::update_discount(discount, context).await
    }

    async fn delete_discount(id: DbUuid, context: &AppState) -> FieldResult<i32> {
        super::catalog::discount_mutations::delete_discount(id, context).await
    }

    // Sales Charge Type Mutations
    async fn create_sales_charge_type(
        charge_type: SalesChargeTypeNewInput,
        context: &AppState,
    ) -> FieldResult<SalesChargeType> {
        super::sales::sales_charge_type_mutations::create_sales_charge_type(charge_type, context).await
    }

    async fn update_sales_charge_type(
        charge_type: SalesChargeTypeUpdateInput,
        context: &AppState,
    ) -> FieldResult<SalesChargeType> {
        super::sales::sales_charge_type_mutations::update_sales_charge_type(charge_type, context).await
    }

    async fn delete_sales_charge_type(id: DbUuid, context: &AppState) -> FieldResult<bool> {
        super::sales::sales_charge_type_mutations::delete_sales_charge_type(id, context).await
    }

    // Variant Type Mutations
    async fn create_variant_type(
        input: VariantTypeNewInput,
        context: &AppState,
    ) -> FieldResult<VariantType> {
        variant_type_mutations::create_variant_type(input, context).await
    }

    async fn update_variant_type(
        input: VariantTypeUpdateInput,
        context: &AppState,
    ) -> FieldResult<VariantType> {
        variant_type_mutations::update_variant_type(input, context).await
    }

    async fn delete_variant_type(id: DbUuid, context: &AppState) -> FieldResult<i32> {
        variant_type_mutations::delete_variant_type(id, context).await
    }

    // Variant Value Mutations
    async fn create_variant_value(
        input: VariantValueNewInput,
        context: &AppState,
    ) -> FieldResult<VariantValue> {
        variant_value_mutations::create_variant_value(input, context).await
    }

    async fn update_variant_value(
        input: VariantValueUpdateInput,
        context: &AppState,
    ) -> FieldResult<VariantValue> {
        variant_value_mutations::update_variant_value(input, context).await
    }

    async fn delete_variant_value(id: DbUuid, context: &AppState) -> FieldResult<i32> {
        variant_value_mutations::delete_variant_value(id, context).await
    }

    // Item Variant Mutations
    async fn create_item_variant(
        input: ItemVariantNewInput,
        context: &AppState,
    ) -> FieldResult<ItemVariant> {
        item_variant_mutations::create_item_variant(input, context).await
    }

    async fn update_item_variant(
        input: ItemVariantUpdateInput,
        context: &AppState,
    ) -> FieldResult<ItemVariant> {
        item_variant_mutations::update_item_variant(input, context).await
    }

    async fn delete_item_variant(id: DbUuid, context: &AppState) -> FieldResult<i32> {
        item_variant_mutations::delete_item_variant(id, context).await
    }

    async fn assign_variant_value_to_item_variant(
        item_variant_id: DbUuid,
        variant_value_id: DbUuid,
        context: &AppState,
    ) -> FieldResult<i32> {
        item_variant_mutations::assign_variant_value_to_item_variant(
            item_variant_id,
            variant_value_id,
            context,
        ).await
    }

    async fn remove_variant_value_from_item_variant(
        item_variant_id: DbUuid,
        variant_value_id: DbUuid,
        context: &AppState,
    ) -> FieldResult<i32> {
        item_variant_mutations::remove_variant_value_from_item_variant(
            item_variant_id,
            variant_value_id,
            context,
        ).await
    }

    // Item Discount Mutations
    async fn add_item_discount(
        &self,
        item_discount: ItemDiscountNewInput,
        context: &AppState,
    ) -> FieldResult<ItemDiscountObject> {
        super::catalog::item_discount::ItemDiscountMutation::add_item_discount(
            context,
            item_discount,
        ).await
    }

    async fn remove_item_discount(
        &self,
        item_id: DbUuid,
        discount_id: DbUuid,
        context: &AppState,
    ) -> FieldResult<bool> {
        super::catalog::item_discount::ItemDiscountMutation::remove_item_discount(
            context,
            item_id,
            discount_id,
        ).await
    }
}
