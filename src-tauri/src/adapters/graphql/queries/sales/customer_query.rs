use chrono::NaiveDateTime;
use juniper::graphql_object;

use crate::{
    core::{models::sales::customer_model::Customer, types::db_uuid::DbUuid},
    AppState,
};

#[graphql_object(context = AppState)]
impl Customer {
    pub fn id(&self) -> DbUuid {
        self.id
    }

    pub fn full_name(&self) -> String {
        self.full_name.clone()
    }

    pub fn email(&self) -> Option<String> {
        self.email.clone()
    }

    pub fn phone(&self) -> Option<String> {
        self.phone.clone()
    }

    pub fn address(&self) -> Option<String> {
        self.address.clone()
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }
}
