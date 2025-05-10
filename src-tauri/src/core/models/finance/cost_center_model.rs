use chrono::NaiveDateTime;
use derive_more::Display;
use juniper::{GraphQLEnum, GraphQLInputObject};
use lightning_macros::{LibsqlEnum, LibsqlFromRow, SeaQueryCrud, SeaQueryEnum, SeaQueryModel};

use crate::{adapters::outgoing::database::{FromLibsqlValue, FromRow}, core::{db::SeaQueryCrudTrait, types::db_uuid::DbUuid}};

#[derive(Debug, Clone, SeaQueryModel, SeaQueryCrud, LibsqlFromRow)]
pub struct CostCenter {
    pub id: DbUuid,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub state: CostCenterState,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct CostCenterNewInput {
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub state: Option<CostCenterState>,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct CostCenterUpdateInput {
    pub id: DbUuid,
    pub name: Option<String>,
    pub code: Option<String>,
    pub description: Option<Option<String>>,
    pub state: Option<CostCenterState>,
}

#[derive(Debug, Clone, Copy, GraphQLEnum, PartialEq, Eq, Display, SeaQueryEnum, LibsqlEnum)]
pub enum CostCenterState {
    Active,
    Inactive,
}
