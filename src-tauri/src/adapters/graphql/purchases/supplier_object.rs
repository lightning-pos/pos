use chrono::NaiveDateTime;
use juniper::graphql_object;

use crate::{
    core::{models::purchases::supplier_model::Supplier, types::db_uuid::DbUuid},
    AppState,
};

#[graphql_object(context = AppState)]
impl Supplier {
    pub fn id(&self) -> DbUuid {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn address(&self) -> Option<String> {
        self.address.clone()
    }

    pub fn phone(&self) -> Option<String> {
        self.phone.clone()
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }
}
