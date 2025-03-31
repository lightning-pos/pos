use crate::{
    core::types::{db_uuid::DbUuid, money::Money},
    schema::discounts,
};
use chrono::NaiveDateTime;
use diesel::prelude::{AsChangeset, Insertable, Queryable, Selectable};
use diesel_derive_enum::DbEnum;
use juniper::{GraphQLEnum, GraphQLInputObject};

#[derive(Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = discounts)]
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

#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = discounts)]
pub struct DiscountUpdateChangeset {
    pub id: DbUuid,
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub discount_type: Option<DiscountType>,
    pub value: Option<Money>,
    pub scope: Option<DiscountScope>,
    pub state: Option<DiscountState>,
    pub start_date: Option<Option<NaiveDateTime>>,
    pub end_date: Option<Option<NaiveDateTime>>,
    pub updated_at: NaiveDateTime, // Automatically set in command
}

// Using DbEnum derive for mapping Rust enums to database enum types
// Make sure these enum types are created in the database via migrations

#[derive(Debug, Clone, Copy, DbEnum, GraphQLEnum, PartialEq, Eq)]
#[DbValueStyle = "PascalCase"] // Keep this for mapping Rust PascalCase to DB TEXT PascalCase
pub enum DiscountType {
    Percentage,
    FixedAmount,
}

#[derive(Debug, Clone, Copy, DbEnum, GraphQLEnum, PartialEq, Eq)]
#[DbValueStyle = "PascalCase"]
pub enum DiscountScope {
    AllItems,
    // Future scopes might require relations:
    // SpecificItems,
    // SpecificCategories,
}

#[derive(Debug, Clone, Copy, DbEnum, GraphQLEnum, PartialEq, Eq)]
#[DbValueStyle = "PascalCase"]
pub enum DiscountState {
    Active,
    Inactive,
    Scheduled, // If start_date is in the future
    Expired,   // If end_date is in the past
}
