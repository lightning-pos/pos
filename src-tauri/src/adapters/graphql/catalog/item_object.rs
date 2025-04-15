use chrono::NaiveDateTime;
use sea_query::{Expr, Query, SqliteQueryBuilder};
use juniper::{graphql_object, FieldResult};

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        models::{
            catalog::{
                item_group_model::{ItemGroup, ItemCategories},
                item_model::{Item, ItemNature, ItemState},
                item_variant_model::{ItemVariant, ItemVariants},
            },
            common::tax_model::{Tax, Taxes, ItemTaxes},
        },
        types::{db_uuid::DbUuid, money::Money},
    },
    AppState,
};

#[graphql_object(context = AppState)]
impl Item {
    pub fn id(&self) -> DbUuid {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }

    pub fn nature(&self) -> ItemNature {
        self.nature
    }

    pub fn state(&self) -> ItemState {
        self.state
    }

    pub fn price(&self) -> Money {
        self.price
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }

    pub fn category(&self, context: &AppState) -> FieldResult<ItemGroup> {
        let service = context.service.lock().unwrap();
        
        let query = Query::select()
            .from(ItemCategories::Table)
            .columns([
                ItemCategories::Id,
                ItemCategories::Name,
                ItemCategories::Description,
                ItemCategories::State,
                ItemCategories::CreatedAt,
                ItemCategories::UpdatedAt,
            ])
            .and_where(Expr::col(ItemCategories::Id).eq(self.category_id.to_string()))
            .to_string(SqliteQueryBuilder);
            
        let result = service.db_adapter.query_one::<ItemGroup>(&query, vec![])?;
        
        Ok(result)
    }

    pub fn taxes(&self, context: &AppState) -> FieldResult<Vec<Tax>> {
        let service = context.service.lock().unwrap();

        // First, get the tax IDs for this item
        let tax_ids_query = Query::select()
            .from(ItemTaxes::Table)
            .column(ItemTaxes::TaxId)
            .and_where(Expr::col(ItemTaxes::ItemId).eq(self.id.to_string()))
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
            
        let taxes = service.db_adapter.query_many::<Tax>(&taxes_query, vec![])?;

        Ok(taxes)
    }

    pub fn variants(&self, context: &AppState) -> FieldResult<Vec<ItemVariant>> {
        let service = context.service.lock().unwrap();
        
        let query = Query::select()
            .from(ItemVariants::Table)
            .columns([
                ItemVariants::Id,
                ItemVariants::ItemId,
                ItemVariants::Sku,
                ItemVariants::PriceAdjustment,
                ItemVariants::IsDefault,
                ItemVariants::CreatedAt,
                ItemVariants::UpdatedAt,
            ])
            .and_where(Expr::col(ItemVariants::ItemId).eq(self.id.to_string()))
            .to_string(SqliteQueryBuilder);
            
        let variants = service.db_adapter.query_many::<ItemVariant>(&query, vec![])?;
        
        Ok(variants)
    }

    pub fn has_variants(&self, context: &AppState) -> FieldResult<bool> {
        let service = context.service.lock().unwrap();
        
        // Count query to check if variants exist
        let count_query = Query::select()
            .from(ItemVariants::Table)
            .expr(Expr::col(ItemVariants::Id).count())
            .and_where(Expr::col(ItemVariants::ItemId).eq(self.id.to_string()))
            .to_string(SqliteQueryBuilder);
            
        let count = service.db_adapter.query_one::<i64>(&count_query, vec![])?;
        
        Ok(count > 0)
    }

    pub fn default_variant(&self, context: &AppState) -> FieldResult<Option<ItemVariant>> {
        let service = context.service.lock().unwrap();
        
        let query = Query::select()
            .from(ItemVariants::Table)
            .columns([
                ItemVariants::Id,
                ItemVariants::ItemId,
                ItemVariants::Sku,
                ItemVariants::PriceAdjustment,
                ItemVariants::IsDefault,
                ItemVariants::CreatedAt,
                ItemVariants::UpdatedAt,
            ])
            .and_where(Expr::col(ItemVariants::ItemId).eq(self.id.to_string()))
            .and_where(Expr::col(ItemVariants::IsDefault).eq(true))
            .to_string(SqliteQueryBuilder);
            
        let default_variant = service.db_adapter.query_optional::<ItemVariant>(&query, vec![])?;
        
        Ok(default_variant)
    }
}