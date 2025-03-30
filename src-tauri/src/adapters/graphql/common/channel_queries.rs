use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use juniper::FieldResult;

use crate::{
    core::{models::common::channel_model::Channel, types::db_uuid::DbUuid},
    schema::channels,
    AppState,
};

pub fn get_channel(id: DbUuid, context: &AppState) -> FieldResult<Channel> {
    let conn = &mut context.service.lock().unwrap().conn;
    let channel = channels::table
        .filter(channels::id.eq(id))
        .select(Channel::as_select())
        .get_result(conn)?;

    Ok(channel)
}

pub fn get_channels(context: &AppState) -> FieldResult<Vec<Channel>> {
    let conn = &mut context.service.lock().unwrap().conn;
    let channels_list = channels::table.select(Channel::as_select()).load(conn)?;

    Ok(channels_list)
}

pub fn get_active_channels(context: &AppState) -> FieldResult<Vec<Channel>> {
    let conn = &mut context.service.lock().unwrap().conn;
    let channels_list = channels::table
        .filter(channels::is_active.eq(true))
        .select(Channel::as_select())
        .load(conn)?;

    Ok(channels_list)
}
