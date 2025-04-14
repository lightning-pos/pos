use chrono::Utc;
use sea_query::{Expr, Query, SqliteQueryBuilder, Value};
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

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.db_adapter.transaction(|db| {
            let now = Utc::now().naive_utc();
            let new_variant_type = VariantType {
                id: Uuid::now_v7().into(),
                name: self.variant_type.name.clone(),
                description: self.variant_type.description.clone(),
                created_at: now,
                updated_at: now,
            };

            let query = Query::insert()
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
                ])
                .to_string(SqliteQueryBuilder);

            db.execute(&query, vec![])?;

            Ok(new_variant_type)
        })
    }
}

impl Command for UpdateVariantTypeCommand {
    type Output = VariantType;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.db_adapter.transaction(|db| {
            // Verify variant type exists
            let query = Query::select()
                .from(VariantTypes::Table)
                .columns([
                    VariantTypes::Id,
                    VariantTypes::Name,
                    VariantTypes::Description,
                    VariantTypes::CreatedAt,
                    VariantTypes::UpdatedAt,
                ])
                .and_where(Expr::col(VariantTypes::Id).eq(self.variant_type.id.to_string()))
                .to_string(SqliteQueryBuilder);

            let existing_type = db.query_optional::<VariantType>(&query, vec![])?;
            if existing_type.is_none() {
                return Err(Error::NotFoundError);
            }
            let existing_type = existing_type.unwrap();

            let now = Utc::now().naive_utc();

            // Build update query
            let mut update_query = Query::update();
            let update = update_query
                .table(VariantTypes::Table)
                .and_where(Expr::col(VariantTypes::Id).eq(self.variant_type.id.to_string()))
                .value(VariantTypes::UpdatedAt, now.to_string());

            if let Some(name) = &self.variant_type.name {
                update.value(VariantTypes::Name, name.clone());
            }

            if let Some(description) = &self.variant_type.description {
                match description {
                    Some(desc) => update.value(VariantTypes::Description, desc.clone()),
                    None => update.value(VariantTypes::Description, Value::String(None)),
                };
            }

            let sql = update.to_string(SqliteQueryBuilder);
            db.execute(&sql, vec![])?;

            // Fetch the updated record
            let query = Query::select()
                .from(VariantTypes::Table)
                .columns([
                    VariantTypes::Id,
                    VariantTypes::Name,
                    VariantTypes::Description,
                    VariantTypes::CreatedAt,
                    VariantTypes::UpdatedAt,
                ])
                .and_where(Expr::col(VariantTypes::Id).eq(self.variant_type.id.to_string()))
                .to_string(SqliteQueryBuilder);

            let updated_type = db.query_one::<VariantType>(&query, vec![])?;
            Ok(updated_type)
        })
    }
}

impl Command for DeleteVariantTypeCommand {
    type Output = usize;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.db_adapter.transaction(|db| {
            // Check if there are any variant values using this type
            let count_query = Query::select()
                .from(crate::core::models::catalog::variant_value_model::VariantValues::Table)
                .expr(Expr::count(Expr::col(crate::core::models::catalog::variant_value_model::VariantValues::Id)))
                .and_where(Expr::col(crate::core::models::catalog::variant_value_model::VariantValues::VariantTypeId).eq(self.id.to_string()))
                .to_string(SqliteQueryBuilder);

            let count: i64 = db.query_one(&count_query, vec![])?;

            if count > 0 {
                return Err(Error::HasChildrenError);
            }

            // Delete the variant type
            let delete_query = Query::delete()
                .from_table(VariantTypes::Table)
                .and_where(Expr::col(VariantTypes::Id).eq(self.id.to_string()))
                .to_string(SqliteQueryBuilder);

            let num_deleted = db.execute(&delete_query, vec![])?;

            if num_deleted == 0 {
                Err(Error::NotFoundError)
            } else {
                Ok(num_deleted as usize)
            }
        })
    }
}

impl Command for GetVariantTypeCommand {
    type Output = VariantType;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.db_adapter.transaction(|db| {
            let query = Query::select()
                .from(VariantTypes::Table)
                .columns([
                    VariantTypes::Id,
                    VariantTypes::Name,
                    VariantTypes::Description,
                    VariantTypes::CreatedAt,
                    VariantTypes::UpdatedAt,
                ])
                .and_where(Expr::col(VariantTypes::Id).eq(self.id.to_string()))
                .to_string(SqliteQueryBuilder);

            let variant_type = db.query_optional::<VariantType>(&query, vec![])?;
            match variant_type {
                Some(vt) => Ok(vt),
                None => Err(Error::NotFoundError),
            }
        })
    }
}

impl Command for ListVariantTypesCommand {
    type Output = Vec<VariantType>;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.db_adapter.transaction(|db| {
            let query = Query::select()
                .from(VariantTypes::Table)
                .columns([
                    VariantTypes::Id,
                    VariantTypes::Name,
                    VariantTypes::Description,
                    VariantTypes::CreatedAt,
                    VariantTypes::UpdatedAt,
                ])
                .to_string(SqliteQueryBuilder);

            let results = db.query_many::<VariantType>(&query, vec![])?;
            Ok(results)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::commands::app_service::tests::setup_service;

    fn create_test_variant_type(service: &mut AppService) -> VariantType {
        let command = CreateVariantTypeCommand {
            variant_type: VariantTypeNewInput {
                name: "Test Variant Type".to_string(),
                description: Some("Test Description".to_string()),
            },
        };
        command.exec(service).unwrap()
    }

    #[test]
    fn test_create_variant_type() {
        let mut service = setup_service();
        let variant_type = create_test_variant_type(&mut service);
        assert_eq!(variant_type.name, "Test Variant Type");
        assert_eq!(variant_type.description, Some("Test Description".to_string()));
    }

    #[test]
    fn test_update_variant_type() {
        let mut service = setup_service();
        let created = create_test_variant_type(&mut service);

        let update_command = UpdateVariantTypeCommand {
            variant_type: VariantTypeUpdateInput {
                id: created.id,
                name: Some("Updated Name".to_string()),
                description: Some(Some("Updated Description".to_string())),
                updated_at: None,
            },
        };

        let updated = update_command.exec(&mut service).unwrap();
        assert_eq!(updated.name, "Updated Name");
        assert_eq!(updated.description, Some("Updated Description".to_string()));
    }

    #[test]
    fn test_get_variant_type() {
        let mut service = setup_service();
        let created = create_test_variant_type(&mut service);

        let get_command = GetVariantTypeCommand { id: created.id };
        let retrieved = get_command.exec(&mut service).unwrap();
        assert_eq!(retrieved.id, created.id);
        assert_eq!(retrieved.name, created.name);
    }

    #[test]
    fn test_list_variant_types() {
        let mut service = setup_service();
        let created = create_test_variant_type(&mut service);

        let list_command = ListVariantTypesCommand;
        let list = list_command.exec(&mut service).unwrap();
        assert!(!list.is_empty());
        assert!(list.iter().any(|vt| vt.id == created.id));
    }

    #[test]
    fn test_delete_variant_type() {
        let mut service = setup_service();
        let created = create_test_variant_type(&mut service);

        let delete_command = DeleteVariantTypeCommand { id: created.id };
        let result = delete_command.exec(&mut service).unwrap();
        assert_eq!(result, 1);

        // Verify it's gone
        let get_command = GetVariantTypeCommand { id: created.id };
        let result = get_command.exec(&mut service);
        assert!(result.is_err());
    }
}
