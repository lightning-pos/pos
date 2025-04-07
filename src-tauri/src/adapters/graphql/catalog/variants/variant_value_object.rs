use chrono::NaiveDateTime;
use diesel::prelude::*;
use juniper::{graphql_object, FieldResult};

use crate::core::models::catalog::variant_type_model::VariantType;
use crate::core::models::catalog::variant_value_model::VariantValue;
use crate::core::types::db_uuid::DbUuid;
use crate::schema::variant_types;
use crate::AppState;

#[graphql_object(context = AppState)]
impl VariantValue {
    pub fn id(&self) -> DbUuid {
        self.id
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }

    pub fn display_order(&self) -> i32 {
        self.display_order
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }

    pub fn variant_type(&self, context: &AppState) -> FieldResult<VariantType> {
        let mut service = context.service.lock().unwrap();
        let variant_type = variant_types::table
            .find(&self.variant_type_id)
            .select(VariantType::as_select())
            .get_result::<VariantType>(&mut service.conn)?;
        Ok(variant_type)
    }
}
