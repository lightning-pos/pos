use chrono::NaiveDateTime;
use juniper::{graphql_object, FieldResult};

use crate::core::commands::catalog::variant_value_commands::ListVariantValuesCommand;
use crate::core::commands::Command;
use crate::core::models::catalog::variant_type_model::VariantType;
use crate::core::models::catalog::variant_value_model::VariantValue;
use crate::core::types::db_uuid::DbUuid;
use crate::AppState;

#[graphql_object(context = AppState)]
impl VariantType {
    pub fn id(&self) -> DbUuid {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
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

    pub fn values(&self, context: &AppState) -> FieldResult<Vec<VariantValue>> {
        let mut service = context.service.lock().unwrap();
        let command = ListVariantValuesCommand {
            variant_type_id: Some(self.id),
        };
        let values = command.exec(&mut service)?;
        Ok(values)
    }
}
