use crate::{core::types::db_uuid::DbUuid, schema::item_categories};
use chrono::NaiveDateTime;
use diesel::prelude::{AsChangeset, Insertable, Queryable, Selectable};
use diesel_derive_enum::DbEnum;
use juniper::{GraphQLEnum, GraphQLInputObject};

#[derive(Debug, Queryable, Insertable, Selectable)]
#[diesel(table_name = item_categories)]
pub struct ItemGroup {
    pub id: DbUuid,
    pub name: String,
    pub description: Option<String>,
    pub state: ItemGroupState,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct ItemGroupNew {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, AsChangeset, GraphQLInputObject)]
#[diesel(table_name = item_categories)]
pub struct ItemGroupUpdate {
    pub id: DbUuid,
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub state: Option<ItemGroupState>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Copy, DbEnum, GraphQLEnum)]
pub enum ItemGroupState {
    Active,
    Inactive,
    Deleted,
}