//! Item Variant Value Model
//!
//! This module defines the junction table between item variants and variant values.
//!
//! # Design Decisions
//!
//! ## Many-to-Many Relationship
//! - Each item variant can have multiple variant values (e.g., Small, Red)
//! - Each variant value can be used in multiple item variants
//! - This junction table connects them with a many-to-many relationship
//!
//! ## Validation Rules
//! - Each item variant can only have one value from each variant type
//! - This is enforced in the commands, not at the database level
//! - This prevents invalid combinations (e.g., both Small and Medium)
//!
//! ## No Additional Fields
//! - This is a pure junction table with only foreign keys
//! - No additional metadata is stored at this level
//! - All variant-specific data is stored in the ItemVariant table

use diesel::{prelude::Insertable, Associations, Queryable};
use juniper::GraphQLInputObject;
use sea_query::Iden;

use crate::core::models::catalog::{
    item_variant_model::ItemVariant, variant_value_model::VariantValue,
};
use crate::core::types::db_uuid::DbUuid;
use crate::schema::item_variant_values;

/// Junction table connecting item variants to variant values.
///
/// This represents the many-to-many relationship between variants and values.
/// For example, a "Small-Red T-shirt" variant would have two entries in this table,
/// one connecting to "Small" and another connecting to "Red".
///
/// # Fields
/// - `item_variant_id`: Reference to the item variant
/// - `variant_value_id`: Reference to the variant value
///
/// # Relationships
/// - Belongs to one ItemVariant
/// - Belongs to one VariantValue
///
/// # Validation
/// The system enforces that each item variant can only have one value from each variant type.
/// This validation happens in the command layer, not at the database level.
#[derive(Debug, Clone, Queryable, Insertable, Associations)]
#[diesel(table_name = item_variant_values)]
#[diesel(belongs_to(ItemVariant, foreign_key = item_variant_id))]
#[diesel(belongs_to(VariantValue, foreign_key = variant_value_id))]
pub struct ItemVariantValue {
    pub item_variant_id: DbUuid,
    pub variant_value_id: DbUuid,
}

// Define table and column identifiers for SeaQuery
#[derive(Iden)]
pub enum ItemVariantValues {
    Table,
    ItemVariantId,
    VariantValueId,
}

/// Input type for creating a new item variant value association.
///
/// Used when manually associating a variant value with an item variant.
/// The system validates that the item variant doesn't already have a value
/// from the same variant type.
///
/// # Fields
/// - `item_variant_id`: ID of the item variant
/// - `variant_value_id`: ID of the variant value to associate
#[derive(Debug, Clone, GraphQLInputObject)]
pub struct ItemVariantValueInput {
    pub item_variant_id: DbUuid,
    pub variant_value_id: DbUuid,
}
