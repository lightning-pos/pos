use chrono::NaiveDateTime;
use diesel::prelude::*;
use juniper::{graphql_object, FieldResult};

use crate::{
    core::{
        models::sales::{cart_model::Cart, customer_model::Customer},
        types::db_uuid::DbUuid,
    },
    schema::customers,
    AppState,
};

#[graphql_object(context = AppState)]
impl Cart {
    pub fn id(&self) -> DbUuid {
        self.id
    }

    pub fn cart_data(&self) -> String {
        self.cart_data.clone()
    }

    pub fn customer_id(&self) -> Option<DbUuid> {
        self.customer_id
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }

    // Relationships
    pub fn customer(&self, context: &AppState) -> FieldResult<Option<Customer>> {
        if let Some(customer_id) = self.customer_id {
            let mut service = context.service.lock().unwrap();
            let customer = customers::table
                .find(customer_id)
                .select(Customer::as_select())
                .first::<Customer>(&mut service.conn)?;
            Ok(Some(customer))
        } else {
            Ok(None)
        }
    }
}
