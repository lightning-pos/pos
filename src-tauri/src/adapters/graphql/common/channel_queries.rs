use sea_query::{Expr, Query, SqliteQueryBuilder};
use juniper::FieldResult;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        models::common::channel_model::{Channel, Channels},
        types::db_uuid::DbUuid,
    },
    AppState,
};

pub fn get_channel(id: DbUuid, context: &AppState) -> FieldResult<Channel> {
    let service = context.service.lock().unwrap();
    
    let query = Query::select()
        .from(Channels::Table)
        .columns([
            Channels::Id,
            Channels::Name,
            Channels::Description,
            Channels::IsActive,
            Channels::CreatedAt,
            Channels::UpdatedAt,
        ])
        .and_where(Expr::col(Channels::Id).eq(id.to_string()))
        .to_string(SqliteQueryBuilder);
        
    let channel = service.db_adapter.query_one::<Channel>(&query, vec![])?;

    Ok(channel)
}

pub fn get_channels(context: &AppState) -> FieldResult<Vec<Channel>> {
    let service = context.service.lock().unwrap();
    
    let query = Query::select()
        .from(Channels::Table)
        .columns([
            Channels::Id,
            Channels::Name,
            Channels::Description,
            Channels::IsActive,
            Channels::CreatedAt,
            Channels::UpdatedAt,
        ])
        .to_string(SqliteQueryBuilder);
        
    let channels_list = service.db_adapter.query_many::<Channel>(&query, vec![])?;

    Ok(channels_list)
}

pub fn get_active_channels(context: &AppState) -> FieldResult<Vec<Channel>> {
    let service = context.service.lock().unwrap();
    
    let query = Query::select()
        .from(Channels::Table)
        .columns([
            Channels::Id,
            Channels::Name,
            Channels::Description,
            Channels::IsActive,
            Channels::CreatedAt,
            Channels::UpdatedAt,
        ])
        .and_where(Expr::col(Channels::IsActive).eq(true))
        .to_string(SqliteQueryBuilder);
        
    let channels_list = service.db_adapter.query_many::<Channel>(&query, vec![])?;

    Ok(channels_list)
}