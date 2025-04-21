use chrono::Utc;
use sea_query::{Expr, Query};
use uuid::Uuid;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        commands::{app_service::AppService, Command},
        models::common::channel_model::{
            Channel, ChannelNewInput, ChannelUpdateInput, Channels,
        },
        types::db_uuid::DbUuid,
    },
    error::{Error, Result},
};

// Commands
pub struct CreateChannelCommand {
    pub channel: ChannelNewInput,
}

pub struct UpdateChannelCommand {
    pub channel: ChannelUpdateInput,
}

pub struct DeleteChannelCommand {
    pub id: DbUuid,
}

// Command Implementations
impl Command for CreateChannelCommand {
    type Output = Channel;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let now = Utc::now().naive_utc();
        let new_channel = Channel {
            id: Uuid::now_v7().into(),
            name: self.channel.name.clone(),
            description: self.channel.description.clone(),
            is_active: self.channel.is_active.unwrap_or(true),
            created_at: now,
            updated_at: now,
        };

        // Insert the channel
        let mut insert_query = Query::insert();
        let insert_stmt = insert_query
            .into_table(Channels::Table)
            .columns([
                Channels::Id,
                Channels::Name,
                Channels::Description,
                Channels::IsActive,
                Channels::CreatedAt,
                Channels::UpdatedAt,
            ])
            .values_panic([
                new_channel.id.to_string().into(),
                new_channel.name.clone().into(),
                match &new_channel.description {
                    Some(desc) => desc.clone().into(),
                    None => sea_query::Value::String(None).into(),
                },
                new_channel.is_active.to_string().into(),
                new_channel.created_at.to_string().into(),
                new_channel.updated_at.to_string().into(),
            ]);

        // Use insert_many instead of execute
        service.db_adapter.insert_many(&insert_stmt).await?;

        Ok(new_channel)
    }
}

impl Command for UpdateChannelCommand {
    type Output = Channel;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Get the existing channel
        let mut query_builder = Query::select();
        let select_stmt = query_builder
            .from(Channels::Table)
            .columns([
                Channels::Id,
                Channels::Name,
                Channels::Description,
                Channels::IsActive,
                Channels::CreatedAt,
                Channels::UpdatedAt,
            ])
            .and_where(Expr::col(Channels::Id).eq(self.channel.id.to_string()));

        let channel = service.db_adapter.query_optional::<Channel>(&select_stmt).await?;
        if channel.is_none() {
            return Err(Error::NotFoundError);
        }
        let channel = channel.unwrap();

        let now = Utc::now().naive_utc();

        // Build update query
        let mut update_query = Query::update();
        let update_stmt = update_query
            .table(Channels::Table)
            .and_where(Expr::col(Channels::Id).eq(self.channel.id.to_string()))
            .value(Channels::UpdatedAt, now.to_string());

        if let Some(name) = &self.channel.name {
            update_stmt.value(Channels::Name, name.clone());
        }

        if let Some(description) = &self.channel.description {
            match description {
                Some(desc) => update_stmt.value(Channels::Description, desc.clone()),
                None => update_stmt.value(Channels::Description, sea_query::Value::String(None)),
            };
        }

        if let Some(is_active) = self.channel.is_active {
            update_stmt.value(Channels::IsActive, is_active.to_string());
        }

        // Use update_many instead of execute
        service.db_adapter.update_many(&update_stmt).await?;

        // Return the updated channel
        let updated_channel = Channel {
            id: channel.id,
            name: self.channel.name.clone().unwrap_or(channel.name),
            description: match &self.channel.description {
                Some(Some(desc)) => Some(desc.clone()),
                Some(None) => None,
                None => channel.description,
            },
            is_active: self.channel.is_active.unwrap_or(channel.is_active),
            created_at: channel.created_at,
            updated_at: now,
        };

        Ok(updated_channel)
    }
}

impl Command for DeleteChannelCommand {
    type Output = i32;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let mut delete_query = Query::delete();
        let delete_stmt = delete_query
            .from_table(Channels::Table)
            .and_where(Expr::col(Channels::Id).eq(self.id.to_string()));

        let result = service.db_adapter.delete(&delete_stmt).await?;

        if result == 0 {
            return Err(Error::NotFoundError);
        }

        Ok(result as i32)
    }
}
