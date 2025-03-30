use chrono::NaiveDateTime;
use juniper::graphql_object;

use crate::{
    core::{models::common::brand_model::Brand, types::db_uuid::DbUuid},
    AppState,
};

#[graphql_object(context = AppState)]
impl Brand {
    pub fn id(&self) -> DbUuid {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }
}
