use chrono::Utc;
use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::{
    core::{
        commands::{app_service::AppService, Command},
        models::catalog::discount_model::{
            Discount, DiscountNewInput, DiscountState, DiscountUpdateChangeset, DiscountUpdateInput,
        },
        types::db_uuid::DbUuid,
    },
    error::{Error, Result},
    schema::discounts::dsl::*,
};

// --- Command Structs ---

pub struct CreateDiscountCommand {
    pub discount: DiscountNewInput,
}

pub struct UpdateDiscountCommand {
    pub discount: DiscountUpdateInput,
}

pub struct DeleteDiscountCommand {
    pub id: DbUuid,
}

pub struct GetDiscountCommand {
    pub id: DbUuid,
}

pub struct ListDiscountsCommand;

// --- Command Implementations ---

impl Command for CreateDiscountCommand {
    type Output = Discount;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            // Optional: Check for uniqueness by name
            let existing = discounts
                .filter(name.eq(&self.discount.name))
                .count()
                .get_result::<i64>(conn)?;
            if existing > 0 {
                return Err(Error::UniqueConstraintError);
            }

            let now = Utc::now().naive_utc();
            let new_discount = Discount {
                id: Uuid::now_v7().into(),
                name: self.discount.name.clone(),
                description: self.discount.description.clone(),
                discount_type: self.discount.discount_type,
                value: self.discount.value,
                scope: self.discount.scope,
                state: self.discount.state.unwrap_or(DiscountState::Active),
                start_date: self.discount.start_date,
                end_date: self.discount.end_date,
                created_at: now,
                updated_at: now,
            };

            let created_discount = diesel::insert_into(discounts)
                .values(&new_discount)
                .returning(Discount::as_returning())
                .get_result(conn)?;

            Ok(created_discount)
        })
    }
}

impl Command for UpdateDiscountCommand {
    type Output = Discount;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            // Ensure the discount exists before updating
            discounts
                .filter(id.eq(&self.discount.id))
                .select(id)
                .first::<DbUuid>(conn)?;

            let now = Utc::now().naive_utc();

            // Create changeset from input
            let discount_changeset = DiscountUpdateChangeset {
                id: self.discount.id,
                name: self.discount.name.clone(),
                description: self.discount.description.clone(),
                discount_type: self.discount.discount_type,
                value: self.discount.value,
                scope: self.discount.scope,
                state: self.discount.state,
                start_date: self.discount.start_date,
                end_date: self.discount.end_date,
                updated_at: now,
            };

            let updated_discount = diesel::update(discounts.filter(id.eq(&self.discount.id)))
                .set(&discount_changeset)
                .returning(Discount::as_returning())
                .get_result(conn)?;

            Ok(updated_discount)
        })
    }
}

impl Command for DeleteDiscountCommand {
    type Output = usize;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            // Optional: Add checks for foreign key constraints if discounts are linked elsewhere

            let num_deleted = diesel::delete(discounts.filter(id.eq(&self.id))).execute(conn)?;

            if num_deleted == 0 {
                Err(Error::NotFoundError)
            } else {
                Ok(num_deleted)
            }
        })
    }
}

impl Command for GetDiscountCommand {
    type Output = Discount;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let discount = discounts
                .filter(id.eq(&self.id))
                .select(Discount::as_select())
                .first::<Discount>(conn)
                .map_err(|_| Error::NotFoundError)?;
            Ok(discount)
        })
    }
}

impl Command for ListDiscountsCommand {
    type Output = Vec<Discount>;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let results = discounts
                .select(Discount::as_select())
                .load::<Discount>(conn)?;
            Ok(results)
        })
    }
}

