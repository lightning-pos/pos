use chrono::NaiveDateTime;
use derive_more::derive::Display;
use juniper::GraphQLEnum;
use lightning_macros::{LibsqlEnum, LibsqlFromRow, SeaQueryEnum, SeaQueryModel};

use crate::{
    adapters::outgoing::database::{FromLibsqlValue, FromRow},
    core::types::db_uuid::DbUuid,
};

#[derive(Debug, SeaQueryModel, LibsqlFromRow)]
#[sea_query_model(new_input, update_input)]
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

pub mod queries {
    use chrono::{NaiveDateTime, Utc};
    use sea_query::{DeleteStatement, Expr, InsertStatement, SelectStatement, SimpleExpr, UpdateStatement, Value};
    use uuid::Uuid;
    use crate::core::types::db_uuid::DbUuid;

    use super::*;

    pub fn find_by_id(id: &DbUuid) -> SelectStatement {
        let columns = Users::all_columns();
        SelectStatement::new()
            .from(Users::Table)
            .columns(columns)
            .and_where(Expr::col(Users::Id).eq(id)).to_owned()
    }

    pub fn find_by_username(username: &str) -> SelectStatement {
        let columns = Users::all_columns();
        SelectStatement::new()
            .from(Users::Table)
            .columns(columns)
            .and_where(Expr::col(Users::Username).eq(username)).to_owned()
    }

    pub fn insert(user: &UserNewInput) -> InsertStatement {
        let user_id: DbUuid = Uuid::now_v7().into();
        let pin_hash = user.pin_hash.clone();
        let full_name = user.full_name.clone();
        let state = user.state.clone();
        let last_login_at: Option<NaiveDateTime> = user.last_login_at;
        let now = Utc::now().naive_utc();

        let mut insert_stmt = InsertStatement::new();
        let _ = insert_stmt
            .into_table(Users::Table)
            .columns(Users::all_columns())
            .values([
                user_id.into(),
                user.username.clone().into(),
                pin_hash.into(),
                full_name.into(),
                state.into(),
                last_login_at.into(),
                now.into(),
                now.into(),
            ]);

        insert_stmt.to_owned()
    }

    pub fn update(user: &UserUpdateInput) -> UpdateStatement {
        let now = Utc::now().naive_utc();

        let mut update_stmt = UpdateStatement::new();
        update_stmt.table(Users::Table);

        if let Some(username) = user.username.clone() {
            update_stmt.value::<Users, String>(Users::Username, username);
        }
        if let Some(pin_hash) = user.pin_hash.clone() {
            update_stmt.value::<Users, String>(Users::PinHash, pin_hash);
        }
        if let Some(full_name) = user.full_name.clone() {
            update_stmt.value::<Users, String>(Users::FullName, full_name);
        }
        if let Some(state) = user.state.clone() {
            update_stmt.value::<Users, UserState>(Users::State, state);
        }
        if let Some(last_login_at) = user.last_login_at {
            match last_login_at {
                Some(last_login_at) => update_stmt.value::<Users, NaiveDateTime>(Users::LastLoginAt, last_login_at),
                None => update_stmt.value::<Users, SimpleExpr>(Users::LastLoginAt, Expr::value(Value::String(None))),
            };
        }
        update_stmt.value::<Users, NaiveDateTime>(Users::UpdatedAt, now);

        update_stmt.and_where(Expr::col(Users::Id).eq(user.id));

        update_stmt.to_owned()
    }

    pub fn delete_by_id(id: &DbUuid) -> DeleteStatement {
        DeleteStatement::new()
            .from_table(Users::Table)
            .and_where(Expr::col(Users::Id).eq(id))
            .to_owned()
    }
}

