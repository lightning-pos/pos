use diesel::{prelude::Insertable, Associations, Queryable};
use juniper::GraphQLInputObject;

use crate::core::models::catalog::{
    item_variant_model::ItemVariant, variant_value_model::VariantValue,
};
use crate::core::types::db_uuid::DbUuid;
use crate::schema::item_variant_values;

#[derive(Debug, Clone, Queryable, Insertable, Associations)]
#[diesel(table_name = item_variant_values)]
#[diesel(belongs_to(ItemVariant, foreign_key = item_variant_id))]
#[diesel(belongs_to(VariantValue, foreign_key = variant_value_id))]
pub struct ItemVariantValue {
    pub item_variant_id: DbUuid,
    pub variant_value_id: DbUuid,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct ItemVariantValueInput {
    pub item_variant_id: DbUuid,
    pub variant_value_id: DbUuid,
}
