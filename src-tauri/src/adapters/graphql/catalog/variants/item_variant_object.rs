use chrono::NaiveDateTime;
use diesel::prelude::*;
use juniper::{graphql_object, FieldResult};

use crate::core::models::catalog::item_model::Item;
use crate::core::models::catalog::item_variant_model::ItemVariant;
use crate::core::models::catalog::variant_value_model::VariantValue;
use crate::core::types::{db_uuid::DbUuid, money::Money};
use crate::schema::{item_variant_values, items, variant_values};
use crate::AppState;

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

    pub fn item(&self, context: &AppState) -> FieldResult<Item> {
        let mut service = context.service.lock().unwrap();
        let item = items::table
            .find(&self.item_id)
            .select(Item::as_select())
            .get_result::<Item>(&mut service.conn)?;
        Ok(item)
    }

    pub fn variant_values(&self, context: &AppState) -> FieldResult<Vec<VariantValue>> {
        let mut service = context.service.lock().unwrap();

        let value_ids = item_variant_values::table
            .filter(item_variant_values::item_variant_id.eq(self.id))
            .select(item_variant_values::variant_value_id)
            .load::<DbUuid>(&mut service.conn)?;

        let values = variant_values::table
            .filter(variant_values::id.eq_any(value_ids))
            .select(VariantValue::as_select())
            .load::<VariantValue>(&mut service.conn)?;

        Ok(values)
    }

    pub fn final_price(&self, context: &AppState) -> FieldResult<Money> {
        let mut service = context.service.lock().unwrap();
        let item = items::table
            .find(&self.item_id)
            .select(Item::as_select())
            .get_result::<Item>(&mut service.conn)?;

        let adjustment = self.price_adjustment.unwrap_or(Money::from(0));
        let final_price = item.price + adjustment;

        Ok(final_price)
    }
}
