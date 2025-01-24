use chrono::NaiveDateTime;
use diesel::{
    query_dsl::methods::{FindDsl, SelectDsl},
    RunQueryDsl, SelectableHelper,
};
use juniper::{graphql_object, FieldResult};

use crate::{
    core::{
        entities::catalog::{
            item::{Item, ItemNature, ItemState},
            item_category::ItemCategory,
        },
        types::db_uuid::DbUuid,
    },
    schema::item_categories,
    AppState,
};

#[graphql_object(context = AppState)]
impl Item {
    pub fn id(&self) -> DbUuid {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }

    pub fn nature(&self) -> ItemNature {
        self.nature
    }

    pub fn state(&self) -> ItemState {
        self.state
    }

    pub fn price(&self) -> i32 {
        self.price
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }

    pub fn category(&self, context: &AppState) -> FieldResult<ItemCategory> {
        let mut service = context.service.lock().unwrap();
        let result = item_categories::table
            .find(&self.category_id)
            .select(ItemCategory::as_select())
            .get_result(&mut service.conn)?;
        Ok(result)
    }
}
