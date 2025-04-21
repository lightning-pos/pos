use sea_query::{Expr, Query};
use juniper::FieldResult;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        models::common::brand_model::{Brand, Brands},
        types::db_uuid::DbUuid,
    },
    AppState,
};

pub async fn get_brand(id: DbUuid, context: &AppState) -> FieldResult<Brand> {
    let service = context.service.lock().await;

    let mut query_builder = Query::select();
    let query = query_builder
        .from(Brands::Table)
        .columns([
            Brands::Id,
            Brands::Name,
            Brands::Description,
            Brands::IsActive,
            Brands::CreatedAt,
            Brands::UpdatedAt,
        ])
        .and_where(Expr::col(Brands::Id).eq(id.to_string()));

    let brand = service.db_adapter.query_one::<Brand>(&query).await?;

    Ok(brand)
}

pub async fn get_brands(context: &AppState) -> FieldResult<Vec<Brand>> {
    let service = context.service.lock().await;

    let mut query_builder = Query::select();
    let query = query_builder
        .from(Brands::Table)
        .columns([
            Brands::Id,
            Brands::Name,
            Brands::Description,
            Brands::IsActive,
            Brands::CreatedAt,
            Brands::UpdatedAt,
        ]);

    let brands_list = service.db_adapter.query_many::<Brand>(&query).await?;

    Ok(brands_list)
}

pub async fn get_active_brands(context: &AppState) -> FieldResult<Vec<Brand>> {
    let service = context.service.lock().await;

    let mut query_builder = Query::select();
    let query = query_builder
        .from(Brands::Table)
        .columns([
            Brands::Id,
            Brands::Name,
            Brands::Description,
            Brands::IsActive,
            Brands::CreatedAt,
            Brands::UpdatedAt,
        ])
        .and_where(Expr::col(Brands::IsActive).eq(true));

    let brands_list = service.db_adapter.query_many::<Brand>(&query).await?;

    Ok(brands_list)
}
