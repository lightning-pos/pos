use chrono::NaiveDateTime;
use juniper::graphql_object;

use crate::{
    core::{
        models::common::tax_model::Tax,
        types::{db_uuid::DbUuid, percentage::Percentage},
    },
    AppState,
};

#[graphql_object(context = AppState)]
impl Tax {
    pub fn id(&self) -> DbUuid {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn rate(&self) -> Percentage {
        self.rate
    }

    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }
}
