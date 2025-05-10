use chrono::Utc;
use sea_query::{Expr, Query};
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

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
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
        let mut insert_query = Query::insert();
        let insert_stmt = insert_query
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
            ]);

        // Use insert_many instead of execute
        service.db_adapter.insert_many(&insert_stmt).await?;

        Ok(new_brand)
    }
}

impl Command for UpdateBrandCommand {
    type Output = Brand;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Get the existing brand
        let mut query_builder = Query::select();
        let select_stmt = query_builder
            .from(Brands::Table)
            .columns([
                Brands::Id,
                Brands::Name,
                Brands::Description,
                Brands::IsActive,
                Brands::CreatedAt,
                Brands::UpdatedAt,
            ])
            .and_where(Expr::col(Brands::Id).eq(self.brand.id.to_string()));

        let brand = service.db_adapter.query_optional::<Brand>(&select_stmt).await?;
        if brand.is_none() {
            return Err(Error::NotFoundError);
        }
        let brand = brand.unwrap();

        let now = Utc::now().naive_utc();

        // Build update query
        let mut update_query = Query::update();
        let update_stmt = update_query
            .table(Brands::Table)
            .and_where(Expr::col(Brands::Id).eq(self.brand.id.to_string()))
            .value(Brands::UpdatedAt, now.to_string());

        if let Some(name) = &self.brand.name {
            update_stmt.value(Brands::Name, name.clone());
        }

        if let Some(description) = &self.brand.description {
            match description {
                Some(desc) => update_stmt.value(Brands::Description, desc.clone()),
                None => update_stmt.value(Brands::Description, sea_query::Value::String(None)),
            };
        }

        if let Some(is_active) = self.brand.is_active {
            update_stmt.value(Brands::IsActive, is_active.to_string());
        }

        // Use update_many instead of execute
        service.db_adapter.update_many(&update_stmt).await?;

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
    }
}

impl Command for DeleteBrandCommand {
    type Output = i32;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let mut delete_query = Query::delete();
        let delete_stmt = delete_query
            .from_table(Brands::Table)
            .and_where(Expr::col(Brands::Id).eq(self.id.to_string()));

        let result = service.db_adapter.delete(&delete_stmt).await?;

        if result == 0 {
            return Err(Error::NotFoundError);
        }

        Ok(result as i32)
    }
}
