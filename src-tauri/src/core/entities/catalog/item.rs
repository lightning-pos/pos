use chrono::NaiveDateTime;
use derive_more::FromStr;
use diesel::{
    expression::AsExpression,
    prelude::{AsChangeset, Associations, Insertable, Queryable},
    serialize::{IsNull, Output, ToSql},
    sql_types::Text,
    Selectable,
};
use juniper::{GraphQLEnum, GraphQLInputObject};
use lightning_macros::{QueryableEnum, ToSqlEnum};

use crate::core::entities::catalog::item_category::ItemCategory;
use crate::schema::items;

#[derive(Debug, Queryable, Selectable, Insertable, Associations)]
#[diesel(table_name = items)]
#[diesel(belongs_to(ItemCategory, foreign_key = category_id))]
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

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct NewItem {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub nature: ItemNature,
    pub state: ItemState,
    pub price: i32,
    pub category_id: String,
}

#[derive(Debug, Clone, AsChangeset, GraphQLInputObject)]
#[diesel(table_name = items)]
pub struct UpdateItem {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub nature: Option<ItemNature>,
    pub state: Option<ItemState>,
    pub price: Option<i32>,
    pub category_id: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Copy, AsExpression, FromStr, GraphQLEnum, ToSqlEnum, QueryableEnum)]
#[diesel(sql_type = Text)]
pub enum ItemNature {
    Goods,
    Service,
}

#[derive(Debug, Clone, Copy, AsExpression, FromStr, GraphQLEnum, ToSqlEnum, QueryableEnum)]
#[diesel(sql_type = Text)]
pub enum ItemState {
    Active,
    Inactive,
    Deleted,
}
