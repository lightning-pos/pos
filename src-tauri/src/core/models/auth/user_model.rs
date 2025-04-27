use chrono::NaiveDateTime;
use derive_more::derive::Display;
use juniper::{GraphQLEnum, GraphQLInputObject};
use lightning_macros::{SeaQueryCrud, SeaQueryEnum, SeaQueryModel};

use crate::{
    adapters::outgoing::database::{FromLibsqlValue, FromRow},
    core::{db::SeaQueryCrudTrait, types::db_uuid::DbUuid},
    error::{Error, Result}
};

#[derive(Debug, SeaQueryModel, SeaQueryCrud)]
pub struct User {
    pub id: DbUuid,
    pub username: String,
    pub pin_hash: String,
    pub full_name: String,
    pub state: UserState,
    pub last_login_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl FromRow<libsql::Row> for User {
    fn from_row(row: &libsql::Row) -> Result<Self> {

        let id = DbUuid::from_libsql_value(row.get_value(0)?)?;

        let username = String::from_libsql_value(row.get_value(1)?)?;

        let pin_hash = String::from_libsql_value(row.get_value(2)?)?;

        let full_name = String::from_libsql_value(row.get_value(3)?)?;

        let state = UserState::from_libsql_value(row.get_value(4)?)?;

        let last_login_at = match row.get_value(5) {
            Ok(libsql::Value::Null) => None,
            Ok(timestamp_str) => Some(NaiveDateTime::from_libsql_value(timestamp_str)?),
            Err(_) => None,
        };

        let created_at = NaiveDateTime::from_libsql_value(row.get_value(6)?)?;

        let updated_at = NaiveDateTime::from_libsql_value(row.get_value(7)?)?;

        Ok(User {
            id,
            username,
            pin_hash,
            full_name,
            state,
            last_login_at,
            created_at,
            updated_at,
        })
    }
}

#[derive(Debug, Clone, Display, PartialEq, GraphQLEnum, SeaQueryEnum)]
pub enum UserState {
    Active,
    Inactive,
    Locked,
}

impl FromLibsqlValue for UserState {
    fn from_libsql_value(value: libsql::Value) -> Result<Self> {
        match value {
            libsql::Value::Text(s) => match s.as_str() {
                "Active" => Ok(UserState::Active),
                "Inactive" => Ok(UserState::Inactive),
                "Locked" => Ok(UserState::Locked),
                _ => Err(Error::DatabaseError("Invalid user state value in database".to_string())),
            },
            _ => Err(Error::DatabaseError("Invalid user state value type in database".to_string())),
        }
    }
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct UserNewInput {
    pub username: String,
    pub pin: String,
    pub full_name: String,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct UserUpdateInput {
    pub id: DbUuid,
    pub username: Option<String>,
    pub pin: Option<String>,
    pub full_name: Option<String>,
    pub state: Option<UserState>,
}
