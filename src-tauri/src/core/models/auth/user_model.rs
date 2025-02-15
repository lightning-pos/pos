use chrono::NaiveDateTime;
use diesel::{prelude::*, Selectable};
use diesel_derive_enum::DbEnum;
use juniper::{GraphQLEnum, GraphQLInputObject};

use crate::{core::types::db_uuid::DbUuid, schema::users};

#[derive(Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = users)]
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

#[derive(Debug, Clone, PartialEq, DbEnum, GraphQLEnum)]
pub enum UserState {
    Active,
    Inactive,
    Locked,
}

#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = users)]
pub struct UserUpdateChangeset {
    pub id: DbUuid,
    pub username: Option<String>,
    pub pin_hash: Option<String>,
    pub full_name: Option<String>,
    pub state: Option<UserState>,
    pub updated_at: NaiveDateTime,
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
