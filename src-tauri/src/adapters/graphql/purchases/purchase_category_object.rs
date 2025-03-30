use chrono::NaiveDateTime;
use juniper::graphql_object;

use crate::core::{
    models::purchases::purchase_category_model::{PurchaseCategory, PurchaseCategoryState},
    types::db_uuid::DbUuid,
};

#[graphql_object(description = "Purchase Category")]
impl PurchaseCategory {
    pub fn id(&self) -> DbUuid {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }

    pub fn state(&self) -> PurchaseCategoryState {
        self.state
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }
}
