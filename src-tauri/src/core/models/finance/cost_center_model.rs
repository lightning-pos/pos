use crate::schema::cost_centers;
use chrono::NaiveDateTime;
use derive_more::Display;
use diesel::prelude::{AsChangeset, Insertable, Queryable, Selectable};
use diesel_derive_enum::DbEnum;
use juniper::{GraphQLEnum, GraphQLInputObject};
use sea_query::Iden;

use crate::core::types::db_uuid::DbUuid;

#[derive(Debug, Queryable, Insertable, Selectable)]
#[diesel(table_name = cost_centers)]
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

#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = cost_centers)]
pub struct CostCenterUpdateChangeset {
    pub id: DbUuid,
    pub name: Option<String>,
    pub code: Option<String>,
    pub description: Option<Option<String>>,
    pub state: Option<CostCenterState>,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Copy, DbEnum, GraphQLEnum, PartialEq, Eq, Display)]
pub enum CostCenterState {
    Active,
    Inactive,
}

// Define table and column identifiers for SeaQuery
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