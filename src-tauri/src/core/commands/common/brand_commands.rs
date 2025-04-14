use chrono::Utc;
use sea_query::{Expr, Query, SqliteQueryBuilder};
use uuid::Uuid;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        commands::{app_service::AppService, Command},
        models::common::brand_model::{
            Brand, BrandNewInput, BrandUpdateInput, Brands,
        },
        types::db_uuid::DbUuid,
    },
    error::{Error, Result},
};

// Commands
pub struct CreateBrandCommand {
    pub brand: BrandNewInput,
}

pub struct UpdateBrandCommand {
    pub brand: BrandUpdateInput,
}

pub struct DeleteBrandCommand {
    pub id: DbUuid,
}

// Command Implementations
impl Command for CreateBrandCommand {
    type Output = Brand;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.db_adapter.transaction(|db| {
            let now = Utc::now().naive_utc();
            let new_brand = Brand {
                id: Uuid::now_v7().into(),
                name: self.brand.name.clone(),
                description: self.brand.description.clone(),
                is_active: self.brand.is_active.unwrap_or(true),
                created_at: now,
                updated_at: now,
            };

            // Insert the brand
            let query = Query::insert()
                .into_table(Brands::Table)
                .columns([
                    Brands::Id,
                    Brands::Name,
                    Brands::Description,
                    Brands::IsActive,
                    Brands::CreatedAt,
                    Brands::UpdatedAt,
                ])
                .values_panic([
                    new_brand.id.to_string().into(),
                    new_brand.name.clone().into(),
                    match &new_brand.description {
                        Some(desc) => desc.clone().into(),
                        None => sea_query::Value::String(None).into(),
                    },
                    new_brand.is_active.to_string().into(),
                    new_brand.created_at.to_string().into(),
                    new_brand.updated_at.to_string().into(),
                ])
                .to_string(SqliteQueryBuilder);

            db.execute(&query, vec![])?;

            Ok(new_brand)
        })
    }
}

impl Command for UpdateBrandCommand {
    type Output = Brand;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.db_adapter.transaction(|db| {
            // Get the existing brand
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
                .and_where(Expr::col(Brands::Id).eq(self.brand.id.to_string()))
                .to_string(SqliteQueryBuilder);

            let brand = db.query_optional::<Brand>(&query, vec![])?;
            if brand.is_none() {
                return Err(Error::NotFoundError);
            }
            let brand = brand.unwrap();

            let now = Utc::now().naive_utc();

            // Build update query
            let mut update_query = Query::update();
            let update = update_query
                .table(Brands::Table)
                .and_where(Expr::col(Brands::Id).eq(self.brand.id.to_string()))
                .value(Brands::UpdatedAt, now.to_string());

            if let Some(name) = &self.brand.name {
                update.value(Brands::Name, name.clone());
            }

            if let Some(description) = &self.brand.description {
                match description {
                    Some(desc) => update.value(Brands::Description, desc.clone()),
                    None => update.value(Brands::Description, sea_query::Value::String(None)),
                };
            }

            if let Some(is_active) = self.brand.is_active {
                update.value(Brands::IsActive, is_active.to_string());
            }

            let sql = update.to_string(SqliteQueryBuilder);
            db.execute(&sql, vec![])?;

            // Return the updated brand
            let updated_brand = Brand {
                id: brand.id,
                name: self.brand.name.clone().unwrap_or(brand.name),
                description: match &self.brand.description {
                    Some(Some(desc)) => Some(desc.clone()),
                    Some(None) => None,
                    None => brand.description,
                },
                is_active: self.brand.is_active.unwrap_or(brand.is_active),
                created_at: brand.created_at,
                updated_at: now,
            };

            Ok(updated_brand)
        })
    }
}

impl Command for DeleteBrandCommand {
    type Output = i32;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.db_adapter.transaction(|db| {
            let query = Query::delete()
                .from_table(Brands::Table)
                .and_where(Expr::col(Brands::Id).eq(self.id.to_string()))
                .to_string(SqliteQueryBuilder);

            let result = db.execute(&query, vec![])?;

            if result == 0 {
                return Err(Error::NotFoundError);
            }

            Ok(result as i32)
        })
    }
}
