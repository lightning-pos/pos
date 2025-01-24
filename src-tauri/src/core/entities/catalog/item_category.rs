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

#[derive(Debug, Clone, Copy, AsExpression, FromStr, GraphQLEnum)]
#[diesel(sql_type = diesel::sql_types::Text)]
pub enum ItemCategoryState {
    Active,
    Inactive,
}

impl Queryable<Text, diesel::sqlite::Sqlite> for ItemCategoryState {
    type Row = String;

    fn build(row: Self::Row) -> diesel::deserialize::Result<Self> {
        Ok(ItemCategoryState::from_str(&row)?)
    }
}

impl ToSql<Text, diesel::sqlite::Sqlite> for ItemCategoryState {
    fn to_sql<'b>(
        &'b self,
        out: &mut Output<'b, '_, diesel::sqlite::Sqlite>,
    ) -> diesel::serialize::Result {
        let s = match self {
            ItemCategoryState::Active => "active",
            ItemCategoryState::Inactive => "inactive",
        };
        out.set_value(s);
        Ok(IsNull::No)
    }
}
