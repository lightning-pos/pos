use chrono::NaiveDateTime;
use juniper::graphql_object;

use crate::{
    core::{
        models::catalog::discount_model::{Discount, DiscountScope, DiscountState, DiscountType},
        types::{db_uuid::DbUuid, money::Money},
    },
    AppState,
};

#[graphql_object(context = AppState)]
impl Discount {
    pub fn id(&self) -> DbUuid {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }

    pub fn discount_type(&self) -> DiscountType {
        self.discount_type
    }

    pub fn value(&self) -> Money {
        self.value
    }

    pub fn scope(&self) -> DiscountScope {
        self.scope
    }

    pub fn state(&self) -> DiscountState {
        self.state
    }

    pub fn start_date(&self) -> Option<NaiveDateTime> {
        self.start_date
    }

    pub fn end_date(&self) -> Option<NaiveDateTime> {
        self.end_date
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }
}
