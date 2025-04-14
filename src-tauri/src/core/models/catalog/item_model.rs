use chrono::NaiveDateTime;
use derive_more::Display;
use diesel::{
    prelude::{AsChangeset, Associations, Insertable, Queryable},
    Selectable,
};
use diesel_derive_enum::DbEnum;
use juniper::{GraphQLEnum, GraphQLInputObject};
use sea_query::Iden;

use crate::core::{
    models::catalog::item_group_model::ItemGroup,
    types::{db_uuid::DbUuid, money::Money},
};
use crate::schema::items;

#[derive(Debug, Queryable, Selectable, Insertable, Associations)]
#[diesel(table_name = items)]
#[diesel(belongs_to(ItemGroup, foreign_key = category_id))]
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

#[derive(Debug, Clone, AsChangeset, GraphQLInputObject)]
#[diesel(table_name = items)]
pub struct UpdateItem {
    pub id: DbUuid,
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub nature: Option<ItemNature>,
    pub state: Option<ItemState>,
    pub price: Option<Money>,
    pub category_id: Option<DbUuid>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Copy, DbEnum, GraphQLEnum, Display)]
pub enum ItemNature {
    Goods,
    Service,
}

#[derive(Debug, Clone, Copy, DbEnum, GraphQLEnum, Display)]
pub enum ItemState {
    Active,
    Inactive,
    Deleted,
}

// Define table and column identifiers for SeaQuery
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