use sea_query::{Expr, Query};
use juniper::FieldResult;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        models::common::channel_model::{Channel, Channels},
        types::db_uuid::DbUuid,
    },
    AppState,
};

pub async fn get_channel(id: DbUuid, context: &AppState) -> FieldResult<Channel> {
    let service = context.service.lock().await;

    let mut query_builder = Query::select();
    let query = query_builder
        .from(Channels::Table)
        .columns([
            Channels::Id,
            Channels::Name,
            Channels::Description,
            Channels::IsActive,
            Channels::CreatedAt,
            Channels::UpdatedAt,
        ])
        .and_where(Expr::col(Channels::Id).eq(id.to_string()));

    let channel = service.db_adapter.query_one::<Channel>(&query).await?;

    Ok(channel)
}

pub async fn get_channels(context: &AppState) -> FieldResult<Vec<Channel>> {
    let service = context.service.lock().await;

    let mut query_builder = Query::select();
    let query = query_builder
        .from(Channels::Table)
        .columns([
            Channels::Id,
            Channels::Name,
            Channels::Description,
            Channels::IsActive,
            Channels::CreatedAt,
            Channels::UpdatedAt,
        ]);

    let channels_list = service.db_adapter.query_many::<Channel>(&query).await?;

    Ok(channels_list)
}

pub async fn get_active_channels(context: &AppState) -> FieldResult<Vec<Channel>> {
    let service = context.service.lock().await;

    let mut query_builder = Query::select();
    let query = query_builder
        .from(Channels::Table)
        .columns([
            Channels::Id,
            Channels::Name,
            Channels::Description,
            Channels::IsActive,
            Channels::CreatedAt,
            Channels::UpdatedAt,
        ])
        .and_where(Expr::col(Channels::IsActive).eq(true));

    let channels_list = service.db_adapter.query_many::<Channel>(&query).await?;

    Ok(channels_list)
}
