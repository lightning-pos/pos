use derive_more::derive::Display;
use juniper::graphql_scalar;
use serde::{Deserialize, Serialize};
use std::hash::Hash;
use uuid::Uuid;

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
