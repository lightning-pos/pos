use derive_more::derive::Display;
use juniper::graphql_scalar;
use serde::{Deserialize, Serialize};
use std::hash::Hash;
use uuid::Uuid;

use crate::{adapters::outgoing::database::{FromLibsqlValue, FromRow}, error::{Error, Result}};
use lightning_macros::{SeaQueryType, LibsqlType};

#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    SeaQueryType,
    LibsqlType
)]
#[graphql_scalar]
#[graphql(transparent)]
pub struct DbUuid(Uuid);

impl From<Uuid> for DbUuid {
    fn from(uuid: Uuid) -> Self {
        DbUuid(uuid)
    }
}

impl DbUuid {
    pub fn parse_str(s: &str) -> Result<Self> {
        Uuid::parse_str(s).map(DbUuid).map_err(Error::UuidError)
    }
}

impl FromRow<libsql::Row> for DbUuid {
    fn from_row(_row: &libsql::Row) -> Result<Self> {
        todo!()
    }
}
