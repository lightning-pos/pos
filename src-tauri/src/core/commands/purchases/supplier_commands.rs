use chrono::Utc;
use diesel::{Connection, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::{
    core::{
        commands::{app_service::AppService, Command},
        models::purchases::supplier_model::{
            Supplier, SupplierNewInput, SupplierUpdateChangeset, SupplierUpdateInput,
        },
        types::db_uuid::DbUuid,
    },
    error::Result,
    schema::suppliers,
};

// Commands
pub struct CreateSupplierCommand {
    pub supplier: SupplierNewInput,
}

pub struct UpdateSupplierCommand {
    pub supplier: SupplierUpdateInput,
}

pub struct DeleteSupplierCommand {
    pub id: DbUuid,
}

// Command Implementations
impl Command for CreateSupplierCommand {
    type Output = Supplier;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let now = Utc::now().naive_utc();
            let new_supplier = Supplier {
                id: Uuid::now_v7().into(),
                name: self.supplier.name.clone(),
                address: self.supplier.address.clone(),
                phone: self.supplier.phone.clone(),
                created_at: now,
                updated_at: now,
            };

            let res = diesel::insert_into(suppliers::table)
                .values(&new_supplier)
                .returning(Supplier::as_returning())
                .get_result(conn)?;

            Ok(res)
        })
    }
}

impl Command for UpdateSupplierCommand {
    type Output = Supplier;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let now = Utc::now().naive_utc();
            let supplier_id = self.supplier.id;

            let changeset = SupplierUpdateChangeset {
                id: supplier_id,
                name: self.supplier.name.clone(),
                address: self.supplier.address.clone(),
                phone: self.supplier.phone.clone(),
                updated_at: now,
            };

            let res = diesel::update(suppliers::table.find(supplier_id))
                .set(changeset)
                .returning(Supplier::as_returning())
                .get_result(conn)?;

            Ok(res)
        })
    }
}

impl Command for DeleteSupplierCommand {
    type Output = i32;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let res = diesel::delete(suppliers::table.find(self.id)).execute(conn)?;
            Ok(res as i32)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::suppliers;
    use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

    #[test]
    fn test_create_supplier() {
        let mut service = AppService::new(":memory:");

        let command = CreateSupplierCommand {
            supplier: SupplierNewInput {
                name: "Test Supplier".to_string(),
                address: Some("123 Test St".to_string()),
                phone: Some("123-456-7890".to_string()),
            },
        };

        let supplier = command.exec(&mut service).unwrap();
        assert_eq!(supplier.name, "Test Supplier");
        assert_eq!(supplier.address, Some("123 Test St".to_string()));
        assert_eq!(supplier.phone, Some("123-456-7890".to_string()));
    }

    #[test]
    fn test_create_supplier_minimal() {
        let mut service = AppService::new(":memory:");

        let command = CreateSupplierCommand {
            supplier: SupplierNewInput {
                name: "Test Supplier".to_string(),
                address: None,
                phone: None,
            },
        };

        let supplier = command.exec(&mut service).unwrap();
        assert_eq!(supplier.name, "Test Supplier");
        assert_eq!(supplier.address, None);
        assert_eq!(supplier.phone, None);
    }

    #[test]
    fn test_update_supplier() {
        let mut service = AppService::new(":memory:");

        // Create supplier
        let create_command = CreateSupplierCommand {
            supplier: SupplierNewInput {
                name: "Test Supplier".to_string(),
                address: Some("123 Test St".to_string()),
                phone: Some("123-456-7890".to_string()),
            },
        };

        let supplier = create_command.exec(&mut service).unwrap();

        // Update supplier
        let update_command = UpdateSupplierCommand {
            supplier: SupplierUpdateInput {
                id: supplier.id,
                name: Some("Updated Supplier".to_string()),
                address: Some(Some("456 New St".to_string())),
                phone: None,
            },
        };

        let updated_supplier = update_command.exec(&mut service).unwrap();
        assert_eq!(updated_supplier.name, "Updated Supplier");
        assert_eq!(updated_supplier.address, Some("456 New St".to_string()));
        assert_eq!(updated_supplier.phone, Some("123-456-7890".to_string())); // Unchanged
    }

    #[test]
    fn test_update_supplier_remove_field() {
        let mut service = AppService::new(":memory:");

        // Create supplier
        let create_command = CreateSupplierCommand {
            supplier: SupplierNewInput {
                name: "Test Supplier".to_string(),
                address: Some("123 Test St".to_string()),
                phone: Some("123-456-7890".to_string()),
            },
        };

        let supplier = create_command.exec(&mut service).unwrap();

        // Update supplier - remove address
        let update_command = UpdateSupplierCommand {
            supplier: SupplierUpdateInput {
                id: supplier.id,
                name: None,
                address: Some(None), // Remove address
                phone: None,
            },
        };

        let updated_supplier = update_command.exec(&mut service).unwrap();
        assert_eq!(updated_supplier.name, "Test Supplier"); // Unchanged
        assert_eq!(updated_supplier.address, None); // Removed
        assert_eq!(updated_supplier.phone, Some("123-456-7890".to_string())); // Unchanged
    }

    #[test]
    fn test_update_nonexistent_supplier() {
        let mut service = AppService::new(":memory:");

        let update_command = UpdateSupplierCommand {
            supplier: SupplierUpdateInput {
                id: Uuid::now_v7().into(),
                name: Some("Updated Supplier".to_string()),
                address: None,
                phone: None,
            },
        };

        let result = update_command.exec(&mut service);
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_supplier() {
        let mut service = AppService::new(":memory:");

        // Create supplier
        let create_command = CreateSupplierCommand {
            supplier: SupplierNewInput {
                name: "Test Supplier".to_string(),
                address: None,
                phone: None,
            },
        };

        let supplier = create_command.exec(&mut service).unwrap();

        // Delete supplier
        let delete_command = DeleteSupplierCommand { id: supplier.id };
        let result = delete_command.exec(&mut service).unwrap();
        assert_eq!(result, 1);

        // Verify supplier no longer exists
        let count: i64 = suppliers::table
            .filter(suppliers::id.eq(supplier.id))
            .count()
            .get_result(&mut service.conn)
            .unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_delete_nonexistent_supplier() {
        let mut service = AppService::new(":memory:");

        // Delete non-existent supplier
        let delete_command = DeleteSupplierCommand {
            id: Uuid::now_v7().into(),
        };
        let result = delete_command.exec(&mut service).unwrap();

        // This should return 0 rows affected, not an error
        assert_eq!(result, 0);
    }
}
