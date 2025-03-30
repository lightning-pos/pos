use chrono::NaiveDateTime;
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
    cost_center_id: Option<DbUuid>,
    start_date: Option<String>,
    end_date: Option<String>,
    context: &AppState,
) -> FieldResult<Vec<Expense>> {
    let mut service = context.service.lock().unwrap();
    let mut query = expenses::table.into_boxed();

    // Apply cost center filter if provided
    if let Some(cc_id) = cost_center_id {
        query = query.filter(expenses::cost_center_id.eq(cc_id));
    }

    // Apply date range filters if provided
    if let Some(start) = start_date {
        if let Ok(date) = NaiveDateTime::parse_from_str(&start, "%Y-%m-%dT%H:%M:%S%.f%z") {
            query = query.filter(expenses::expense_date.ge(date));
        } else if let Ok(date) = NaiveDateTime::parse_from_str(&start, "%Y-%m-%dT%H:%M:%S%.f") {
            query = query.filter(expenses::expense_date.ge(date));
        }
    }

    if let Some(end) = end_date {
        if let Ok(date) = NaiveDateTime::parse_from_str(&end, "%Y-%m-%dT%H:%M:%S%.f%z") {
            query = query.filter(expenses::expense_date.le(date));
        } else if let Ok(date) = NaiveDateTime::parse_from_str(&end, "%Y-%m-%dT%H:%M:%S%.f") {
            query = query.filter(expenses::expense_date.le(date));
        }
    }

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

    Ok(result)
}

