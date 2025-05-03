use crate::{adapters::outgoing::database::{FromLibsqlValue, FromRow}, core::{db::SeaQueryCrudTrait, types::{db_uuid::DbUuid, money::Money}}};
use chrono::NaiveDateTime;
use derive_more::Display;
use juniper::{GraphQLEnum, GraphQLInputObject};
use lightning_macros::{LibsqlEnum, LibsqlFromRow, SeaQueryCrud, SeaQueryEnum, SeaQueryModel};

#[derive(Debug, SeaQueryModel, LibsqlFromRow, SeaQueryCrud)]
pub struct Discount {
    pub id: DbUuid,
    pub name: String,
    pub description: Option<String>,
    pub discount_type: DiscountType,
    pub value: Money,
    pub scope: DiscountScope,
    pub state: DiscountState,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct DiscountNewInput {
    pub name: String,
    pub description: Option<String>,
    pub discount_type: DiscountType,
    pub value: Money,
    pub scope: DiscountScope,
    pub state: Option<DiscountState>, // Default state might be handled elsewhere or set to Active
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct DiscountUpdateInput {
    pub id: DbUuid,
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub discount_type: Option<DiscountType>,
    pub value: Option<Money>,
    pub scope: Option<DiscountScope>,
    pub state: Option<DiscountState>,
    pub start_date: Option<Option<NaiveDateTime>>, // Option<Option<...>> allows setting to None
    pub end_date: Option<Option<NaiveDateTime>>,
}

// Using DbEnum derive for mapping Rust enums to database enum types
// Make sure these enum types are created in the database via migrations

#[derive(Debug, Clone, Copy, GraphQLEnum, PartialEq, Eq, Display, SeaQueryEnum, LibsqlEnum)]
pub enum DiscountType {
    Percentage,
    FixedAmount,
}

#[derive(Debug, Clone, Copy, GraphQLEnum, PartialEq, Eq, Display, SeaQueryEnum, LibsqlEnum)]
pub enum DiscountScope {
    AllItems,
    SpecificItems, // Added for item-specific discounts
                   // Future scopes might include:
                   // SpecificCategories,
}

#[derive(Debug, Clone, Copy, GraphQLEnum, PartialEq, Eq, Display, SeaQueryEnum, LibsqlEnum)]
pub enum DiscountState {
    Active,
    Inactive,
    Scheduled, // If start_date is in the future
    Expired,   // If end_date is in the past
}
