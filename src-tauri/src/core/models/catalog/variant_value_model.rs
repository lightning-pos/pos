use chrono::NaiveDateTime;
use diesel::{
    prelude::{AsChangeset, Insertable, Queryable},
    Associations, Selectable,
};
use juniper::GraphQLInputObject;

use crate::core::models::catalog::variant_type_model::VariantType;
use crate::core::types::db_uuid::DbUuid;
use crate::schema::variant_values;

#[derive(Debug, Clone, Queryable, Selectable, Insertable, Associations)]
#[diesel(table_name = variant_values)]
#[diesel(belongs_to(VariantType, foreign_key = variant_type_id))]
pub struct VariantValue {
    pub id: DbUuid,
    pub variant_type_id: DbUuid,
    pub value: String,
    pub display_order: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct VariantValueNewInput {
    pub variant_type_id: DbUuid,
    pub value: String,
    pub display_order: Option<i32>,
}

#[derive(Debug, Clone, AsChangeset, GraphQLInputObject)]
#[diesel(table_name = variant_values)]
pub struct VariantValueUpdateInput {
    pub id: DbUuid,
    pub value: Option<String>,
    pub display_order: Option<i32>,
    pub updated_at: Option<NaiveDateTime>,
}
