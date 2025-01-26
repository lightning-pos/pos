pub mod auth;
pub mod catalog;

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use juniper::{graphql_object, FieldResult};

use crate::{
    adapters::outgoing::graphql::Query,
    core::{
        entities::{
            auth::user::User,
            catalog::{item::Item, item_category::ItemCategory},
        },
        types::db_uuid::DbUuid,
    },
    schema::{item_categories, items, users},
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
    ) -> FieldResult<Vec<ItemCategory>> {
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
            .select(ItemCategory::as_select())
            .load::<ItemCategory>(&mut service.conn)?;

        Ok(result)
    }

    fn items_category(&self, id: DbUuid, context: &AppState) -> FieldResult<ItemCategory> {
        let mut service = context.service.lock().unwrap();
        let result = item_categories::table
            .filter(item_categories::id.eq(id))
            .select(ItemCategory::as_select())
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
}
