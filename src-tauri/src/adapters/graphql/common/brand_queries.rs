use sea_query::{Expr, Query, SqliteQueryBuilder};
use juniper::FieldResult;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        models::common::brand_model::{Brand, Brands},
        types::db_uuid::DbUuid,
    },
    AppState,
};

pub fn get_brand(id: DbUuid, context: &AppState) -> FieldResult<Brand> {
    let service = context.service.lock().unwrap();
    
    let query = Query::select()
        .from(Brands::Table)
        .columns([
            Brands::Id,
            Brands::Name,
            Brands::Description,
            Brands::IsActive,
            Brands::CreatedAt,
            Brands::UpdatedAt,
        ])
        .and_where(Expr::col(Brands::Id).eq(id.to_string()))
        .to_string(SqliteQueryBuilder);
        
    let brand = service.db_adapter.query_one::<Brand>(&query, vec![])?;

    Ok(brand)
}

pub fn get_brands(context: &AppState) -> FieldResult<Vec<Brand>> {
    let service = context.service.lock().unwrap();
    
    let query = Query::select()
        .from(Brands::Table)
        .columns([
            Brands::Id,
            Brands::Name,
            Brands::Description,
            Brands::IsActive,
            Brands::CreatedAt,
            Brands::UpdatedAt,
        ])
        .to_string(SqliteQueryBuilder);
        
    let brands_list = service.db_adapter.query_many::<Brand>(&query, vec![])?;

    Ok(brands_list)
}

pub fn get_active_brands(context: &AppState) -> FieldResult<Vec<Brand>> {
    let service = context.service.lock().unwrap();
    
    let query = Query::select()
        .from(Brands::Table)
        .columns([
            Brands::Id,
            Brands::Name,
            Brands::Description,
            Brands::IsActive,
            Brands::CreatedAt,
            Brands::UpdatedAt,
        ])
        .and_where(Expr::col(Brands::IsActive).eq(true))
        .to_string(SqliteQueryBuilder);
        
    let brands_list = service.db_adapter.query_many::<Brand>(&query, vec![])?;

    Ok(brands_list)
}