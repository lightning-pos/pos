use chrono::Utc;
use sea_query::{Expr, Query, Value};
use uuid::Uuid;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        commands::{app_service::AppService, Command},
        models::catalog::variant_type_model::{VariantType, VariantTypeNewInput, VariantTypeUpdateInput, VariantTypes},
        types::db_uuid::DbUuid,
    },
    error::{Error, Result},
};

// Commands
pub struct CreateVariantTypeCommand {
    pub variant_type: VariantTypeNewInput,
}

pub struct UpdateVariantTypeCommand {
    pub variant_type: VariantTypeUpdateInput,
}

pub struct DeleteVariantTypeCommand {
    pub id: DbUuid,
}

pub struct GetVariantTypeCommand {
    pub id: DbUuid,
}

pub struct ListVariantTypesCommand;

// Command Implementations
impl Command for CreateVariantTypeCommand {
    type Output = VariantType;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let now = Utc::now().naive_utc();
        let new_variant_type = VariantType {
            id: Uuid::now_v7().into(),
            name: self.variant_type.name.clone(),
            description: self.variant_type.description.clone(),
            created_at: now,
            updated_at: now,
        };

        let mut query = Query::insert();
        let stmt = query
            .into_table(VariantTypes::Table)
            .columns([
                VariantTypes::Id,
                VariantTypes::Name,
                VariantTypes::Description,
                VariantTypes::CreatedAt,
                VariantTypes::UpdatedAt,
            ])
            .values_panic([
                new_variant_type.id.to_string().into(),
                new_variant_type.name.clone().into(),
                new_variant_type.description.clone().into(),
                new_variant_type.created_at.to_string().into(),
                new_variant_type.updated_at.to_string().into(),
            ]);

        service.db_adapter.insert_many(&stmt).await?;

        Ok(new_variant_type)
    }
}

impl Command for UpdateVariantTypeCommand {
    type Output = VariantType;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Verify variant type exists
        let mut select_query = Query::select();
        let select_stmt = select_query
            .from(VariantTypes::Table)
            .columns([
                VariantTypes::Id,
                VariantTypes::Name,
                VariantTypes::Description,
                VariantTypes::CreatedAt,
                VariantTypes::UpdatedAt,
            ])
            .and_where(Expr::col(VariantTypes::Id).eq(self.variant_type.id.to_string()));

        let existing_type = service.db_adapter.query_optional::<VariantType>(&select_stmt).await?;
        if existing_type.is_none() {
            return Err(Error::NotFoundError);
        }

        let now = Utc::now().naive_utc();

        // Build update query
        let mut update_query = Query::update();
        let mut update_stmt = update_query
            .table(VariantTypes::Table)
            .and_where(Expr::col(VariantTypes::Id).eq(self.variant_type.id.to_string()))
            .value(VariantTypes::UpdatedAt, now.to_string());

        if let Some(name) = &self.variant_type.name {
            update_stmt = update_stmt.value(VariantTypes::Name, name.clone());
        }

        if let Some(description) = &self.variant_type.description {
            match description {
                Some(desc) => update_stmt = update_stmt.value(VariantTypes::Description, desc.clone()),
                None => update_stmt = update_stmt.value(VariantTypes::Description, Value::String(None)),
            };
        }

        service.db_adapter.update_many(&update_stmt).await?;

        // Fetch the updated record
        let mut query = Query::select();
        let query_stmt = query
            .from(VariantTypes::Table)
            .columns([
                VariantTypes::Id,
                VariantTypes::Name,
                VariantTypes::Description,
                VariantTypes::CreatedAt,
                VariantTypes::UpdatedAt,
            ])
            .and_where(Expr::col(VariantTypes::Id).eq(self.variant_type.id.to_string()));

        let updated_type = service.db_adapter.query_one::<VariantType>(&query_stmt).await?;
        Ok(updated_type)
    }
}

impl Command for DeleteVariantTypeCommand {
    type Output = usize;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Check if there are any variant values using this type
        let mut count_query = Query::select();
        let count_stmt = count_query
            .from(crate::core::models::catalog::variant_value_model::VariantValues::Table)
            .expr(Expr::count(Expr::col(crate::core::models::catalog::variant_value_model::VariantValues::Id)))
            .and_where(Expr::col(crate::core::models::catalog::variant_value_model::VariantValues::VariantTypeId).eq(self.id.to_string()));

