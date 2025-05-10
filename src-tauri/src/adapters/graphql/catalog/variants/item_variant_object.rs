use chrono::NaiveDateTime;
use sea_query::{Expr, Query};
use juniper::{graphql_object, FieldResult};

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        models::catalog::{
            item_model::{Item, Items},
            item_variant_model::ItemVariant,
            item_variant_value_model::ItemVariantValues,
            variant_value_model::{VariantValue, VariantValues},
        },
        types::{db_uuid::DbUuid, money::Money},
    },
    AppState,
};

#[graphql_object(context = AppState)]
impl ItemVariant {
    pub fn id(&self) -> DbUuid {
        self.id
    }

    pub fn sku(&self) -> Option<String> {
        self.sku.clone()
    }

    pub fn price_adjustment(&self) -> Option<Money> {
        self.price_adjustment
    }

    pub fn is_default(&self) -> bool {
        self.is_default
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }

    pub async fn item(&self, context: &AppState) -> FieldResult<Item> {
        let service = context.service.lock().await;

        let mut query = Query::select();
        let query = query
            .from(Items::Table)
            .columns([
                Items::Id,
                Items::Name,
                Items::Description,
                Items::Nature,
                Items::State,
                Items::Price,
                Items::CategoryId,
                Items::CreatedAt,
                Items::UpdatedAt,
            ])
            .and_where(Expr::col(Items::Id).eq(self.item_id.to_string()));

        let item = service.db_adapter.query_one::<Item>(&query).await?;

        Ok(item)
    }

    pub async fn variant_values(&self, context: &AppState) -> FieldResult<Vec<VariantValue>> {
        let service = context.service.lock().await;

        // First, get the variant value IDs for this item variant
        let mut value_ids_query = Query::select();
        let value_ids_query = value_ids_query
            .from(ItemVariantValues::Table)
            .column(ItemVariantValues::VariantValueId)
            .and_where(Expr::col(ItemVariantValues::ItemVariantId).eq(self.id.to_string()));

        let value_ids = service.db_adapter.query_many::<DbUuid>(&value_ids_query).await?;

        // If no value IDs found, return empty vector
        if value_ids.is_empty() {
            return Ok(vec![]);
        }

        // Convert value IDs to strings for the IN clause
        let value_id_strings: Vec<String> = value_ids.iter().map(|id| id.to_string()).collect();

        // Then get the variant values with those IDs
        let mut values_query = Query::select();
        let values_query = values_query
            .from(VariantValues::Table)
            .columns([
                VariantValues::Id,
                VariantValues::VariantTypeId,
                VariantValues::Value,
                VariantValues::DisplayOrder,
                VariantValues::CreatedAt,
                VariantValues::UpdatedAt,
            ])
            .and_where(Expr::col(VariantValues::Id).is_in(value_id_strings));

        let values = service.db_adapter.query_many::<VariantValue>(&values_query).await?;

        Ok(values)
    }

    pub async fn final_price(&self, context: &AppState) -> FieldResult<Money> {
        let service = context.service.lock().await;

        let mut query = Query::select();
        let query = query
            .from(Items::Table)
            .columns([
                Items::Id,
                Items::Name,
                Items::Description,
                Items::Nature,
                Items::State,
                Items::Price,
                Items::CategoryId,
                Items::CreatedAt,
                Items::UpdatedAt,
            ])
            .and_where(Expr::col(Items::Id).eq(self.item_id.to_string()));

        let item = service.db_adapter.query_one::<Item>(&query).await?;

        let adjustment = self.price_adjustment.unwrap_or(Money::from(0));
        let final_price = item.price + adjustment;

        Ok(final_price)
    }
}
