use crate::schema::item_categories;
use chrono::NaiveDateTime;
use diesel::prelude::{AsChangeset, Insertable, Queryable, Selectable};
use diesel_derive_enum::DbEnum;
use juniper::{GraphQLEnum, GraphQLInputObject};

#[derive(Debug, Queryable, Insertable, Selectable)]
#[diesel(table_name = item_categories)]
pub struct ItemCategory {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub state: ItemCategoryState,
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

#[derive(Debug, Clone, Copy, DbEnum, GraphQLEnum)]
pub enum ItemCategoryState {
    Active,
    Inactive,
    Deleted,
}
