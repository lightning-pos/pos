use std::str::FromStr;

use crate::schema::item_categories;
use chrono::NaiveDateTime;
use derive_more::derive::FromStr;
use diesel::{
    expression::AsExpression,
    prelude::{AsChangeset, Insertable, Queryable, Selectable},
    serialize::{IsNull, Output, ToSql},
    sql_types::Text,
};
use juniper::{GraphQLEnum, GraphQLInputObject};
use lightning_macros::{QueryableEnum, ToSqlEnum};

#[derive(Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = item_categories)]
pub struct ItemCategory {
    pub id: String,
    pub name: String,
    pub state: ItemCategoryState,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct NewItemCategory {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, AsChangeset, GraphQLInputObject)]
#[diesel(table_name = item_categories)]
pub struct UpdateItemCategory {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub state: Option<ItemCategoryState>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Copy, AsExpression, FromStr, GraphQLEnum, ToSqlEnum, QueryableEnum)]
#[diesel(sql_type = diesel::sql_types::Text)]
pub enum ItemCategoryState {
    Active,
    Inactive,
}
