use sea_query::{Expr, Query, SqliteQueryBuilder};
use juniper::FieldResult;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        models::catalog::item_model::{Item, Items},
        types::db_uuid::DbUuid,
    },
    AppState,
};

pub fn items(
    first: Option<i32>,
    offset: Option<i32>,
    context: &AppState,
) -> FieldResult<Vec<Item>> {
    let service = context.service.lock().unwrap();

    // Build the query with SeaQuery
    let mut query_builder = Query::select();
    let query = query_builder
        .from(Items::Table)
        .columns([
            Items::Id,
            Items::Name,
            Items::Description,
            Items::Nature,
            Items::State,
            Items::Price,
            Items::CategoryId,
            Items::CreatedAt,
            Items::UpdatedAt,
        ]);

    if let Some(limit) = first {
        query.limit(limit as u64);
    }

    if let Some(off) = offset {
        query.offset(off as u64);
    }

    let sql = query.to_string(SqliteQueryBuilder);

    // Execute the query
    let result = service.db_adapter.query_many::<Item>(&sql, vec![])?;
    
    Ok(result)
}

pub fn item(id: DbUuid, context: &AppState) -> FieldResult<Item> {
    let service = context.service.lock().unwrap();
    
    // Build the query with SeaQuery
    let query = Query::select()
        .from(Items::Table)
        .columns([
            Items::Id,
            Items::Name,
            Items::Description,
            Items::Nature,
            Items::State,
            Items::Price,
            Items::CategoryId,
            Items::CreatedAt,
            Items::UpdatedAt,
        ])
        .and_where(Expr::col(Items::Id).eq(id.to_string()))
        .to_string(SqliteQueryBuilder);
    
    // Execute the query
    let result = service.db_adapter.query_one::<Item>(&query, vec![])?;
    
    Ok(result)
}