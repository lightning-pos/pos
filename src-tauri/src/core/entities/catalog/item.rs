use chrono::NaiveDateTime;
use derive_more::derive::{Display, From};
use diesel::{
    expression::AsExpression,
    prelude::{AsChangeset, Insertable, Queryable},
    serialize::{IsNull, Output, ToSql},
    sql_types::Text,
    Selectable,
};
use juniper::{GraphQLEnum, GraphQLObject};

use crate::schema::items;

#[derive(Debug, Clone, Queryable, Selectable, Insertable, AsChangeset, GraphQLObject)]
#[diesel(table_name = items)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub nature: ItemNature,
    pub state: ItemState,
    pub price: i32,
    pub category_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Display, From, AsExpression, PartialEq, GraphQLEnum)]
#[diesel(sql_type = Text)]
pub enum ItemNature {
    Goods,
    Service,
}

#[derive(Debug, Clone, Display, From, AsExpression, PartialEq, GraphQLEnum)]
#[diesel(sql_type = Text)]
pub enum ItemState {
    Active,
    Inactive,
    Deleted,
}

impl From<String> for ItemState {
    fn from(s: String) -> Self {
        match s.as_str() {
            "active" => ItemState::Active,
            "inactive" => ItemState::Inactive,
            "deleted" => ItemState::Deleted,
            _ => ItemState::Inactive, // default case
        }
    }
}

impl From<String> for ItemNature {
    fn from(s: String) -> Self {
        match s.as_str() {
            "goods" => ItemNature::Goods,
            "service" => ItemNature::Service,
            _ => ItemNature::Goods, // default case
        }
    }
}

impl ToSql<Text, diesel::sqlite::Sqlite> for ItemState {
    fn to_sql<'b>(
        &'b self,
        out: &mut Output<'b, '_, diesel::sqlite::Sqlite>,
    ) -> diesel::serialize::Result {
        let s = match self {
            ItemState::Active => "active",
            ItemState::Inactive => "unactive",
            ItemState::Deleted => "deleted",
        };
        out.set_value(s);
        Ok(IsNull::No)
    }
}

impl ToSql<Text, diesel::sqlite::Sqlite> for ItemNature {
    fn to_sql<'b>(
        &'b self,
        out: &mut Output<'b, '_, diesel::sqlite::Sqlite>,
    ) -> diesel::serialize::Result {
        let s = match self {
            ItemNature::Goods => "goods",
            ItemNature::Service => "service",
        };
        out.set_value(s);
        Ok(IsNull::No)
    }
}

impl Queryable<Text, diesel::sqlite::Sqlite> for ItemState {
    type Row = String;
    fn build(row: Self::Row) -> diesel::deserialize::Result<Self> {
        Ok(ItemState::from(row))
    }
}

impl Queryable<Text, diesel::sqlite::Sqlite> for ItemNature {
    type Row = String;
    fn build(row: Self::Row) -> diesel::deserialize::Result<Self> {
        Ok(ItemNature::from(row))
    }
}
