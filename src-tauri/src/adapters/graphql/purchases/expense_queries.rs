use diesel::{dsl::count, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use juniper::FieldResult;

use crate::{
    core::{models::purchases::expense_model::Expense, types::db_uuid::DbUuid},
    schema::expenses,
    AppState,
};

pub fn expenses(
    first: Option<i32>,
    offset: Option<i32>,
    context: &AppState,
) -> FieldResult<Vec<Expense>> {
    println!("yoyo inside expenses");
    let mut service = context.service.lock().unwrap();
    let mut query = expenses::table.into_boxed();

    // Order by expense date descending (newest first)
    query = query.order(expenses::expense_date.desc());

    if let Some(limit) = first {
        query = query.limit(limit as i64);
    }
    if let Some(off) = offset {
        query = query.offset(off as i64);
    }
    let result = query
        .select(Expense::as_select())
        .load::<Expense>(&mut service.conn)?;

    println!("yoyo result: {:?}", result);
    Ok(result)
}

pub fn total_expenses(context: &AppState) -> FieldResult<i32> {
    let mut service = context.service.lock().unwrap();
    let result: i64 = expenses::table
        .select(count(expenses::id))
        .get_result(&mut service.conn)?;
    Ok(result as i32)
}

pub fn expense(id: DbUuid, context: &AppState) -> FieldResult<Expense> {
    let mut service = context.service.lock().unwrap();
    let result = expenses::table
        .filter(expenses::id.eq(id))
        .select(Expense::as_select())
        .get_result(&mut service.conn)?;
    Ok(result)
}

pub fn expenses_by_category(
    category_id: DbUuid,
    first: Option<i32>,
    offset: Option<i32>,
    context: &AppState,
) -> FieldResult<Vec<Expense>> {
    let mut service = context.service.lock().unwrap();
    let mut query = expenses::table
        .filter(expenses::category_id.eq(category_id))
        .order(expenses::expense_date.desc())
        .into_boxed();

    if let Some(limit) = first {
        query = query.limit(limit as i64);
    }
    if let Some(off) = offset {
        query = query.offset(off as i64);
    }
    let result = query
        .select(Expense::as_select())
        .load::<Expense>(&mut service.conn)?;
    Ok(result)
}
