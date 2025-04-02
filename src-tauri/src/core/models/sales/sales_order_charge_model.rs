use chrono::NaiveDateTime;
use diesel::prelude::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use juniper::GraphQLInputObject;
use uuid::Uuid;

use crate::{
    core::types::{db_uuid::DbUuid, money::Money},
    schema::sales_order_charges,
    AppState,
};
use juniper::graphql_object;

#[derive(Debug, Queryable, Selectable, Insertable, Identifiable, Clone)]
#[diesel(table_name = sales_order_charges)]
#[diesel(primary_key(id))]
pub struct SalesOrderCharge {
    pub id: DbUuid,
    pub order_id: DbUuid,
    pub charge_type_id: DbUuid,
    pub charge_type_name: String,
    pub amount: Money,
    pub tax_amount: Money,
    pub tax_group_id: Option<DbUuid>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// GraphQL implementation for SalesOrderCharge
#[graphql_object(context = AppState)]
impl SalesOrderCharge {
    // Expose fields needed in GraphQL
    fn id(&self) -> DbUuid {
        self.id
    }
    fn order_id(&self) -> DbUuid {
        self.order_id
    }
    fn charge_type_id(&self) -> DbUuid {
        self.charge_type_id
    }
    fn charge_type_name(&self) -> &str {
        &self.charge_type_name
    }
    fn amount(&self) -> Money {
        self.amount
    }
    fn tax_amount(&self) -> Money {
        self.tax_amount
    }
    fn tax_group_id(&self) -> Option<DbUuid> {
        self.tax_group_id
    }
    fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }
    fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct SalesOrderChargeNewInput {
    pub charge_type_id: DbUuid,
    pub charge_type_name: String,
    pub amount: Money,
    pub tax_amount: Money,
    pub tax_group_id: Option<DbUuid>,
    // order_id is set automatically when creating the order
}

// No UpdateInput or Changeset needed as charges are typically created with the order
// and not updated individually. If needed, they can be added later.
