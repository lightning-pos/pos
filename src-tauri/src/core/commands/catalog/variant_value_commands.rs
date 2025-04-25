use chrono::Utc;
use sea_query::{Expr, Query};
use uuid::Uuid;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        commands::{app_service::AppService, Command},
        models::catalog::variant_type_model::VariantTypes,
        models::catalog::variant_value_model::{
            VariantValue, VariantValueNewInput, VariantValueUpdateInput, VariantValues,
        },
        types::db_uuid::DbUuid,
    },
    error::{Error, Result},
};

// Commands
pub struct CreateVariantValueCommand {
    pub variant_value: VariantValueNewInput,
}

pub struct UpdateVariantValueCommand {
    pub variant_value: VariantValueUpdateInput,
}

pub struct DeleteVariantValueCommand {
    pub id: DbUuid,
}

pub struct GetVariantValueCommand {
    pub id: DbUuid,
}

pub struct ListVariantValuesCommand {
    pub variant_type_id: Option<DbUuid>,
}

// Command Implementations
impl Command for CreateVariantValueCommand {
    type Output = VariantValue;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Verify variant type exists
        let mut type_query = Query::select();
        let type_stmt = type_query
            .from(VariantTypes::Table)
            .columns([
                VariantTypes::Id,
            ])
            .and_where(Expr::col(VariantTypes::Id).eq(self.variant_value.variant_type_id.to_string()));

        let variant_type = service.db_adapter.query_optional::<crate::core::models::catalog::variant_type_model::VariantType>(&type_stmt).await?;
        if variant_type.is_none() {
            return Err(Error::NotFoundError);
        }

        // Get the max display order for this variant type by querying all values and finding the max
        let mut values_query = Query::select();
        let values_stmt = values_query
            .from(VariantValues::Table)
            .columns([
                VariantValues::Id,
                VariantValues::VariantTypeId,
                VariantValues::Value,
                VariantValues::DisplayOrder,
                VariantValues::CreatedAt,
                VariantValues::UpdatedAt,
            ])
            .and_where(Expr::col(VariantValues::VariantTypeId).eq(self.variant_value.variant_type_id.to_string()));

        let values = service.db_adapter.query_many::<VariantValue>(&values_stmt).await?;

        let max_order = values.iter().map(|v| v.display_order).max().unwrap_or(0);

        let display_order = self.variant_value.display_order.unwrap_or(max_order + 1);

        let now = Utc::now().naive_utc();
        let new_variant_value = VariantValue {
            id: Uuid::now_v7().into(),
            variant_type_id: self.variant_value.variant_type_id,
            value: self.variant_value.value.clone(),
            display_order,
            created_at: now,
            updated_at: now,
        };

        let mut insert_query = Query::insert();
        let insert_stmt = insert_query
            .into_table(VariantValues::Table)
            .columns([
                VariantValues::Id,
                VariantValues::VariantTypeId,
                VariantValues::Value,
                VariantValues::DisplayOrder,
                VariantValues::CreatedAt,
                VariantValues::UpdatedAt,
            ])
            .values_panic([
                new_variant_value.id.to_string().into(),
                new_variant_value.variant_type_id.to_string().into(),
                new_variant_value.value.clone().into(),
                new_variant_value.display_order.to_string().into(),
                new_variant_value.created_at.to_string().into(),
                new_variant_value.updated_at.to_string().into(),
            ]);

        service.db_adapter.insert_many(&insert_stmt).await?;

        Ok(new_variant_value)
    }
}

impl Command for UpdateVariantValueCommand {
    type Output = VariantValue;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Verify variant value exists
        let mut select_query = Query::select();
        let select_stmt = select_query
            .from(VariantValues::Table)
            .columns([
                VariantValues::Id,
                VariantValues::VariantTypeId,
                VariantValues::Value,
                VariantValues::DisplayOrder,
                VariantValues::CreatedAt,
                VariantValues::UpdatedAt,
            ])
            .and_where(Expr::col(VariantValues::Id).eq(self.variant_value.id.to_string()));

        let existing_value = service.db_adapter.query_optional::<VariantValue>(&select_stmt).await?;
        if existing_value.is_none() {
            return Err(Error::NotFoundError);
        }
        let _existing_value = existing_value.unwrap();