        let count: i64 = service.db_adapter.query_one(&count_stmt).await?;

        if count > 0 {
            return Err(Error::HasChildrenError);
        }

        // Delete the variant type
        let mut delete_query = Query::delete();
        let delete_stmt = delete_query
            .from_table(VariantTypes::Table)
            .and_where(Expr::col(VariantTypes::Id).eq(self.id.to_string()));

        let num_deleted = service.db_adapter.delete(&delete_stmt).await?;

        if num_deleted == 0 {
            Err(Error::NotFoundError)
        } else {
            Ok(num_deleted as usize)
        }
    }
}

impl Command for GetVariantTypeCommand {
    type Output = VariantType;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let mut query = Query::select();
        let stmt = query
            .from(VariantTypes::Table)
            .columns([
                VariantTypes::Id,
                VariantTypes::Name,
                VariantTypes::Description,
                VariantTypes::CreatedAt,
                VariantTypes::UpdatedAt,
            ])
            .and_where(Expr::col(VariantTypes::Id).eq(self.id.to_string()));

        let variant_type = service.db_adapter.query_optional::<VariantType>(&stmt).await?;
        match variant_type {
            Some(vt) => Ok(vt),
            None => Err(Error::NotFoundError),
        }
    }
}

impl Command for ListVariantTypesCommand {
    type Output = Vec<VariantType>;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let mut query = Query::select();
        let stmt = query
            .from(VariantTypes::Table)
            .columns([
                VariantTypes::Id,
                VariantTypes::Name,
                VariantTypes::Description,
                VariantTypes::CreatedAt,
                VariantTypes::UpdatedAt,
            ]);

        let results = service.db_adapter.query_many::<VariantType>(&stmt).await?;
        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::commands::app_service::tests::setup_service;
    use tokio;

    async fn create_test_variant_type(service: &mut AppService) -> VariantType {
        let command = CreateVariantTypeCommand {
            variant_type: VariantTypeNewInput {
                name: "Test Variant Type".to_string(),
                description: Some("Test Description".to_string()),
            },
        };
        command.exec(service).await.unwrap()
    }

    #[tokio::test]
    async fn test_create_variant_type() {
        let mut service = setup_service();
        let variant_type = create_test_variant_type(&mut service).await;
        assert_eq!(variant_type.name, "Test Variant Type");
        assert_eq!(variant_type.description, Some("Test Description".to_string()));
    }

    #[tokio::test]
    async fn test_update_variant_type() {
        let mut service = setup_service();
        let created = create_test_variant_type(&mut service).await;

        let update_command = UpdateVariantTypeCommand {
            variant_type: VariantTypeUpdateInput {
                id: created.id,
                name: Some("Updated Name".to_string()),
                description: Some(Some("Updated Description".to_string())),
                updated_at: None,
            },
        };

        let updated = update_command.exec(&mut service).await.unwrap();
        assert_eq!(updated.name, "Updated Name");
        assert_eq!(updated.description, Some("Updated Description".to_string()));
    }

    #[tokio::test]
    async fn test_get_variant_type() {
        let mut service = setup_service();
        let created = create_test_variant_type(&mut service).await;

        let get_command = GetVariantTypeCommand { id: created.id };
        let retrieved = get_command.exec(&mut service).await.unwrap();
        assert_eq!(retrieved.id, created.id);
        assert_eq!(retrieved.name, created.name);
    }

    #[tokio::test]
    async fn test_list_variant_types() {
        let mut service = setup_service();
        let created = create_test_variant_type(&mut service).await;

        let list_command = ListVariantTypesCommand;
        let list = list_command.exec(&mut service).await.unwrap();
        assert!(!list.is_empty());
        assert!(list.iter().any(|vt| vt.id == created.id));
    }

    #[tokio::test]
    async fn test_delete_variant_type() {
        let mut service = setup_service();
        let created = create_test_variant_type(&mut service).await;

        let delete_command = DeleteVariantTypeCommand { id: created.id };
        let result = delete_command.exec(&mut service).await.unwrap();
        assert_eq!(result, 1);

        // Verify it's gone
        let get_command = GetVariantTypeCommand { id: created.id };
        let result = get_command.exec(&mut service).await;
        assert!(result.is_err());
    }
}
