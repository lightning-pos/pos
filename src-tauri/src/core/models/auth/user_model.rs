use chrono::NaiveDateTime;
use derive_more::derive::Display;
use juniper::{GraphQLEnum, GraphQLInputObject};
use lightning_macros::{LibsqlEnum, LibsqlFromRow, SeaQueryCrud, SeaQueryEnum, SeaQueryModel};

use crate::{
    adapters::outgoing::database::FromLibsqlValue,
    core::{db::SeaQueryCrudTrait, types::db_uuid::DbUuid},
};

#[derive(Debug, SeaQueryModel, LibsqlFromRow, SeaQueryCrud)]
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
