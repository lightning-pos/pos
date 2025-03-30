use chrono::NaiveDateTime;
use juniper::graphql_object;

use crate::core::{
    models::finance::payment_method_model::{PaymentMethod, PaymentMethodState},
    types::db_uuid::DbUuid,
};

#[graphql_object(description = "Payment Method")]
impl PaymentMethod {
    pub fn id(&self) -> DbUuid {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn code(&self) -> String {
        self.code.clone()
    }

    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }

    pub fn state(&self) -> PaymentMethodState {
        self.state
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }
}
