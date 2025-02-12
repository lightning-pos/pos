use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use juniper::FieldResult;

use crate::{
    core::{models::auth::user_model::User, types::db_uuid::DbUuid},
    schema::users,
    AppState,
};

pub fn users(
    first: Option<i32>,
    offset: Option<i32>,
    context: &AppState,
) -> FieldResult<Vec<User>> {
    let mut service = context.service.lock().unwrap();
    let mut query = users::table.into_boxed();
    if let Some(limit) = first {
        query = query.limit(limit as i64);
    }
    if let Some(off) = offset {
        query = query.offset(off as i64);
    }
    let result = query
        .select(User::as_select())
        .load::<User>(&mut service.conn)?;
    Ok(result)
}

pub fn user(id: DbUuid, context: &AppState) -> FieldResult<User> {
    let mut service = context.service.lock().unwrap();
    let result = users::table
        .filter(users::id.eq(id))
        .select(User::as_select())
        .get_result(&mut service.conn)?;
    Ok(result)
}
