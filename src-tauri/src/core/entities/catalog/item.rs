use chrono::NaiveDateTime;
use diesel::{
    prelude::{AsChangeset, Associations, Insertable, Queryable},
    Selectable,
};
use diesel_derive_enum::DbEnum;
use juniper::{GraphQLEnum, GraphQLInputObject};

use crate::core::{entities::catalog::item_category::ItemCategory, types::db_uuid::DbUuid};
use crate::schema::items;

#[derive(Debug, Queryable, Selectable, Insertable, Associations)]
#[diesel(table_name = items)]
#[diesel(belongs_to(ItemCategory, foreign_key = category_id))]
pub struct Item {
    pub id: DbUuid,
    pub name: String,
    pub description: Option<String>,
    pub nature: ItemNature,
    pub state: ItemState,
    pub price: i32,
    pub category_id: DbUuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct NewItem {
    pub id: DbUuid,
    pub name: String,
    pub description: Option<String>,
    pub nature: ItemNature,
    pub state: ItemState,
    pub price: i32,
    pub category_id: DbUuid,
}

#[derive(Debug, Clone, AsChangeset, GraphQLInputObject)]
#[diesel(table_name = items)]
pub struct UpdateItem {
    pub id: DbUuid,
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub nature: Option<ItemNature>,
    pub state: Option<ItemState>,
    pub price: Option<i32>,
    pub category_id: Option<DbUuid>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Copy, DbEnum, GraphQLEnum)]
pub enum ItemNature {
    Goods,
    Service,
}

#[derive(Debug, Clone, Copy, DbEnum, GraphQLEnum)]
pub enum ItemState {
    Active,
    Inactive,
    Deleted,
}
