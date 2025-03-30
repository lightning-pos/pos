use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use juniper::FieldResult;

use crate::{
    core::{models::finance::payment_method_model::PaymentMethod, types::db_uuid::DbUuid},
    schema::payment_methods,
    AppState,
};

pub fn payment_methods(
    first: Option<i32>,
    offset: Option<i32>,
    context: &AppState,
) -> FieldResult<Vec<PaymentMethod>> {
    let mut service = context.service.lock().unwrap();

    let mut query = payment_methods::table.into_boxed();

    // Apply pagination if parameters are provided
    if let Some(limit) = first {
        query = query.limit(limit as i64);
    }
    if let Some(off) = offset {
        query = query.offset(off as i64);
    }

    let result = query
        .select(PaymentMethod::as_select())
        .load::<PaymentMethod>(&mut service.conn)?;

    Ok(result)
}

pub fn payment_method(id: DbUuid, context: &AppState) -> FieldResult<PaymentMethod> {
    let mut service = context.service.lock().unwrap();
    let result = payment_methods::table
        .filter(payment_methods::id.eq(id))
        .first::<PaymentMethod>(&mut service.conn)?;
    Ok(result)
}

pub fn all_payment_methods(context: &AppState) -> FieldResult<Vec<PaymentMethod>> {
    let mut service = context.service.lock().unwrap();

    use crate::core::models::finance::payment_method_model::PaymentMethodState;

    let result = payment_methods::table
        .filter(payment_methods::state.eq(PaymentMethodState::Active))
        .load::<PaymentMethod>(&mut service.conn)?;
    Ok(result)
}

pub fn total_payment_methods(context: &AppState) -> FieldResult<i32> {
    let mut service = context.service.lock().unwrap();
    let count: i64 = payment_methods::table
        .count()
        .get_result(&mut service.conn)?;
    Ok(count as i32)
}
