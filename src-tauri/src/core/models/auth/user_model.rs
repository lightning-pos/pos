use chrono::NaiveDateTime;
use derive_more::derive::Display;
use juniper::GraphQLEnum;
use lightning_macros::{LibsqlEnum, LibsqlFromRow, SeaQueryEnum, SeaQueryModel};

use crate::{
    adapters::outgoing::database::{FromLibsqlValue, FromRow},
    core::types::db_uuid::DbUuid,
};

#[derive(Debug, SeaQueryModel, LibsqlFromRow)]
#[sea_query_model(new_input, update_input, queries)]
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

#[derive(Debug, Clone, Display, PartialEq, GraphQLEnum, SeaQueryEnum, LibsqlEnum)]
pub enum UserState {
    Active,
    Inactive,
    Locked,
}
