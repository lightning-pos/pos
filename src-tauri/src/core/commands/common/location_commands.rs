use chrono::Utc;
use sea_query::{Expr, Query, SqliteQueryBuilder};
use uuid::Uuid;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        commands::{app_service::AppService, Command},
        models::common::location_model::{Location, LocationNewInput, LocationUpdateInput, Locations},
        types::db_uuid::DbUuid,
    },
    error::Result,
};

pub struct CreateLocationCommand {
    pub location: LocationNewInput,
}

pub struct UpdateLocationCommand {
    pub location: LocationUpdateInput,
}

pub struct DeleteLocationCommand {
    pub id: DbUuid,
}

impl Command for CreateLocationCommand {
    type Output = Location;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let now = Utc::now().naive_utc();
        let is_active = self.location.is_active.unwrap_or(true);

        let new_location = Location {
            id: Uuid::now_v7().into(),
            name: self.location.name.clone(),
            description: self.location.description.clone(),
            address: self.location.address.clone(),
            is_active,
            created_at: now,
            updated_at: now,
        };

        // Insert the location
        let query = Query::insert()
            .into_table(Locations::Table)
            .columns([
                Locations::Id,
                Locations::Name,
                Locations::Description,
                Locations::Address,
                Locations::IsActive,
                Locations::CreatedAt,
                Locations::UpdatedAt,
            ])
            .values_panic([
                new_location.id.to_string().into(),
                new_location.name.clone().into(),
                match &new_location.description {
                    Some(desc) => desc.clone().into(),
                    None => sea_query::Value::String(None).into(),
                },
                match &new_location.address {
                    Some(addr) => addr.clone().into(),
                    None => sea_query::Value::String(None).into(),
                },
                new_location.is_active.to_string().into(),
                new_location.created_at.to_string().into(),
                new_location.updated_at.to_string().into(),
            ])
            .to_string(SqliteQueryBuilder);

        service.db_adapter.execute(&query).await?;

        Ok(new_location)
    }
}

impl Command for UpdateLocationCommand {
    type Output = Location;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // First, get the existing location
        let mut query_builder = Query::select();
        let query = query_builder
            .from(Locations::Table)
            .columns([
                Locations::Id,
                Locations::Name,
                Locations::Description,
                Locations::Address,
                Locations::IsActive,
                Locations::CreatedAt,
                Locations::UpdatedAt,
            ])
            .and_where(Expr::col(Locations::Id).eq(self.location.id.to_string()));

        let existing_location = service.db_adapter.query_optional::<Location>(&query).await?;
        if existing_location.is_none() {
            return Err(crate::error::Error::NotFoundError);
        }
        let existing_location = existing_location.unwrap();

        let now = Utc::now().naive_utc();

        // Build update query
        let mut update_query = Query::update();
        let update = update_query
            .table(Locations::Table)
            .and_where(Expr::col(Locations::Id).eq(self.location.id.to_string()))
            .value(Locations::UpdatedAt, now.to_string());

        if let Some(name) = &self.location.name {
            update.value(Locations::Name, name.clone());
        }

        if let Some(description) = &self.location.description {
            match description {
                Some(desc) => update.value(Locations::Description, desc.clone()),
                None => update.value(Locations::Description, sea_query::Value::String(None)),
            };
        }

        if let Some(address) = &self.location.address {
            match address {
                Some(addr) => update.value(Locations::Address, addr.clone()),
                None => update.value(Locations::Address, sea_query::Value::String(None)),
            };
        }

        if let Some(is_active) = self.location.is_active {
            update.value(Locations::IsActive, is_active.to_string());
        }

        let sql = update.to_string(SqliteQueryBuilder);
        service.db_adapter.execute(&sql).await?;

        // Return the updated location
        let updated_location = Location {
            id: existing_location.id,
            name: self.location.name.clone().unwrap_or(existing_location.name),
            description: match &self.location.description {
                Some(Some(desc)) => Some(desc.clone()),
                Some(None) => None,
                None => existing_location.description,
            },
            address: match &self.location.address {
                Some(Some(addr)) => Some(addr.clone()),
                Some(None) => None,
                None => existing_location.address,
            },
            is_active: self.location.is_active.unwrap_or(existing_location.is_active),
            created_at: existing_location.created_at,
            updated_at: now,
        };

        Ok(updated_location)
    }
}

impl Command for DeleteLocationCommand {
    type Output = ();

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let query = Query::delete()
            .from_table(Locations::Table)
            .and_where(Expr::col(Locations::Id).eq(self.id.to_string()))
            .to_string(SqliteQueryBuilder);

        service.db_adapter.execute(&query).await?;

        Ok(())
    }
}
