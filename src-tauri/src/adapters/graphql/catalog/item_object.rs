use chrono::NaiveDateTime;
use sea_query::{Expr, Func, Query};
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

    pub async fn category(&self, context: &AppState) -> FieldResult<ItemGroup> {
        let service = context.service.lock().await;

        let mut query_builder = Query::select();
        let query = query_builder
            .from(ItemCategories::Table)
            .columns([
                ItemCategories::Id,
                ItemCategories::Name,
                ItemCategories::Description,
                ItemCategories::State,
                ItemCategories::CreatedAt,
                ItemCategories::UpdatedAt,
            ])
            .and_where(Expr::col(ItemCategories::Id).eq(self.category_id.to_string()));
        let result = service.db_adapter.query_one::<ItemGroup>(&query).await?;
        Ok(result)
    }

    pub async fn taxes(&self, context: &AppState) -> FieldResult<Vec<Tax>> {
        let service = context.service.lock().await;

        // First, get the tax IDs for this item
        let mut tax_ids_query_builder = Query::select();
        let tax_ids_query = tax_ids_query_builder
            .from(ItemTaxes::Table)
            .column(ItemTaxes::TaxId)
            .and_where(Expr::col(ItemTaxes::ItemId).eq(self.id.to_string()));

        let tax_ids = service.db_adapter.query_many::<DbUuid>(&tax_ids_query).await?;

        // If no tax IDs found, return empty vector
        if tax_ids.is_empty() {
            return Ok(vec![]);
        }

        // Convert tax IDs to strings for the IN clause
        let tax_id_strings: Vec<String> = tax_ids.iter().map(|id| id.to_string()).collect();

        // Then get the taxes with those IDs
        let mut taxes_query_builder = Query::select();
        let taxes_query = taxes_query_builder
            .from(Taxes::Table)
            .columns([
                Taxes::Id,
                Taxes::Name,
                Taxes::Rate,
                Taxes::Description,
                Taxes::CreatedAt,
                Taxes::UpdatedAt,
            ])
            .and_where(Expr::col(Taxes::Id).is_in(tax_id_strings));

        let taxes = service.db_adapter.query_many::<Tax>(&taxes_query).await?;

        Ok(taxes)
    }

    pub async fn variants(&self, context: &AppState) -> FieldResult<Vec<ItemVariant>> {
        let service = context.service.lock().await;

        let mut query = Query::select();
        let query = query
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
            .and_where(Expr::col(ItemVariants::ItemId).eq(self.id.to_string()));

        let variants = service.db_adapter.query_many::<ItemVariant>(&query).await?;

        Ok(variants)
    }

    pub async fn has_variants(&self, context: &AppState) -> FieldResult<bool> {
        let service = context.service.lock().await;

        // Count query to check if variants exist
        let mut count_query_builder = Query::select();
        let count_query = count_query_builder
            .from(ItemVariants::Table)
            .expr(Func::count(Expr::col(ItemVariants::Id)))
            .and_where(Expr::col(ItemVariants::ItemId).eq(self.id.to_string()));

        let count = service.db_adapter.query_one::<i64>(&count_query).await?;

        Ok(count > 0)
    }

    pub async fn default_variant(&self, context: &AppState) -> FieldResult<Option<ItemVariant>> {
        let service = context.service.lock().await;

        let mut query_builder = Query::select();
        let query = query_builder
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
            .and_where(Expr::col(ItemVariants::IsDefault).eq(true));

        let default_variant = service.db_adapter.query_optional::<ItemVariant>(&query).await?;

        Ok(default_variant)
    }
}

pub fn assert_send<T: Send>(_: &T ) {}
