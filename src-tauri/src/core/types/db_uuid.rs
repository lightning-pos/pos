use derive_more::derive::Display;
use juniper::graphql_scalar;
use serde::{Deserialize, Serialize};
use std::hash::Hash;
use uuid::Uuid;

use crate::{adapters::outgoing::database::FromRow, error::{Error, Result}};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]

#[graphql_scalar]
#[graphql(transparent)]
#[derive(Hash, Display)]
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
    fn from_row(row: &libsql::Row) -> Result<Self> {
        todo!()
    }
}
