use derive_more::derive::Display;
use juniper::graphql_scalar;
use std::hash::Hash;
use uuid::Uuid;

use crate::{adapters::outgoing::database::{FromLibsqlValue, FromRow}, error::{Error, Result}};
use lightning_macros::{LibsqlType, SeaQueryType};

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
    SeaQueryType,
    LibsqlType
)]
#[graphql_scalar]
#[graphql(transparent)]
pub struct DbUuid(Uuid);

impl DbUuid {
    pub fn from_str(s: &str) -> Result<Self> {
        Uuid::parse_str(s).map(DbUuid).map_err(Error::UuidError)
    }
}

impl From<Uuid> for DbUuid {
    fn from(uuid: Uuid) -> Self {
        DbUuid(uuid)
    }
}

impl FromRow<libsql::Row> for DbUuid {
    fn from_row(row: &libsql::Row) -> Result<Self> {
        match row.get(0) {
            Ok(libsql::Value::Text(s)) => DbUuid::from_str(&s),
            _ => Err(Error::DatabaseError("Invalid UUID value type in database".to_string())),
        }
    }
}


