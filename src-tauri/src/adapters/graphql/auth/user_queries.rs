use sea_query::{Expr, Query};
use juniper::FieldResult;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        models::auth::user_model::{User, Users},
        types::db_uuid::DbUuid,
    },
    AppState,
};

pub async fn users(
    first: Option<i32>,
    offset: Option<i32>,
    context: &AppState,
) -> FieldResult<Vec<User>> {
    // Build the query with SeaQuery
    let mut query = Query::select();
    let stmt = query
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
        stmt.limit(limit as u64);
    }
    if let Some(off) = offset {
        stmt.offset(off as u64);
    }

    // Execute the query
    let service = context.service.lock().await;
    let result = service.db_adapter.query_many::<User>(&stmt).await?;

    Ok(result)
}

pub async fn user(id: DbUuid, context: &AppState) -> FieldResult<User> {
    // Build the query with SeaQuery
    let mut query = Query::select();
    let stmt = query
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
        .and_where(Expr::col(Users::Id).eq(id.to_string()));

    // Execute the query
    let service = context.service.lock().await;
    let result = service.db_adapter.query_one::<User>(&stmt).await?;

    Ok(result)
}
