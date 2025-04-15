use sea_query::{Expr, Query, SqliteQueryBuilder};
use juniper::FieldResult;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        models::auth::user_model::{User, Users},
        types::db_uuid::DbUuid,
    },
    AppState,
};

pub fn users(
    first: Option<i32>,
    offset: Option<i32>,
    context: &AppState,
) -> FieldResult<Vec<User>> {
    let service = context.service.lock().unwrap();
    
    // Build the query with SeaQuery
    let mut query_builder = Query::select();
    let query = query_builder
        .from(Users::Table)
        .columns([
            Users::Id,
            Users::Username,
            Users::PinHash,
            Users::FullName,
            Users::State,
            Users::LastLoginAt,
            Users::CreatedAt,
            Users::UpdatedAt,
        ]);
    
    // Apply pagination if parameters are provided
    if let Some(limit) = first {
        query.limit(limit as u64);
    }
    if let Some(off) = offset {
        query.offset(off as u64);
    }
    
    let sql = query.to_string(SqliteQueryBuilder);
    
    // Execute the query
    let result = service.db_adapter.query_many::<User>(&sql, vec![])?;
    
    Ok(result)
}

pub fn user(id: DbUuid, context: &AppState) -> FieldResult<User> {
    let service = context.service.lock().unwrap();
    
    // Build the query with SeaQuery
    let query = Query::select()
        .from(Users::Table)
        .columns([
            Users::Id,
            Users::Username,
            Users::PinHash,
            Users::FullName,
            Users::State,
            Users::LastLoginAt,
            Users::CreatedAt,
            Users::UpdatedAt,
        ])
        .and_where(Expr::col(Users::Id).eq(id.to_string()))
        .to_string(SqliteQueryBuilder);
    
    // Execute the query
    let result = service.db_adapter.query_one::<User>(&query, vec![])?;
    
    Ok(result)
}