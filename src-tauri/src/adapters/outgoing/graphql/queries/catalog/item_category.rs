use chrono::NaiveDateTime;
use juniper::graphql_object;

use crate::{
    core::entities::catalog::item_category::{ItemCategory, ItemCategoryState},
    AppState,
};

#[graphql_object(context = AppState)]
impl ItemCategory {
    pub fn id(&self) -> String {
        self.id.to_string()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }

    pub fn state(&self) -> ItemCategoryState {
        self.state
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }
}