pub fn total_expenses(
    cost_center_id: Option<DbUuid>,
    start_date: Option<String>,
    end_date: Option<String>,
    context: &AppState,
) -> FieldResult<i32> {
    let mut service = context.service.lock().unwrap();
    let mut query = expenses::table.into_boxed();

    // Apply cost center filter if provided
    if let Some(cc_id) = cost_center_id {
        query = query.filter(expenses::cost_center_id.eq(cc_id));
    }

    // Apply date range filters if provided
    if let Some(start) = start_date {
        if let Ok(date) = NaiveDateTime::parse_from_str(&start, "%Y-%m-%dT%H:%M:%S%.f%z") {
            query = query.filter(expenses::expense_date.ge(date));
        } else if let Ok(date) = NaiveDateTime::parse_from_str(&start, "%Y-%m-%dT%H:%M:%S%.f") {
            query = query.filter(expenses::expense_date.ge(date));
        }
    }

    if let Some(end) = end_date {
        if let Ok(date) = NaiveDateTime::parse_from_str(&end, "%Y-%m-%dT%H:%M:%S%.f%z") {
            query = query.filter(expenses::expense_date.le(date));
        } else if let Ok(date) = NaiveDateTime::parse_from_str(&end, "%Y-%m-%dT%H:%M:%S%.f") {
            query = query.filter(expenses::expense_date.le(date));
        }
    }

    let result: i64 = query
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

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Utc};
    use diesel::RunQueryDsl;
    use std::sync::Mutex;
    use uuid::Uuid;

    use crate::core::{
        commands::AppService, models::purchases::expense_model::Expense, types::db_uuid::DbUuid,
        types::money::Money,
    };
    use crate::schema::expenses;

    use super::*;

    fn create_app_state() -> AppState {
        // Create a mutable service for database schema creation
        let mut service = AppService::new(":memory:");

        // Create test database schema if it doesn't exist
        diesel::sql_query(
            "CREATE TABLE IF NOT EXISTS expenses (
                id TEXT PRIMARY KEY NOT NULL,
                title TEXT NOT NULL,
                amount BIGINT NOT NULL,
                expense_date TIMESTAMP NOT NULL,
                category_id TEXT NOT NULL,
                cost_center_id TEXT NOT NULL,
                description TEXT,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL
            )",
        )
        .execute(&mut service.conn)
        .unwrap();

        AppState {
            service: Mutex::new(service),
        }
    }

    fn create_test_expense(
        service: &mut AppService,
        id: &str,
        title: &str,
        amount: f64,
        expense_date: NaiveDateTime,
        category_id: &str,
        cost_center_id: &str,
        description: Option<&str>,
    ) -> Expense {
        let now = Utc::now().naive_utc();
        let expense = Expense {
            id: DbUuid::from(Uuid::parse_str(id).unwrap()),
            title: title.to_string(),
            amount: Money::from_float(amount),
            expense_date,
            category_id: DbUuid::from(Uuid::parse_str(category_id).unwrap()),
            cost_center_id: DbUuid::from(Uuid::parse_str(cost_center_id).unwrap()),
            description: description.map(|s| s.to_string()),
            created_at: now,
            updated_at: now,
        };

        diesel::insert_into(expenses::table)
            .values(&expense)
            .execute(&mut service.conn)
            .unwrap();

        expense
    }

    fn create_test_data(app_state: &AppState) -> Vec<Expense> {
        let mut service = app_state.service.lock().unwrap();
        let mut result = Vec::new();

        // Category and cost center IDs
        let category_id_1 = "550e8400-e29b-41d4-a716-446655440000";
        let category_id_2 = "550e8400-e29b-41d4-a716-446655440001";
        let cost_center_id_1 = "550e8400-e29b-41d4-a716-446655440002";
        let cost_center_id_2 = "550e8400-e29b-41d4-a716-446655440003";

        // Create expenses with different dates
        let date1 = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2023, 1, 15).unwrap(),
            NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        );
        let date2 = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2023, 2, 20).unwrap(),
            NaiveTime::from_hms_opt(14, 30, 0).unwrap(),
        );
        let date3 = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2023, 3, 25).unwrap(),
            NaiveTime::from_hms_opt(16, 45, 0).unwrap(),
        );
        let date4 = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2023, 4, 5).unwrap(),
            NaiveTime::from_hms_opt(9, 15, 0).unwrap(),
        );
        let date5 = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2023, 5, 10).unwrap(),
            NaiveTime::from_hms_opt(11, 30, 0).unwrap(),
        );

        // Create test expenses
        result.push(create_test_expense(
            &mut service,
            "11111111-1111-1111-1111-111111111111",
            "Office Supplies",
            50.00,
            date1,
            category_id_1,
            cost_center_id_1,
            Some("Notebooks and pens"),
        ));

        result.push(create_test_expense(
            &mut service,
            "22222222-2222-2222-2222-222222222222",
            "Software License",
            150.00,
            date2,
            category_id_1,
            cost_center_id_2,
            Some("Annual subscription"),
        ));

        result.push(create_test_expense(
            &mut service,
            "33333333-3333-3333-3333-333333333333",
            "Office Furniture",
            300.00,
            date3,
            category_id_2,
            cost_center_id_1,
            None,
        ));

        result.push(create_test_expense(
            &mut service,
            "44444444-4444-4444-4444-444444444444",
            "Training Materials",
            75.00,
            date4,
            category_id_2,
            cost_center_id_2,
            Some("Employee training books"),
        ));

        result.push(create_test_expense(
            &mut service,
            "55555555-5555-5555-5555-555555555555",
            "Travel Expenses",
            250.00,
            date5,
            category_id_1,
            cost_center_id_1,
            Some("Business trip to conference"),
        ));

        result
    }

    #[test]
    fn test_expenses_no_filters() {
        let app_state = create_app_state();
        let expenses_data = create_test_data(&app_state);

        // Test fetching all expenses with no filters
        let result = expenses(None, None, None, None, None, &app_state).unwrap();

        // Should return all 5 expenses, sorted by date descending
        assert_eq!(result.len(), 5);

        // Verify the order (newest first)
        assert_eq!(result[0].id, expenses_data[4].id); // Travel Expenses (May)
        assert_eq!(result[1].id, expenses_data[3].id); // Training Materials (April)
        assert_eq!(result[2].id, expenses_data[2].id); // Office Furniture (March)
    }

    #[test]
    fn test_expenses_with_pagination() {
        let app_state = create_app_state();
        let expenses_data = create_test_data(&app_state);

        // Test with limit
        let result = expenses(Some(2), None, None, None, None, &app_state).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].id, expenses_data[4].id); // Most recent
        assert_eq!(result[1].id, expenses_data[3].id); // Second most recent

        // Test with offset
        let result = expenses(Some(2), Some(2), None, None, None, &app_state).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].id, expenses_data[2].id); // Third most recent
        assert_eq!(result[1].id, expenses_data[1].id); // Fourth most recent
    }

    #[test]
    fn test_expenses_by_cost_center() {
        let app_state = create_app_state();
        let _expenses_data = create_test_data(&app_state);

        // Test filtering by cost center
        let cost_center_id =
            DbUuid::from(Uuid::parse_str("550e8400-e29b-41d4-a716-446655440002").unwrap());
        let result = expenses(None, None, Some(cost_center_id), None, None, &app_state).unwrap();

        assert_eq!(result.len(), 3);

        // Check that all returned expenses have the correct cost center
        for expense in result {
            assert_eq!(expense.cost_center_id, cost_center_id);
        }
    }

    #[test]
    fn test_expenses_by_date_range() {
        let app_state = create_app_state();
        let _expenses_data = create_test_data(&app_state);

        // Test with start date only
        let start_date = "2023-03-01T00:00:00";
        let result = expenses(
            None,
            None,
            None,
            Some(start_date.to_string()),
            None,
            &app_state,
        )
        .unwrap();

        assert_eq!(result.len(), 3); // Should include Mar, Apr, May expenses

        // Verify all returned expenses have dates after the start date
        for expense in &result {
            assert!(
                expense.expense_date
                    >= NaiveDateTime::parse_from_str("2023-03-01T00:00:00", "%Y-%m-%dT%H:%M:%S")
                        .unwrap()
            );
        }

        // Test with end date only
        let end_date = "2023-03-31T23:59:59";
        let result = expenses(
            None,
            None,
            None,
            None,
            Some(end_date.to_string()),
            &app_state,
        )
        .unwrap();

        assert_eq!(result.len(), 3); // Should include Jan, Feb, Mar expenses

        // Verify all returned expenses have dates before the end date
        for expense in &result {
            assert!(
                expense.expense_date
                    <= NaiveDateTime::parse_from_str("2023-03-31T23:59:59", "%Y-%m-%dT%H:%M:%S")
                        .unwrap()
            );
        }

        // Test with both start and end date
        let start_date = "2023-02-01T00:00:00";
        let end_date = "2023-04-30T23:59:59";
        let result = expenses(
            None,
            None,
            None,
            Some(start_date.to_string()),
            Some(end_date.to_string()),
            &app_state,
        )
        .unwrap();

        assert_eq!(result.len(), 3); // Should include Feb, Mar, Apr expenses

        // Verify all returned expenses have dates within the range
        for expense in &result {
            assert!(
                expense.expense_date
                    >= NaiveDateTime::parse_from_str("2023-02-01T00:00:00", "%Y-%m-%dT%H:%M:%S")
                        .unwrap()
            );
            assert!(
                expense.expense_date
                    <= NaiveDateTime::parse_from_str("2023-04-30T23:59:59", "%Y-%m-%dT%H:%M:%S")
                        .unwrap()
            );
        }
    }

    #[test]
    fn test_expenses_combined_filters() {
        let app_state = create_app_state();
        let _expenses_data = create_test_data(&app_state);

        // Test with cost center and date range filters
        let cost_center_id =
            DbUuid::from(Uuid::parse_str("550e8400-e29b-41d4-a716-446655440002").unwrap());
        let start_date = "2023-01-01T00:00:00";
        let end_date = "2023-03-31T23:59:59";

        let result = expenses(
            None,
            None,
            Some(cost_center_id),
            Some(start_date.to_string()),
            Some(end_date.to_string()),
            &app_state,
        )
        .unwrap();

        assert_eq!(result.len(), 2); // Should include Jan and Mar expenses with the specified cost center

        // Verify all returned expenses meet all criteria
        for expense in &result {
            assert_eq!(expense.cost_center_id, cost_center_id);
            assert!(
                expense.expense_date
                    >= NaiveDateTime::parse_from_str("2023-01-01T00:00:00", "%Y-%m-%dT%H:%M:%S")
                        .unwrap()
            );
            assert!(
                expense.expense_date
                    <= NaiveDateTime::parse_from_str("2023-03-31T23:59:59", "%Y-%m-%dT%H:%M:%S")
                        .unwrap()
            );
        }
    }

    #[test]
    fn test_total_expenses() {
        let app_state = create_app_state();
        create_test_data(&app_state);

        // Test with no filters
        let result = total_expenses(None, None, None, &app_state).unwrap();
        assert_eq!(result, 5);

        // Test with cost center filter
        let cost_center_id =
            DbUuid::from(Uuid::parse_str("550e8400-e29b-41d4-a716-446655440002").unwrap());
        let result = total_expenses(Some(cost_center_id), None, None, &app_state).unwrap();
        assert_eq!(result, 3);

        // Test with date range
        let start_date = "2023-03-01T00:00:00";
        let end_date = "2023-05-31T23:59:59";
        let result = total_expenses(
            None,
            Some(start_date.to_string()),
            Some(end_date.to_string()),
            &app_state,
        )
        .unwrap();
        assert_eq!(result, 3); // Mar, Apr, May

        // Test with combined filters
        let result = total_expenses(
            Some(cost_center_id),
            Some(start_date.to_string()),
            Some(end_date.to_string()),
            &app_state,
        )
        .unwrap();
        assert_eq!(result, 2); // Mar, May with specified cost center
    }

    #[test]
    fn test_expense_by_id() {
        let app_state = create_app_state();
        let _expenses_data = create_test_data(&app_state);

        // Test fetching by ID
        let expense_id =
            DbUuid::from(Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap());
        let result = expense(expense_id, &app_state).unwrap();

        assert_eq!(result.id, expense_id);
        assert_eq!(result.title, "Office Supplies");
        assert_eq!(result.amount, Money::from_float(50.00));
    }

    #[test]
    fn test_expenses_by_category() {
        let app_state = create_app_state();
        let _expenses_data = create_test_data(&app_state);

        // Test fetching by category
        let category_id =
            DbUuid::from(Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap());

        // No pagination
        let result = expenses_by_category(category_id, None, None, &app_state).unwrap();
        assert_eq!(result.len(), 3); // Should have 3 expenses in this category

        // With pagination
        let result = expenses_by_category(category_id, Some(1), None, &app_state).unwrap();
        assert_eq!(result.len(), 1);

        let result = expenses_by_category(category_id, Some(2), Some(1), &app_state).unwrap();
        assert_eq!(result.len(), 2);

        // Verify all returned expenses have the correct category
        for expense in result {
            assert_eq!(expense.category_id, category_id);
        }
    }
}
