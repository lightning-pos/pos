use juniper::{graphql_object, FieldResult};

use crate::{
    core::{
        models::{
            auth::user_model::{User, UserNewInput, UserUpdateInput},
            catalog::{
                item_group_model::{ItemGroup, ItemGroupNew, ItemGroupUpdate},
                item_model::{Item, NewItem, UpdateItem},
            },
            common::{
                brand_model::{Brand, BrandNewInput, BrandUpdateInput},
                channel_model::{Channel, ChannelNewInput, ChannelUpdateInput},
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
    fn login(username: String, password: String, context: &AppState) -> FieldResult<bool> {
        super::auth::auth_mutations::login(username, password, context)?;
        Ok(true)
    }

    fn logout(context: &AppState) -> FieldResult<bool> {
        super::auth::auth_mutations::logout(context)?;
        Ok(true)
    }

    fn add_user(user: UserNewInput, context: &AppState) -> FieldResult<User> {
        super::auth::user_mutations::add_user(user, context)
    }

    fn update_user(user: UserUpdateInput, context: &AppState) -> FieldResult<User> {
        super::auth::user_mutations::update_user(user, context)
    }

    fn delete_user(id: DbUuid, context: &AppState) -> FieldResult<i32> {
        super::auth::user_mutations::delete_user(id, context)
    }

    fn create_item(item: NewItem, context: &AppState) -> FieldResult<Item> {
        super::catalog::item_mutations::create_item(item, context)
    }

    fn update_item(item: UpdateItem, context: &AppState) -> FieldResult<Item> {
        super::catalog::item_mutations::update_item(item, context)
    }

    fn delete_item(id: DbUuid, context: &AppState) -> FieldResult<i32> {
        super::catalog::item_mutations::delete_item(id, context)
    }

    fn create_item_category(
        new_category: ItemGroupNew,
        context: &AppState,
    ) -> FieldResult<ItemGroup> {
        super::catalog::item_group_mutations::create_item_category(new_category, context)
    }

    fn update_item_category(
        category: ItemGroupUpdate,
        context: &AppState,
    ) -> FieldResult<ItemGroup> {
        super::catalog::item_group_mutations::update_item_category(category, context)
    }

    fn delete_item_category(id: DbUuid, context: &AppState) -> FieldResult<i32> {
        super::catalog::item_group_mutations::delete_item_category(id, context)
    }

    fn create_purchase_category(
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
        )
    }

    fn update_purchase_category(
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
        )
    }

    fn delete_purchase_category(&self, id: DbUuid, context: &AppState) -> FieldResult<DbUuid> {
        super::purchases::purchase_category_mutations::delete_purchase_category(id, context)
    }

    // Sales Order Mutations
    fn create_sales_order(
        sales_order: SalesOrderNewInput,
        context: &AppState,
    ) -> FieldResult<SalesOrder> {
        super::sales::sales_order_mutations::create_sales_order(sales_order, context)
    }

    fn void_sales_order(id: DbUuid, context: &AppState) -> FieldResult<SalesOrder> {
        super::sales::sales_order_mutations::void_sales_order(id, context)
    }

    // Cart Mutations
    fn create_cart(cart: CartNewInput, context: &AppState) -> FieldResult<Cart> {
        super::sales::cart_mutations::create_cart(cart, context)
    }

    fn update_cart(cart: CartUpdateInput, context: &AppState) -> FieldResult<Cart> {
        super::sales::cart_mutations::update_cart(cart, context)
    }

    fn delete_cart(id: DbUuid, context: &AppState) -> FieldResult<i32> {
        super::sales::cart_mutations::delete_cart(id, context)
    }

    // Customer Mutations
    fn create_customer(customer: CustomerNewInput, context: &AppState) -> FieldResult<Customer> {
        super::sales::customer_mutations::create_customer(customer, context)
    }

    fn update_customer(customer: CustomerUpdateInput, context: &AppState) -> FieldResult<Customer> {
        super::sales::customer_mutations::update_customer(customer, context)
    }

    fn delete_customer(id: DbUuid, context: &AppState) -> FieldResult<i32> {
        super::sales::customer_mutations::delete_customer(id, context)
    }

    // Tax Mutations
    fn create_tax(input: TaxNewInput, context: &AppState) -> FieldResult<Tax> {
        super::common::tax_mutations::create_tax(input, context)
    }

    fn update_tax(input: TaxUpdateInput, context: &AppState) -> FieldResult<Tax> {
        super::common::tax_mutations::update_tax(input, context)
    }

    fn delete_tax(id: DbUuid, context: &AppState) -> FieldResult<i32> {
        super::common::tax_mutations::delete_tax(id, context)
    }

    fn assign_tax_to_item(input: ItemTaxNewInput, context: &AppState) -> FieldResult<i32> {
        super::common::tax_mutations::assign_tax_to_item(input, context)
    }

    fn remove_tax_from_item(
        item_id: DbUuid,
        tax_id: DbUuid,
        context: &AppState,
    ) -> FieldResult<i32> {
        super::common::tax_mutations::remove_tax_from_item(item_id, tax_id, context)
    }

    // Supplier Mutations
    fn create_supplier(supplier: SupplierNewInput, context: &AppState) -> FieldResult<Supplier> {
        super::purchases::supplier_mutations::create_supplier(supplier, context)
    }

    fn update_supplier(supplier: SupplierUpdateInput, context: &AppState) -> FieldResult<Supplier> {
        super::purchases::supplier_mutations::update_supplier(supplier, context)
    }

    fn delete_supplier(id: DbUuid, context: &AppState) -> FieldResult<i32> {
        super::purchases::supplier_mutations::delete_supplier(id, context)
    }

    // Expense Mutations
    fn create_expense(expense: ExpenseNewInput, context: &AppState) -> FieldResult<Expense> {
        super::purchases::expense_mutations::create_expense(expense, context)
    }

    fn update_expense(expense: ExpenseUpdateInput, context: &AppState) -> FieldResult<Expense> {
        super::purchases::expense_mutations::update_expense(expense, context)
    }

    fn delete_expense(id: DbUuid, context: &AppState) -> FieldResult<i32> {
        super::purchases::expense_mutations::delete_expense(id, context)
    }

    // Channel Mutations
    fn create_channel(input: ChannelNewInput, context: &AppState) -> FieldResult<Channel> {
        super::common::channel_mutations::create_channel(input, context)
    }

    fn update_channel(input: ChannelUpdateInput, context: &AppState) -> FieldResult<Channel> {
        super::common::channel_mutations::update_channel(input, context)
    }

    fn delete_channel(id: DbUuid, context: &AppState) -> FieldResult<i32> {
        super::common::channel_mutations::delete_channel(id, context)
    }

    // Brand Mutations
    fn create_brand(input: BrandNewInput, context: &AppState) -> FieldResult<Brand> {
        super::common::brand_mutations::create_brand(input, context)
    }

    fn update_brand(input: BrandUpdateInput, context: &AppState) -> FieldResult<Brand> {
        super::common::brand_mutations::update_brand(input, context)
    }

    fn delete_brand(id: DbUuid, context: &AppState) -> FieldResult<i32> {
        super::common::brand_mutations::delete_brand(id, context)
    }

    // Cost Center Mutations
    fn create_cost_center(
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
        )
    }

    fn update_cost_center(
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
        )
    }

    fn delete_cost_center(&self, id: DbUuid, context: &AppState) -> FieldResult<DbUuid> {
        super::finance::cost_center_mutations::delete_cost_center(id, context)
    }

    // Payment Method Mutations
    fn create_payment_method(
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
        )
    }

    fn update_payment_method(
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
        )
    }

    fn delete_payment_method(&self, id: DbUuid, context: &AppState) -> FieldResult<DbUuid> {
        super::finance::payment_method_mutations::delete_payment_method(id, context)
    }
}
