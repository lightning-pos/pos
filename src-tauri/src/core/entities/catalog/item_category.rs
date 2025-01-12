use derive_more::derive::Display;
use diesel::{
    expression::AsExpression,
    prelude::*,
    serialize::{IsNull, Output, ToSql},
    sql_types::Text,
};

#[derive(Debug, Clone, Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::item_categories)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ItemCategory {
    pub id: String,
    pub name: String,
    pub state: ItemCategoryState,
    pub description: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Display, AsExpression, PartialEq)]
#[diesel(sql_type = diesel::sql_types::Text)]
pub enum ItemCategoryState {
    Active,
    Inactive,
}

impl From<String> for ItemCategoryState {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Active" => ItemCategoryState::Active,
            "Inactive" => ItemCategoryState::Inactive,
            _ => ItemCategoryState::Inactive, // default case
        }
    }
}

impl Queryable<Text, diesel::sqlite::Sqlite> for ItemCategoryState {
    type Row = String;

    fn build(row: Self::Row) -> diesel::deserialize::Result<Self> {
        Ok(ItemCategoryState::from(row))
    }
}

impl ToSql<Text, diesel::sqlite::Sqlite> for ItemCategoryState {
    fn to_sql<'b>(
        &'b self,
        out: &mut Output<'b, '_, diesel::sqlite::Sqlite>,
    ) -> diesel::serialize::Result {
        let s = match self {
            ItemCategoryState::Active => "Active",
            ItemCategoryState::Inactive => "Inactive",
        };
        out.set_value(s);
        Ok(IsNull::No)
    }
}
