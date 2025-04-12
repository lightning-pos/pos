use chrono::Utc;
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::{
    core::{
        commands::{app_service::AppService, Command},
        models::common::location_model::{Location, LocationNewInput, LocationUpdateInput},
        types::db_uuid::DbUuid,
    },
    error::Result,
    schema::locations,
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

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
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

        let location = diesel::insert_into(locations::table)
            .values(&new_location)
            .returning(Location::as_returning())
            .get_result(&mut service.conn)?;

        Ok(location)
    }
}

impl Command for UpdateLocationCommand {
    type Output = Location;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let now = Utc::now().naive_utc();
        let changeset = self.location.clone().into_changeset(now);

        let location = diesel::update(locations::table.find(self.location.id))
            .set(changeset)
            .returning(Location::as_returning())
            .get_result(&mut service.conn)?;

        Ok(location)
    }
}

impl Command for DeleteLocationCommand {
    type Output = ();

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        diesel::delete(locations::table.find(self.id))
            .execute(&mut service.conn)?;

        Ok(())
    }
}
