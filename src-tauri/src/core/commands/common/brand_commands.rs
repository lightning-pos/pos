use chrono::Utc;
use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::{
    core::{
        commands::{app_service::AppService, Command},
        models::common::brand_model::{
            Brand, BrandNewInput, BrandUpdateChangeset, BrandUpdateInput,
        },
        types::db_uuid::DbUuid,
    },
    error::{Error, Result},
    schema::brands,
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
        service.conn.transaction(|conn| {
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
            diesel::insert_into(brands::table)
                .values(&new_brand)
                .execute(conn)?;

            Ok(new_brand)
        })
    }
}

impl Command for UpdateBrandCommand {
    type Output = Brand;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            // Get the existing brand
            let brand = brands::table
                .filter(brands::id.eq(&self.brand.id))
                .select(Brand::as_select())
                .get_result::<Brand>(conn)?;

            let now = Utc::now().naive_utc();

            // Create changeset
            let changeset = BrandUpdateChangeset {
                name: self.brand.name.clone(),
                description: self.brand.description.clone(),
                is_active: self.brand.is_active,
                updated_at: now,
            };

            // Update the brand
            diesel::update(brands::table)
                .filter(brands::id.eq(&self.brand.id))
                .set(&changeset)
                .execute(conn)?;

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
        service.conn.transaction(|conn| {
            let result = diesel::delete(brands::table)
                .filter(brands::id.eq(&self.id))
                .execute(conn)?;

            if result == 0 {
                return Err(Error::NotFoundError);
            }

            Ok(result as i32)
        })
    }
}
