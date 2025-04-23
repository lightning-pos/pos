use chrono::Utc;
use sea_query::{Expr, Query};
use uuid::Uuid;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        commands::{app_service::AppService, Command},
        models::purchases::supplier_model::{
            Supplier, SupplierNewInput, SupplierUpdateInput, Suppliers,
        },
        types::db_uuid::DbUuid,
    },
    error::{Error, Result},
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

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let now = Utc::now().naive_utc();
        let new_id = Uuid::now_v7();

        let new_supplier = Supplier {
            id: new_id.into(),
            name: self.supplier.name.clone(),
            address: self.supplier.address.clone(),
            phone: self.supplier.phone.clone(),
            created_at: now,
            updated_at: now,
        };

        // Build the insert query with SeaQuery
        let mut insert_query = Query::insert();
        let insert_stmt = insert_query
            .into_table(Suppliers::Table)
            .columns([
                Suppliers::Id,
                Suppliers::Name,
                Suppliers::Address,
                Suppliers::Phone,
                Suppliers::CreatedAt,
                Suppliers::UpdatedAt,
            ])
            .values_panic([
                new_id.to_string().into(),
                self.supplier.name.clone().into(),
                self.supplier.address.clone().into(),
                self.supplier.phone.clone().into(),
                now.to_string().into(),
                now.to_string().into(),
            ]);

        // Execute the query
        service.db_adapter.insert_one::<Supplier>(&insert_stmt).await?;

        // Return the newly created supplier
        Ok(new_supplier)
    }
}

impl Command for UpdateSupplierCommand {
    type Output = Supplier;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let now = Utc::now().naive_utc();
        let supplier_id = self.supplier.id;

        // First, check if the supplier exists
        let mut check_query = Query::select();
        let check_stmt = check_query
            .from(Suppliers::Table)
            .columns([
                Suppliers::Id,
                Suppliers::Name,
                Suppliers::Address,
                Suppliers::Phone,
                Suppliers::CreatedAt,
                Suppliers::UpdatedAt,
            ])
            .and_where(Expr::col(Suppliers::Id).eq(supplier_id.to_string()));

        let existing = service.db_adapter.query_optional::<Supplier>(&check_stmt).await?;

        if existing.is_none() {
            return Err(Error::NotFoundError);
        }

        // Build the update query with SeaQuery
        let mut update_query = Query::update();
        let mut update_stmt = update_query.table(Suppliers::Table);

        // Only set fields that are provided in the update input
        if let Some(name) = &self.supplier.name {
            update_stmt = update_stmt.value(Suppliers::Name, name.clone());
        }

        if let Some(address) = &self.supplier.address {
            match address {
                Some(addr) => update_stmt = update_stmt.value(Suppliers::Address, addr.clone()),
                None => update_stmt = update_stmt.value(Suppliers::Address, sea_query::Value::String(None)),
            };
        }

        if let Some(phone) = &self.supplier.phone {
            match phone {
                Some(p) => update_stmt = update_stmt.value(Suppliers::Phone, p.clone()),
                None => update_stmt = update_stmt.value(Suppliers::Phone, sea_query::Value::String(None)),
            };
        }

        // Always update the updated_at timestamp
        update_stmt = update_stmt.value(Suppliers::UpdatedAt, now.to_string());

        // Add the WHERE clause
        update_stmt = update_stmt.and_where(Expr::col(Suppliers::Id).eq(supplier_id.to_string()));

        // Execute the query
        service.db_adapter.update_many(&update_stmt).await?;

        // Get the updated supplier
        let updated_supplier = service.db_adapter.query_one::<Supplier>(&check_stmt).await?;

        Ok(updated_supplier)
    }
}

impl Command for DeleteSupplierCommand {
    type Output = i32;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Build the delete query with SeaQuery
        let mut delete_query = Query::delete();
        let delete_stmt = delete_query
            .from_table(Suppliers::Table)
            .and_where(Expr::col(Suppliers::Id).eq(self.id.to_string()));

        // Execute the query
        let affected_rows = service.db_adapter.delete(&delete_stmt).await?;

        Ok(affected_rows as i32)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::commands::tests::setup_service;

    use super::*;
    use sea_query::{Alias, Expr, Query};
    use tokio;

