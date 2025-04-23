use chrono::NaiveDateTime;
use derive_more::derive::Display;
use juniper::{GraphQLEnum, GraphQLInputObject};
use sea_query::Iden;

use crate::{
    core::types::db_uuid::DbUuid,
    adapters::outgoing::database::FromRow,
    error::{Error, Result},
};

#[derive(Debug)]
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
        // Get the column values by index
        let id_str = row.get::<String>(0)
            .map_err(|e| Error::DatabaseError(format!("Failed to get id: {}", e)))?;
        let id = DbUuid::parse_str(&id_str)
            .map_err(|e| Error::DatabaseError(format!("Failed to parse id as UUID: {}", e)))?;

        let username = row.get::<String>(1)
            .map_err(|e| Error::DatabaseError(format!("Failed to get username: {}", e)))?;

        let pin_hash = row.get::<String>(2)
            .map_err(|e| Error::DatabaseError(format!("Failed to get pin_hash: {}", e)))?;

        let full_name = row.get::<String>(3)
            .map_err(|e| Error::DatabaseError(format!("Failed to get full_name: {}", e)))?;

        let state_str = row.get::<String>(4)
            .map_err(|e| Error::DatabaseError(format!("Failed to get state: {}", e)))?;
        let state = match state_str.as_str() {
            "Active" => UserState::Active,
            "Inactive" => UserState::Inactive,
            "Locked" => UserState::Locked,
            _ => return Err(Error::DatabaseError(format!("Invalid user state: {}", state_str))),
        };

        // last_login_at is optional (column 5)
        let last_login_at = match row.get::<String>(5) {
            Ok(timestamp_str) => {
                Some(NaiveDateTime::parse_from_str(&timestamp_str, "%Y-%m-%d %H:%M:%S")
                    .map_err(|e| Error::DatabaseError(format!("Failed to parse last_login_at: {}", e)))?
                )
            },
            Err(_) => None,
        };

        let created_at_str = row.get::<String>(6)
            .map_err(|e| Error::DatabaseError(format!("Failed to get created_at: {}", e)))?;
        let created_at = NaiveDateTime::parse_from_str(&created_at_str, "%Y-%m-%d %H:%M:%S")
            .map_err(|e| Error::DatabaseError(format!("Failed to parse created_at: {}", e)))?;

        let updated_at_str = row.get::<String>(7)
            .map_err(|e| Error::DatabaseError(format!("Failed to get updated_at: {}", e)))?;
        let updated_at = NaiveDateTime::parse_from_str(&updated_at_str, "%Y-%m-%d %H:%M:%S")
            .map_err(|e| Error::DatabaseError(format!("Failed to parse updated_at: {}", e)))?;

        Ok(User {
            id,
            username: username.to_string(),
            pin_hash: pin_hash.to_string(),
            full_name: full_name.to_string(),
            state,
            last_login_at,
            created_at,
            updated_at,
        })
    }
}

// Define table and column identifiers for SeaQuery
#[derive(Iden)]
pub enum Users {
    Table,
    Id,
    Username,
    PinHash,
    FullName,
    State,
    LastLoginAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(Debug, Display, Clone, PartialEq, GraphQLEnum)]
pub enum UserState {
    Active,
    Inactive,
    Locked,
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
