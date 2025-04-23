use chrono::NaiveDateTime;
use derive_more::Display;
use juniper::{GraphQLEnum, GraphQLInputObject};
use sea_query::Iden;

use crate::{adapters::outgoing::database::FromRow, core::types::{db_uuid::DbUuid, money::Money}, error::Result};

#[derive(Debug)]
pub struct Item {
    pub id: DbUuid,
    pub name: String,
    pub description: Option<String>,
    pub nature: ItemNature,
    pub state: ItemState,
    pub price: Money,
    pub category_id: DbUuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct NewItem {
    pub name: String,
    pub description: Option<String>,
    pub nature: ItemNature,
    pub state: ItemState,
    pub price: Money,
    pub category_id: DbUuid,
    pub tax_ids: Option<Vec<DbUuid>>, // Optional list of tax IDs to assign to this item
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct UpdateItem {
    pub id: DbUuid,
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub nature: Option<ItemNature>,
    pub state: Option<ItemState>,
    pub price: Option<Money>,
    pub category_id: Option<DbUuid>,
}

#[derive(Debug, Clone, Copy, GraphQLEnum, Display)]
pub enum ItemNature {
    Goods,
    Service,
}

#[derive(Debug, Clone, Copy, GraphQLEnum, Display)]
pub enum ItemState {
    Active,
    Inactive,
    Deleted,
}

#[derive(Iden)]
pub enum Items {
    Table,
    Id,
    Name,
    Description,
    Nature,
    State,
    Price,
    CategoryId,
    CreatedAt,
    UpdatedAt,
}

impl FromRow<libsql::Row> for Item {
    fn from_row(row: &libsql::Row) -> Result<Self> {
        todo!()
    }
}
