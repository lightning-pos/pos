pub mod auth;
pub mod catalog;
pub mod sales;

use juniper::{graphql_object, FieldResult};

use crate::{
    core::{
        models::{
            auth::user_model::{User, UserNewInput, UserUpdateInput},
            catalog::{
                item_group_model::{ItemGroup, ItemGroupNew, ItemGroupUpdate},
                item_model::{Item, NewItem, UpdateItem},
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
        auth::auth_mutations::login(username, password, context)?;
        Ok(true)
    }

    fn logout(context: &AppState) -> FieldResult<bool> {
        auth::auth_mutations::logout(context)?;
        Ok(true)
    }

    fn add_user(user: UserNewInput, context: &AppState) -> FieldResult<User> {
        auth::user_mutations::add_user(user, context)
    }

    fn update_user(user: UserUpdateInput, context: &AppState) -> FieldResult<User> {
        auth::user_mutations::update_user(user, context)
    }

    fn delete_user(id: DbUuid, context: &AppState) -> FieldResult<i32> {
        auth::user_mutations::delete_user(id, context)
    }

    fn create_item(item: NewItem, context: &AppState) -> FieldResult<Item> {
        catalog::item_mutations::create_item(item, context)
    }

    fn update_item(item: UpdateItem, context: &AppState) -> FieldResult<Item> {
        catalog::item_mutations::update_item(item, context)
    }

    fn delete_item(id: DbUuid, context: &AppState) -> FieldResult<i32> {
        catalog::item_mutations::delete_item(id, context)
    }

    fn create_item_category(
        new_category: ItemGroupNew,
        context: &AppState,
    ) -> FieldResult<ItemGroup> {
        catalog::item_group_mutations::create_item_category(new_category, context)
    }

    fn update_item_category(
        category: ItemGroupUpdate,
        context: &AppState,
    ) -> FieldResult<ItemGroup> {
        catalog::item_group_mutations::update_item_category(category, context)
    }

    fn delete_item_category(id: DbUuid, context: &AppState) -> FieldResult<i32> {
        catalog::item_group_mutations::delete_item_category(id, context)
    }

    // Sales Order Mutations
    fn create_sales_order(
        sales_order: SalesOrderNewInput,
        context: &AppState,
    ) -> FieldResult<SalesOrder> {
        sales::sales_order_mutations::create_sales_order(sales_order, context)
    }

    fn void_sales_order(id: DbUuid, context: &AppState) -> FieldResult<SalesOrder> {
        sales::sales_order_mutations::void_sales_order(id, context)
    }

    // Cart Mutations
    fn create_cart(cart: CartNewInput, context: &AppState) -> FieldResult<Cart> {
        sales::cart_mutations::create_cart(cart, context)
    }

    fn update_cart(cart: CartUpdateInput, context: &AppState) -> FieldResult<Cart> {
        sales::cart_mutations::update_cart(cart, context)
    }

    fn delete_cart(id: DbUuid, context: &AppState) -> FieldResult<i32> {
        sales::cart_mutations::delete_cart(id, context)
    }

    // Customer Mutations
    fn create_customer(customer: CustomerNewInput, context: &AppState) -> FieldResult<Customer> {
        sales::customer_mutations::create_customer(customer, context)
    }

    fn update_customer(customer: CustomerUpdateInput, context: &AppState) -> FieldResult<Customer> {
        sales::customer_mutations::update_customer(customer, context)
    }

    fn delete_customer(id: DbUuid, context: &AppState) -> FieldResult<i32> {
        sales::customer_mutations::delete_customer(id, context)
    }
}
