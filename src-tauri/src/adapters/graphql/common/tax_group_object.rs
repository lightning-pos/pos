use chrono::NaiveDateTime;
use diesel::{dsl, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use juniper::{graphql_object, FieldResult};

use crate::{
    core::{
        models::common::{tax_group_model::TaxGroup, tax_model::Tax},
        types::db_uuid::DbUuid,
    },
    schema::{tax_group_taxes, taxes},
    AppState,
};

#[graphql_object(context = AppState)]
impl TaxGroup {
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

    pub fn taxes(&self, context: &AppState) -> FieldResult<Vec<Tax>> {
        let mut service = context.service.lock().unwrap();

        let tax_ids = tax_group_taxes::table
            .filter(tax_group_taxes::tax_group_id.eq(self.id))
            .select(tax_group_taxes::tax_id)
            .load::<DbUuid>(&mut service.conn)?;

        let result = taxes::table
            .filter(taxes::id.eq_any(tax_ids))
            .select(Tax::as_select())
            .load::<Tax>(&mut service.conn)?;

        Ok(result)
    }
}
