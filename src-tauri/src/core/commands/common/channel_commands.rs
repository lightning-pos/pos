use chrono::Utc;
use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::{
    core::{
        commands::{app_service::AppService, Command},
        models::common::channel_model::{
            Channel, ChannelNewInput, ChannelUpdateChangeset, ChannelUpdateInput,
        },
        types::db_uuid::DbUuid,
    },
    error::{Error, Result},
    schema::channels,
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

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
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
            diesel::insert_into(channels::table)
                .values(&new_channel)
                .execute(conn)?;

            Ok(new_channel)
        })
    }
}

impl Command for UpdateChannelCommand {
    type Output = Channel;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            // Get the existing channel
            let channel = channels::table
                .filter(channels::id.eq(&self.channel.id))
                .select(Channel::as_select())
                .get_result::<Channel>(conn)?;

            let now = Utc::now().naive_utc();

            // Create changeset
            let changeset = ChannelUpdateChangeset {
                name: self.channel.name.clone(),
                description: self.channel.description.clone(),
                is_active: self.channel.is_active,
                updated_at: now,
            };

            // Update the channel
            diesel::update(channels::table)
                .filter(channels::id.eq(&self.channel.id))
                .set(&changeset)
                .execute(conn)?;

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
        })
    }
}

impl Command for DeleteChannelCommand {
    type Output = i32;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let result = diesel::delete(channels::table)
                .filter(channels::id.eq(&self.id))
                .execute(conn)?;

            if result == 0 {
                return Err(Error::NotFoundError);
            }

            Ok(result as i32)
        })
    }
}
