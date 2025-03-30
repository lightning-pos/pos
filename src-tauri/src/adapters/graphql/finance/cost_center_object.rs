use chrono::NaiveDateTime;
use juniper::graphql_object;

use crate::core::{
    models::finance::cost_center_model::{CostCenter, CostCenterState},
    types::db_uuid::DbUuid,
};

#[graphql_object(description = "Cost Center")]
impl CostCenter {
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

    pub fn state(&self) -> CostCenterState {
        self.state
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }
}
