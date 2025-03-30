use crate::schema::payment_methods;
use chrono::NaiveDateTime;
use diesel::prelude::{AsChangeset, Insertable, Queryable, Selectable};
use diesel_derive_enum::DbEnum;
use juniper::{GraphQLEnum, GraphQLInputObject};

use crate::core::types::db_uuid::DbUuid;

#[derive(Debug, Queryable, Insertable, Selectable)]
#[diesel(table_name = payment_methods)]
pub struct PaymentMethod {
    pub id: DbUuid,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub state: PaymentMethodState,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct PaymentMethodNewInput {
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub state: Option<PaymentMethodState>,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct PaymentMethodUpdateInput {
    pub id: DbUuid,
    pub name: Option<String>,
    pub code: Option<String>,
    pub description: Option<Option<String>>,
    pub state: Option<PaymentMethodState>,
}

#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = payment_methods)]
pub struct PaymentMethodUpdateChangeset {
    pub id: DbUuid,
    pub name: Option<String>,
    pub code: Option<String>,
    pub description: Option<Option<String>>,
    pub state: Option<PaymentMethodState>,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Copy, DbEnum, GraphQLEnum, PartialEq, Eq)]
pub enum PaymentMethodState {
    Active,
    Inactive,
}
