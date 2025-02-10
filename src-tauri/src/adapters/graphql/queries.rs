use diesel::{dsl::count, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use juniper::{graphql_object, FieldResult};

use crate::{
    adapters::graphql::Query,
    core::{
        models::{
            auth::user_model::User,
            catalog::{item_group_model::ItemGroup, item_model::Item},
            common::tax_model::Tax,
            sales::{cart_model::Cart, customer_model::Customer, sales_order_model::SalesOrder},
        },
        types::db_uuid::DbUuid,
    },
    schema::{carts, customers, item_categories, items, sales_orders, taxes, users},
    AppState,
};

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
        let mut service = context.service.lock().unwrap();

        let mut query = item_categories::table.into_boxed();

        // Apply pagination if parameters are provided
        if let Some(limit) = first {
            query = query.limit(limit as i64);
        }
        if let Some(off) = offset {
            query = query.offset(off as i64);
        }

        let result = query
            .select(ItemGroup::as_select())
            .load::<ItemGroup>(&mut service.conn)?;

        Ok(result)
    }

    fn items_category(&self, id: DbUuid, context: &AppState) -> FieldResult<ItemGroup> {
        let mut service = context.service.lock().unwrap();
        let result = item_categories::table
            .filter(item_categories::id.eq(id))
            .select(ItemGroup::as_select())
            .get_result(&mut service.conn)?;
        Ok(result)
    }

    fn items(
        &self,
        first: Option<i32>,
        offset: Option<i32>,
        context: &AppState,
    ) -> FieldResult<Vec<Item>> {
        let mut service = context.service.lock().unwrap();

        let mut query = items::table.into_boxed();

        if let Some(limit) = first {
            query = query.limit(limit as i64);
        }

        if let Some(off) = offset {
            query = query.offset(off as i64);
        }

        let result = query
            .select(Item::as_select())
            .load::<Item>(&mut service.conn)?;
        Ok(result)
    }

    fn item(&self, id: DbUuid, context: &AppState) -> FieldResult<Item> {
        let mut service = context.service.lock().unwrap();
        let result = items::table
            .filter(items::id.eq(id))
            .select(Item::as_select())
            .get_result(&mut service.conn)?;
        Ok(result)
    }

    fn users(
        &self,
        first: Option<i32>,
        offset: Option<i32>,
        context: &AppState,
    ) -> FieldResult<Vec<User>> {
        let mut service = context.service.lock().unwrap();
        let mut query = users::table.into_boxed();
        if let Some(limit) = first {
            query = query.limit(limit as i64);
        }
        if let Some(off) = offset {
            query = query.offset(off as i64);
        }
        let result = query
            .select(User::as_select())
            .load::<User>(&mut service.conn)?;
        Ok(result)
    }

    fn user(&self, id: DbUuid, context: &AppState) -> FieldResult<User> {
        let mut service = context.service.lock().unwrap();
        let result = users::table
            .filter(users::id.eq(id))
            .select(User::as_select())
            .get_result(&mut service.conn)?;
        Ok(result)
    }

    fn customers(
        &self,
        first: Option<i32>,
        offset: Option<i32>,
        context: &AppState,
    ) -> FieldResult<Vec<Customer>> {
        let mut service = context.service.lock().unwrap();
        let mut query = customers::table.into_boxed();
        if let Some(limit) = first {
            query = query.limit(limit as i64);
        }
        if let Some(off) = offset {
            query = query.offset(off as i64);
        }
        let result = query
            .select(Customer::as_select())
            .load::<Customer>(&mut service.conn)?;
        Ok(result)
    }

    fn total_customers(&self, context: &AppState) -> FieldResult<i32> {
        let mut service = context.service.lock().unwrap();
        let result: i64 = customers::table
            .select(count(customers::id))
            .get_result(&mut service.conn)?;
        Ok(result as i32)
    }

    fn customer(&self, id: DbUuid, context: &AppState) -> FieldResult<Customer> {
        let mut service = context.service.lock().unwrap();
        let result = customers::table
            .filter(customers::id.eq(id))
            .select(Customer::as_select())
            .get_result(&mut service.conn)?;
        Ok(result)
    }

    fn customer_by_phone(&self, phone: String, context: &AppState) -> FieldResult<Customer> {
        let mut service = context.service.lock().unwrap();
        let result = customers::table
            .filter(customers::phone.eq(phone))
            .select(Customer::as_select())
            .get_result(&mut service.conn)?;
        Ok(result)
    }

    fn sales_orders(
        &self,
        first: Option<i32>,
        offset: Option<i32>,
        context: &AppState,
    ) -> FieldResult<Vec<SalesOrder>> {
        let mut service = context.service.lock().unwrap();
        let mut query = sales_orders::table.into_boxed();
        if let Some(limit) = first {
            query = query.limit(limit as i64);
        }
        if let Some(off) = offset {
            query = query.offset(off as i64);
        }
        let result = query
            .select(SalesOrder::as_select())
            .load::<SalesOrder>(&mut service.conn)?;
        Ok(result)
    }

    fn total_sales_orders(&self, context: &AppState) -> FieldResult<i32> {
        let mut service = context.service.lock().unwrap();
        let result: i64 = sales_orders::table
            .select(count(sales_orders::id))
            .get_result(&mut service.conn)?;
        Ok(result as i32)
    }

    fn sales_order(&self, id: DbUuid, context: &AppState) -> FieldResult<SalesOrder> {
        let mut service = context.service.lock().unwrap();
        let result = sales_orders::table
            .filter(sales_orders::id.eq(id))
            .select(SalesOrder::as_select())
            .get_result(&mut service.conn)?;
        Ok(result)
    }

    fn carts(
        &self,
        first: Option<i32>,
        offset: Option<i32>,
        context: &AppState,
    ) -> FieldResult<Vec<Cart>> {
        let mut service = context.service.lock().unwrap();
        let mut query = carts::table.into_boxed();
        if let Some(limit) = first {
            query = query.limit(limit as i64);
        }
        if let Some(off) = offset {
            query = query.offset(off as i64);
        }
        let result = query
            .select(Cart::as_select())
            .load::<Cart>(&mut service.conn)?;
        Ok(result)
    }

    fn total_carts(&self, context: &AppState) -> FieldResult<i32> {
        let mut service = context.service.lock().unwrap();
        let result: i64 = carts::table
            .select(count(carts::id))
            .get_result(&mut service.conn)?;
        Ok(result as i32)
    }

    fn cart(&self, id: DbUuid, context: &AppState) -> FieldResult<Cart> {
        let mut service = context.service.lock().unwrap();
        let result = carts::table
            .find(id)
            .select(Cart::as_select())
            .get_result::<Cart>(&mut service.conn)?;
        Ok(result)
    }

    // Tax Queries
    fn taxes(
        &self,
        first: Option<i32>,
        offset: Option<i32>,
        context: &AppState,
    ) -> FieldResult<Vec<Tax>> {
        let mut service = context.service.lock().unwrap();

        let mut query = taxes::table.order(taxes::created_at.desc()).into_boxed();

        // Apply pagination if parameters are provided
        if let Some(limit) = first {
            query = query.limit(limit as i64);
        }
        if let Some(off) = offset {
            query = query.offset(off as i64);
        }

        let result = query
            .select(Tax::as_select())
            .load::<Tax>(&mut service.conn)?;

        Ok(result)
    }

    fn tax(&self, id: DbUuid, context: &AppState) -> FieldResult<Tax> {
        let mut service = context.service.lock().unwrap();
        let result = taxes::table
            .find(id)
            .select(Tax::as_select())
            .get_result::<Tax>(&mut service.conn)?;
        Ok(result)
    }

    fn total_taxes(&self, context: &AppState) -> FieldResult<i32> {
        let mut service = context.service.lock().unwrap();
        let result: i64 = taxes::table
            .select(count(taxes::id))
            .get_result(&mut service.conn)?;
        Ok(result as i32)
    }
}
