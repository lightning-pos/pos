use chrono::NaiveDateTime;
use derive_more::Display;
use juniper::{GraphQLEnum, GraphQLInputObject};
use sea_query::Iden;

use crate::{adapters::outgoing::database::FromRow, core::types::db_uuid::DbUuid, error::Result};

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, Copy, GraphQLEnum, PartialEq, Eq, Display)]
pub enum CostCenterState {
    Active,
    Inactive,
}

#[derive(Iden)]
pub enum CostCenters {
    Table,
    Id,
    Name,
    Code,
    Description,
    State,
    CreatedAt,
    UpdatedAt,
}

impl FromRow<libsql::Row> for CostCenter {
    fn from_row(row: &libsql::Row) -> Result<Self> {
        todo!()
    }
}