        let now = Utc::now().naive_utc();

        // Build update query
        let mut update_query = Query::update();
        let mut update_stmt = update_query
            .table(VariantValues::Table)
            .and_where(Expr::col(VariantValues::Id).eq(self.variant_value.id.to_string()))
            .value(VariantValues::UpdatedAt, now.to_string());

        if let Some(value) = &self.variant_value.value {
            update_stmt = update_stmt.value(VariantValues::Value, value.clone());
        }

        if let Some(display_order) = self.variant_value.display_order {
            update_stmt = update_stmt.value(VariantValues::DisplayOrder, display_order.to_string());
        }

        service.db_adapter.update_many(&update_stmt).await?;

        // Fetch the updated record
        let mut query = Query::select();
        let query_stmt = query
            .from(VariantValues::Table)
            .columns([
                VariantValues::Id,
                VariantValues::VariantTypeId,
                VariantValues::Value,
                VariantValues::DisplayOrder,
                VariantValues::CreatedAt,
                VariantValues::UpdatedAt,
            ])
            .and_where(Expr::col(VariantValues::Id).eq(self.variant_value.id.to_string()));

        let updated_value = service.db_adapter.query_one::<VariantValue>(&query_stmt).await?;
        Ok(updated_value)
    }
}

impl Command for DeleteVariantValueCommand {
    type Output = usize;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Check if this variant value is used in any item variants
        let mut count_query = Query::select();
        let count_stmt = count_query
            .from(crate::core::models::catalog::item_variant_value_model::ItemVariantValues::Table)
            .expr(Expr::count(Expr::col(crate::core::models::catalog::item_variant_value_model::ItemVariantValues::ItemVariantId)))
            .and_where(Expr::col(crate::core::models::catalog::item_variant_value_model::ItemVariantValues::VariantValueId).eq(self.id.to_string()));

        let count: i64 = service.db_adapter.query_one(&count_stmt).await?;

        if count > 0 {
            return Err(Error::HasChildrenError);
        }

        // Delete the variant value
        let mut delete_query = Query::delete();
        let delete_stmt = delete_query
            .from_table(VariantValues::Table)
            .and_where(Expr::col(VariantValues::Id).eq(self.id.to_string()));

        let num_deleted = service.db_adapter.delete(&delete_stmt).await?;

        if num_deleted == 0 {
            Err(Error::NotFoundError)
        } else {
            Ok(num_deleted as usize)
        }
    }
}

impl Command for GetVariantValueCommand {
    type Output = VariantValue;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let mut query = Query::select();
        let stmt = query
            .from(VariantValues::Table)
            .columns([
                VariantValues::Id,
                VariantValues::VariantTypeId,
                VariantValues::Value,
                VariantValues::DisplayOrder,
                VariantValues::CreatedAt,
                VariantValues::UpdatedAt,
            ])
            .and_where(Expr::col(VariantValues::Id).eq(self.id.to_string()));

        let variant_value = service.db_adapter.query_optional::<VariantValue>(&stmt).await?;
        match variant_value {
            Some(vv) => Ok(vv),
            None => Err(Error::NotFoundError),
        }
    }
}

impl Command for ListVariantValuesCommand {
    type Output = Vec<VariantValue>;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let mut query_builder = Query::select();
        let mut query = query_builder
            .from(VariantValues::Table)
            .columns([
                VariantValues::Id,
                VariantValues::VariantTypeId,
                VariantValues::Value,
                VariantValues::DisplayOrder,
                VariantValues::CreatedAt,
                VariantValues::UpdatedAt,
            ]);

        if let Some(type_id) = &self.variant_type_id {
            query = query.and_where(Expr::col(VariantValues::VariantTypeId).eq(type_id.to_string()));
        }

        // Order by display_order
        query = query.order_by(VariantValues::DisplayOrder, sea_query::Order::Asc);

        let results = service.db_adapter.query_many::<VariantValue>(&query).await?;
        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::commands::app_service::tests::setup_service;
    use crate::core::commands::catalog::variant_type_commands::CreateVariantTypeCommand;
    use crate::core::models::catalog::variant_type_model::VariantTypeNewInput;
    use tokio;