// --- Tests ---
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{
        models::catalog::discount_model::{DiscountScope, DiscountType},
        types::money::Money,
    };
    // Assuming AppService::new(":memory:") works for setting up an in-memory SQLite DB

    fn setup_service() -> AppService {
        AppService::new(":memory:")
    }

    fn create_basic_discount(service: &mut AppService) -> Discount {
        let new_discount_info = DiscountNewInput {
            name: "Test Discount".to_string(),
            description: Some("Basic test discount".to_string()),
            discount_type: DiscountType::Percentage,
            value: Money::from_float(10.0),
            scope: DiscountScope::AllItems,
            state: Some(DiscountState::Active),
            start_date: None,
            end_date: None,
        };
        let create_cmd = CreateDiscountCommand {
            discount: new_discount_info,
        };
        create_cmd
            .exec(service)
            .expect("Failed to create test discount")
    }

    #[test]
    fn test_create_discount() {
        let mut service = setup_service();
        let created = create_basic_discount(&mut service);

        assert_eq!(created.name, "Test Discount");
        assert_eq!(created.discount_type, DiscountType::Percentage);
        assert_eq!(created.value, Money::from_float(10.0));
        assert_eq!(created.state, DiscountState::Active);
    }

    #[test]
    fn test_create_discount_unique_constraint() {
        let mut service = setup_service();
        create_basic_discount(&mut service); // Create first one

        // Try creating another with the same name
        let new_discount_info = DiscountNewInput {
            name: "Test Discount".to_string(), // Same name
            description: None,
            discount_type: DiscountType::FixedAmount,
            value: Money::from_float(5.0),
            scope: DiscountScope::AllItems,
            state: None,
            start_date: None,
            end_date: None,
        };
        let create_cmd = CreateDiscountCommand {
            discount: new_discount_info,
        };
        let result = create_cmd.exec(&mut service);

        assert!(matches!(result, Err(Error::UniqueConstraintError)));
    }

    #[test]
    fn test_get_discount() {
        let mut service = setup_service();
        let created = create_basic_discount(&mut service);

        let get_cmd = GetDiscountCommand { id: created.id };
        let fetched = get_cmd.exec(&mut service).expect("Failed to get discount");

        assert_eq!(fetched.id, created.id);
        assert_eq!(fetched.name, "Test Discount");
    }

    #[test]
    fn test_get_discount_not_found() {
        let mut service = setup_service();
        let non_existent_id = Uuid::now_v7().into();
        let get_cmd = GetDiscountCommand {
            id: non_existent_id,
        };
        let result = get_cmd.exec(&mut service);

        assert!(matches!(result, Err(Error::NotFoundError)));
    }

    #[test]
    fn test_list_discounts() {
        let mut service = setup_service();
        let created1 = create_basic_discount(&mut service);

        let list_cmd = ListDiscountsCommand;
        let list1 = list_cmd
            .exec(&mut service)
            .expect("Failed to list discounts");
        assert_eq!(list1.len(), 1);
        assert_eq!(list1[0].id, created1.id);

        // Add another discount
        let new_discount_info = DiscountNewInput {
            name: "Another Discount".to_string(),
            description: None,
            discount_type: DiscountType::FixedAmount,
            value: Money::from_float(5.0),
            scope: DiscountScope::AllItems,
            state: Some(DiscountState::Inactive),
            start_date: None,
            end_date: None,
        };
        let create_cmd = CreateDiscountCommand {
            discount: new_discount_info,
        };
        let created2 = create_cmd
            .exec(&mut service)
            .expect("Failed to create second discount");

        let list2 = list_cmd
            .exec(&mut service)
            .expect("Failed to list discounts again");
        assert_eq!(list2.len(), 2);
        // Check if both IDs are present (order might vary)
        assert!(list2.iter().any(|d| d.id == created1.id));
        assert!(list2.iter().any(|d| d.id == created2.id));
    }

    #[test]
    fn test_update_discount() {
        let mut service = setup_service();
        let created = create_basic_discount(&mut service);

        let update_info = DiscountUpdateInput {
            id: created.id,
            name: Some("Updated Discount Name".to_string()),
            description: Some(Some("Updated description".to_string())), // Update description
            discount_type: Some(DiscountType::FixedAmount),
            value: Some(Money::from_float(25.50)),
            scope: None, // Keep scope same
            state: Some(DiscountState::Inactive),
            start_date: None,
            end_date: None,
        };
        let update_cmd = UpdateDiscountCommand {
            discount: update_info,
        };
        let updated = update_cmd
            .exec(&mut service)
            .expect("Failed to update discount");

        assert_eq!(updated.id, created.id);
        assert_eq!(updated.name, "Updated Discount Name");
        assert_eq!(updated.description, Some("Updated description".to_string()));
        assert_eq!(updated.discount_type, DiscountType::FixedAmount);
        assert_eq!(updated.value, Money::from_float(25.50));
        assert_eq!(updated.state, DiscountState::Inactive);
        assert!(updated.updated_at > created.updated_at); // Check timestamp updated
    }

    #[test]
    fn test_update_discount_not_found() {
        let mut service = setup_service();
        let non_existent_id = Uuid::now_v7().into();
        let update_info = DiscountUpdateInput {
            id: non_existent_id,
            name: Some("Does not matter".to_string()),
            description: None,
            discount_type: None,
            value: None,
            scope: None,
            state: None,
            start_date: None,
            end_date: None,
        };
        let update_cmd = UpdateDiscountCommand {
            discount: update_info,
        };
        let result = update_cmd.exec(&mut service);

        // This should fail because the initial query in the command fails
        assert!(matches!(result, Err(Error::DieselError(_)))); // Or potentially NotFoundError depending on how DieselError maps
    }

    #[test]
    fn test_delete_discount() {
        let mut service = setup_service();
        let created = create_basic_discount(&mut service);

        // Delete
        let delete_cmd = DeleteDiscountCommand { id: created.id };
        let delete_result = delete_cmd.exec(&mut service).expect("Delete failed");
        assert_eq!(delete_result, 1); // 1 row deleted

        // Verify deletion
        let get_cmd = GetDiscountCommand { id: created.id };
        let get_result = get_cmd.exec(&mut service);
        assert!(matches!(get_result, Err(Error::NotFoundError)));

        // Verify list is empty
        let list_cmd = ListDiscountsCommand;
        let list = list_cmd
            .exec(&mut service)
            .expect("List failed after delete");
        assert!(list.is_empty());
    }

    #[test]
    fn test_delete_discount_not_found() {
        let mut service = setup_service();
        let non_existent_id = Uuid::now_v7().into();

        let delete_cmd = DeleteDiscountCommand {
            id: non_existent_id,
        };
        let result = delete_cmd.exec(&mut service);

        assert!(matches!(result, Err(Error::NotFoundError)));
    }
}