    #[tokio::test]
    async fn test_create_supplier() {
        let mut service = setup_service();

        let command = CreateSupplierCommand {
            supplier: SupplierNewInput {
                name: "Test Supplier".to_string(),
                address: Some("123 Test St".to_string()),
                phone: Some("123-456-7890".to_string()),
            },
        };

        let supplier = command.exec(&mut service).await.unwrap();
        assert_eq!(supplier.name, "Test Supplier");
        assert_eq!(supplier.address, Some("123 Test St".to_string()));
        assert_eq!(supplier.phone, Some("123-456-7890".to_string()));
    }

    #[tokio::test]
    async fn test_create_supplier_minimal() {
        let mut service = setup_service();

        let command = CreateSupplierCommand {
            supplier: SupplierNewInput {
                name: "Test Supplier".to_string(),
                address: None,
                phone: None,
            },
        };

        let supplier = command.exec(&mut service).await.unwrap();
        assert_eq!(supplier.name, "Test Supplier");
        assert_eq!(supplier.address, None);
        assert_eq!(supplier.phone, None);
    }

    #[tokio::test]
    async fn test_update_supplier() {
        let mut service = setup_service();

        // Create supplier
        let create_command = CreateSupplierCommand {
            supplier: SupplierNewInput {
                name: "Test Supplier".to_string(),
                address: Some("123 Test St".to_string()),
                phone: Some("123-456-7890".to_string()),
            },
        };

        let supplier = create_command.exec(&mut service).await.unwrap();

        // Update supplier
        let update_command = UpdateSupplierCommand {
            supplier: SupplierUpdateInput {
                id: supplier.id,
                name: Some("Updated Supplier".to_string()),
                address: Some(Some("456 New St".to_string())),
                phone: None,
            },
        };

        let updated_supplier = update_command.exec(&mut service).await.unwrap();
        assert_eq!(updated_supplier.name, "Updated Supplier");
        assert_eq!(updated_supplier.address, Some("456 New St".to_string()));
        assert_eq!(updated_supplier.phone, Some("123-456-7890".to_string())); // Unchanged
    }

    #[tokio::test]
    async fn test_update_supplier_remove_field() {
        let mut service = setup_service();

        // Create supplier
        let create_command = CreateSupplierCommand {
            supplier: SupplierNewInput {
                name: "Test Supplier".to_string(),
                address: Some("123 Test St".to_string()),
                phone: Some("123-456-7890".to_string()),
            },
        };

        let supplier = create_command.exec(&mut service).await.unwrap();

        // Update supplier - remove address
        let update_command = UpdateSupplierCommand {
            supplier: SupplierUpdateInput {
                id: supplier.id,
                name: None,
                address: Some(None), // Remove address
                phone: None,
            },
        };

        let updated_supplier = update_command.exec(&mut service).await.unwrap();
        assert_eq!(updated_supplier.name, "Test Supplier"); // Unchanged
        assert_eq!(updated_supplier.address, None); // Removed
        assert_eq!(updated_supplier.phone, Some("123-456-7890".to_string())); // Unchanged
    }

    #[tokio::test]
    async fn test_update_nonexistent_supplier() {
        let mut service = setup_service();

        let update_command = UpdateSupplierCommand {
            supplier: SupplierUpdateInput {
                id: Uuid::now_v7().into(),
                name: Some("Updated Supplier".to_string()),
                address: None,
                phone: None,
            },
        };

        let result = update_command.exec(&mut service).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_supplier() {
        let mut service = setup_service();

        // Create supplier
        let create_command = CreateSupplierCommand {
            supplier: SupplierNewInput {
                name: "Test Supplier".to_string(),
                address: None,
                phone: None,
            },
        };

        let supplier = create_command.exec(&mut service).await.unwrap();

        // Delete supplier
        let delete_command = DeleteSupplierCommand { id: supplier.id };
        let result = delete_command.exec(&mut service).await.unwrap();
        assert_eq!(result, 1);

        // Verify supplier no longer exists
        let mut count_query = Query::select();
        let count_stmt = count_query
            .from(Suppliers::Table)
            .expr_as(Expr::col(Suppliers::Id).count(), Alias::new("count"))
            .and_where(Expr::col(Suppliers::Id).eq(supplier.id.to_string()));

        let count = service.db_adapter.query_one::<i64>(&count_stmt).await.unwrap();
        assert_eq!(count, 0);
    }

    #[tokio::test]
    async fn test_delete_nonexistent_supplier() {
        let mut service = setup_service();

        // Delete non-existent supplier
        let delete_command = DeleteSupplierCommand {
            id: Uuid::now_v7().into(),
        };
        let result = delete_command.exec(&mut service).await.unwrap();

        // This should return 0 rows affected, not an error
        assert_eq!(result, 0);
    }
}
