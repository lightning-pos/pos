use chrono::NaiveDateTime;
use sea_query::{Expr, Query, SqliteQueryBuilder};
use juniper::{graphql_object, FieldResult};

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        models::common::{
            tax_group_model::{TaxGroup, TaxGroupTaxes},
            tax_model::{Tax, Taxes},
        },
        types::db_uuid::DbUuid,
    },
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
        let service = context.service.lock().unwrap();

        // First, get the tax IDs for this tax group
        let tax_ids_query = Query::select()
            .from(TaxGroupTaxes::Table)
            .column(TaxGroupTaxes::TaxId)
            .and_where(Expr::col(TaxGroupTaxes::TaxGroupId).eq(self.id.to_string()))
            .to_string(SqliteQueryBuilder);
            
        let tax_ids = service.db_adapter.query_many::<DbUuid>(&tax_ids_query, vec![])?;

        // If no tax IDs found, return empty vector
        if tax_ids.is_empty() {
            return Ok(vec![]);
        }

        // Convert tax IDs to strings for the IN clause
        let tax_id_strings: Vec<String> = tax_ids.iter().map(|id| id.to_string()).collect();
        
        // Then get the taxes with those IDs
        let taxes_query = Query::select()
            .from(Taxes::Table)
            .columns([
                Taxes::Id,
                Taxes::Name,
                Taxes::Rate,
                Taxes::Description,
                Taxes::CreatedAt,
                Taxes::UpdatedAt,
            ])
            .and_where(Expr::col(Taxes::Id).is_in(tax_id_strings))
            .to_string(SqliteQueryBuilder);
            
        let result = service.db_adapter.query_many::<Tax>(&taxes_query, vec![])?;

        Ok(result)
    }
}