use chrono::NaiveDateTime;
use diesel::{
    prelude::{AsChangeset, Insertable, Queryable},
    Associations, Selectable,
};
use juniper::GraphQLInputObject;

use crate::core::models::catalog::item_model::Item;
use crate::core::types::{db_uuid::DbUuid, money::Money};
use crate::schema::item_variants;

#[derive(Debug, Clone, Queryable, Selectable, Insertable, Associations)]
#[diesel(table_name = item_variants)]
#[diesel(belongs_to(Item, foreign_key = item_id))]
pub struct ItemVariant {
    pub id: DbUuid,
    pub item_id: DbUuid,
    pub sku: Option<String>,
    pub price_adjustment: Option<Money>,
    pub is_default: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct ItemVariantNewInput {
    pub item_id: DbUuid,
    pub sku: Option<String>,
    pub price_adjustment: Option<Money>,
    pub is_default: Option<bool>,
    pub variant_value_ids: Vec<DbUuid>,
}

#[derive(Debug, Clone, AsChangeset, GraphQLInputObject)]
#[diesel(table_name = item_variants)]
pub struct ItemVariantUpdateInput {
    pub id: DbUuid,
    pub sku: Option<Option<String>>,
    pub price_adjustment: Option<Option<Money>>,
    pub is_default: Option<bool>,
    pub updated_at: Option<NaiveDateTime>,
}