    async fn create_test_variant_type(
        service: &mut AppService,
    ) -> crate::core::models::catalog::variant_type_model::VariantType {
        let command = CreateVariantTypeCommand {
            variant_type: VariantTypeNewInput {
                name: "Test Variant Type".to_string(),
                description: Some("Test Description".to_string()),
            },
        };
        command.exec(service).await.unwrap()
    }

    async fn create_test_variant_value(service: &mut AppService) -> VariantValue {
        let variant_type = create_test_variant_type(service).await;
        let command = CreateVariantValueCommand {
            variant_value: VariantValueNewInput {
                variant_type_id: variant_type.id,
                value: "Test Value".to_string(),
                display_order: Some(1),
            },
        };
        command.exec(service).await.unwrap()
    }

    #[tokio::test]
    async fn test_create_variant_value() {
        let mut service = setup_service().await;
        let variant_type = create_test_variant_type(&mut service).await;

        let command = CreateVariantValueCommand {
            variant_value: VariantValueNewInput {
                variant_type_id: variant_type.id,
                value: "Test Value".to_string(),
                display_order: Some(1),
            },
        };

        let variant_value = command.exec(&mut service).await.unwrap();
        assert_eq!(variant_value.value, "Test Value");
        assert_eq!(variant_value.display_order, 1);
        assert_eq!(variant_value.variant_type_id, variant_type.id);
    }

    #[tokio::test]
    async fn test_update_variant_value() {
        let mut service = setup_service().await;
        let created = create_test_variant_value(&mut service).await;

        let update_command = UpdateVariantValueCommand {
            variant_value: VariantValueUpdateInput {
                id: created.id,
                value: Some("Updated Value".to_string()),
                display_order: Some(2),
                updated_at: None,
            },
        };

        let updated = update_command.exec(&mut service).await.unwrap();
        assert_eq!(updated.value, "Updated Value");
        assert_eq!(updated.display_order, 2);
    }

    #[tokio::test]
    async fn test_get_variant_value() {
        let mut service = setup_service().await;
        let created = create_test_variant_value(&mut service).await;

        let get_command = GetVariantValueCommand { id: created.id };
        let retrieved = get_command.exec(&mut service).await.unwrap();
        assert_eq!(retrieved.id, created.id);
        assert_eq!(retrieved.value, created.value);
    }

    #[tokio::test]
    async fn test_list_variant_values() {
        let mut service = setup_service().await;
        let variant_type = create_test_variant_type(&mut service).await;

        // Create multiple values
        let command1 = CreateVariantValueCommand {
            variant_value: VariantValueNewInput {
                variant_type_id: variant_type.id,
                value: "Value 1".to_string(),
                display_order: Some(1),
            },
        };
        let value1 = command1.exec(&mut service).await.unwrap();

        let command2 = CreateVariantValueCommand {
            variant_value: VariantValueNewInput {
                variant_type_id: variant_type.id,
                value: "Value 2".to_string(),
                display_order: Some(2),
            },
        };
        let value2 = command2.exec(&mut service).await.unwrap();

        // List all values
        let list_command = ListVariantValuesCommand {
            variant_type_id: None,
        };
        let all_values = list_command.exec(&mut service).await.unwrap();
        assert!(all_values.len() >= 2);

        // List values for specific type
        let list_command = ListVariantValuesCommand {
            variant_type_id: Some(variant_type.id),
        };
        let type_values = list_command.exec(&mut service).await.unwrap();
        assert_eq!(type_values.len(), 2);
        assert!(type_values.iter().any(|v| v.id == value1.id));
        assert!(type_values.iter().any(|v| v.id == value2.id));
    }

    #[tokio::test]
    async fn test_delete_variant_value() {
        let mut service = setup_service().await;
        let created = create_test_variant_value(&mut service).await;

        let delete_command = DeleteVariantValueCommand { id: created.id };
        let result = delete_command.exec(&mut service).await.unwrap();
        assert_eq!(result, 1);

        // Verify it's gone
        let get_command = GetVariantValueCommand { id: created.id };
        let result = get_command.exec(&mut service).await;
        assert!(result.is_err());
    }
}
