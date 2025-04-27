use chrono::NaiveDateTime;
use derive_more::Display;
use juniper::{GraphQLEnum, GraphQLInputObject};
use lightning_macros::{LibsqlFromRow, SeaQueryCrud, SeaQueryEnum, SeaQueryModel, LibsqlEnum};

use crate::{adapters::outgoing::database::FromLibsqlValue, core::{db::SeaQueryCrudTrait, types::{db_uuid::DbUuid, money::Money}}};

#[derive(Debug, SeaQueryModel, SeaQueryCrud, LibsqlFromRow)]
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

#[derive(Debug, Clone, Copy, GraphQLEnum, Display, SeaQueryEnum, LibsqlEnum)]
pub enum ItemNature {
    Goods,
    Service,
}

#[derive(Debug, Clone, Copy, GraphQLEnum, Display, SeaQueryEnum, LibsqlEnum)]
pub enum ItemState {
    Active,
    Inactive,
    Deleted,
}
