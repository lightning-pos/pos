use chrono::NaiveDateTime;
use diesel::{
    query_dsl::methods::{FindDsl, SelectDsl},
    RunQueryDsl, SelectableHelper,
};
use juniper::{graphql_object, FieldResult};

use crate::{
    core::{
        models::catalog::{
            item_model::{Item, ItemNature, ItemState},
            item_group_model::ItemGroup,
        },
        types::{db_uuid::DbUuid, money::Money},
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

    pub fn price(&self) -> Money {
        self.price
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }

    pub fn category(&self, context: &AppState) -> FieldResult<ItemGroup> {
        let mut service = context.service.lock().unwrap();
        let result = item_categories::table
            .find(&self.category_id)
            .select(ItemGroup::as_select())
            .get_result(&mut service.conn)?;
        Ok(result)
    }
}
