use chrono::NaiveDateTime;
use derive_more::Display;
use juniper::{GraphQLEnum, GraphQLInputObject};
use lightning_macros::{LibsqlEnum, LibsqlFromRow, SeaQueryCrud, SeaQueryEnum, SeaQueryModel};

use crate::{adapters::outgoing::database::FromLibsqlValue, core::{db::SeaQueryCrudTrait, types::db_uuid::DbUuid}};

#[derive(Debug, Clone, SeaQueryModel, SeaQueryCrud, LibsqlFromRow)]
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

#[derive(Debug, Clone, Copy, GraphQLEnum, PartialEq, Eq, Display, SeaQueryEnum, LibsqlEnum)]
pub enum PaymentMethodState {
    Active,
    Inactive,
}
