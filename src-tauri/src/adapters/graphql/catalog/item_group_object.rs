use chrono::NaiveDateTime;
use juniper::graphql_object;

use crate::{
    core::{
        models::catalog::item_group_model::{ItemGroup, ItemGroupState},
        types::db_uuid::DbUuid,
    },
    AppState,
};

#[graphql_object(context = AppState)]
impl ItemGroup {
    pub fn id(&self) -> DbUuid {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }

    pub fn state(&self) -> ItemGroupState {
        self.state
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }
}
