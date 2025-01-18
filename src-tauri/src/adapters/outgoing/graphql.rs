use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use juniper::{graphql_object, Context, EmptyMutation, EmptySubscription, FieldResult, RootNode};

use crate::{
    core::entities::catalog::item_category::ItemCategory, schema::item_categories, AppState,
};

pub type Schema = RootNode<'static, Query, EmptyMutation<AppState>, EmptySubscription<AppState>>;

impl Context for AppState {}

#[derive(Default)]
pub struct Query;

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
}
