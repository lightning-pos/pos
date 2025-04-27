//! Item Variant Model
//!
//! This module defines the data structures for item variants in the catalog.
//!
//! # Design Decisions
//!
//! ## Variant Structure
//! The variant system is designed with a flexible approach that allows:
//! - Multiple variant types (e.g., Size, Color)
//! - Multiple values per type (e.g., Small/Medium/Large for Size)
//! - Combinations of values to create specific variants (e.g., Small-Red)
//!
//! ## Database Schema
//! The database schema uses three main tables:
//! - `variant_types`: Defines categories like Size, Color
//! - `variant_values`: Defines specific values like Small, Red
//! - `item_variants`: Represents a specific combination for an item
//! - `item_variant_values`: Junction table connecting variants to values
//!
//! ## Validation Rules
//! - Each item variant can only have one value from each variant type
//! - This prevents invalid combinations (e.g., both Small and Medium)
//! - The default variant is used when no specific selection is made
//!
//! ## Price Adjustments
//! - Variants can adjust the base price of an item
//! - This allows for price differences between variants (e.g., Large costs more)
//! - The adjustment can be positive or negative

use chrono::NaiveDateTime;
use juniper::GraphQLInputObject;
use lightning_macros::{LibsqlFromRow, SeaQueryCrud, SeaQueryModel};

use crate::core::{db::SeaQueryCrudTrait, types::{db_uuid::DbUuid, money::Money}};

/// Represents a specific variant of an item with its own SKU and price adjustment.
///
/// Each ItemVariant represents a unique combination of variant values (e.g., "Small-Red").
/// The actual variant values are stored in the item_variant_values junction table.
///
/// # Fields
/// - `id`: Unique identifier
/// - `item_id`: Reference to the base item
/// - `sku`: Optional SKU code specific to this variant
/// - `price_adjustment`: Optional price difference from the base item price
/// - `is_default`: Whether this is the default variant for the item
///
/// # Relationships
/// - Belongs to one Item
/// - Has many VariantValues through ItemVariantValues
#[derive(Debug, SeaQueryModel, SeaQueryCrud, LibsqlFromRow)]
pub struct ItemVariant {
    pub id: DbUuid,
    pub item_id: DbUuid,
    pub sku: Option<String>,
    pub price_adjustment: Option<Money>,
    pub is_default: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Input type for creating a new item variant.
///
/// This includes both the variant properties and the IDs of variant values to associate.
/// The system validates that only one value per variant type is included.
///
/// # Fields
/// - `variant_value_ids`: List of variant value IDs to associate with this variant
///   (e.g., ["size-small-id", "color-red-id"])
#[derive(Debug, Clone, GraphQLInputObject)]
pub struct ItemVariantNewInput {
    pub item_id: DbUuid,
    pub sku: Option<String>,
    pub price_adjustment: Option<Money>,
    pub is_default: Option<bool>,
    pub variant_value_ids: Vec<DbUuid>,
}

/// Input type for updating an existing item variant.
///
/// Uses nested Options to handle nullable fields in updates:
/// - The outer Option determines if the field should be updated
/// - The inner Option (for nullable fields) determines if the value should be null
///
/// # Fields
/// - `id`: ID of the variant to update
/// - `sku`: Optional SKU update (Option<Option<String>>)
/// - `price_adjustment`: Optional price adjustment update
/// - `is_default`: Whether to set this as the default variant
#[derive(Debug, Clone, GraphQLInputObject)]
pub struct ItemVariantUpdateInput {
    pub id: DbUuid,
    pub sku: Option<Option<String>>,
    pub price_adjustment: Option<Option<Money>>,
    pub is_default: Option<bool>,
    pub updated_at: Option<NaiveDateTime>,
}
