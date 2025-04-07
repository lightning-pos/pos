use chrono::NaiveDateTime;
use diesel::prelude::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use juniper::GraphQLInputObject;

use crate::{core::types::db_uuid::DbUuid, schema::sales_charge_types};

#[derive(Debug, Queryable, Selectable, Insertable, Identifiable)]
#[diesel(table_name = sales_charge_types)]
#[diesel(primary_key(id))]
pub struct SalesChargeType {
    pub id: DbUuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct SalesChargeTypeNewInput {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, GraphQLInputObject, Identifiable)]
#[diesel(table_name = sales_charge_types)]
#[diesel(primary_key(id))]
pub struct SalesChargeTypeUpdateInput {
    pub id: DbUuid,
    pub name: Option<String>,
    pub description: Option<Option<String>>, // Double optional for nullable field
}

#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = sales_charge_types)]
pub struct SalesChargeTypeUpdateChangeset {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub updated_at: NaiveDateTime,
}

// Helper to create changeset from input
impl SalesChargeTypeUpdateInput {
    pub fn into_changeset(self, now: NaiveDateTime) -> SalesChargeTypeUpdateChangeset {
        SalesChargeTypeUpdateChangeset {
            name: self.name,
            description: self.description,
            updated_at: now,
        }
    }
}
