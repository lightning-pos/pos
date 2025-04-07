use chrono::Utc;
use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::{
    core::{
        commands::{app_service::AppService, Command},
        models::catalog::variant_type_model::{VariantType, VariantTypeNewInput, VariantTypeUpdateInput},
        types::db_uuid::DbUuid,
    },
    error::{Error, Result},
    schema::variant_types,
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
        service.conn.transaction(|conn| {
            let now = Utc::now().naive_utc();
            let new_variant_type = VariantType {
                id: Uuid::now_v7().into(),
                name: self.variant_type.name.clone(),
                description: self.variant_type.description.clone(),
                created_at: now,
                updated_at: now,
            };

            let res = diesel::insert_into(variant_types::table)
                .values(&new_variant_type)
                .returning(VariantType::as_returning())
                .get_result(conn)?;

            Ok(res)
        })
    }
}

impl Command for UpdateVariantTypeCommand {
    type Output = VariantType;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            // Verify variant type exists
            variant_types::table
                .find(&self.variant_type.id)
                .select(VariantType::as_select())
                .get_result::<VariantType>(conn)?;

            let now = Utc::now().naive_utc();

            let mut variant_type = self.variant_type.clone();
            variant_type.updated_at = Some(now);

            let res = diesel::update(variant_types::table.find(&self.variant_type.id))
                .set(&variant_type)
                .returning(VariantType::as_returning())
                .get_result(conn)?;

            Ok(res)
        })
    }
}

impl Command for DeleteVariantTypeCommand {
    type Output = usize;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            // Check if there are any variant values using this type
            let count: i64 = crate::schema::variant_values::table
                .filter(crate::schema::variant_values::variant_type_id.eq(&self.id))
                .count()
                .get_result(conn)?;

            if count > 0 {
                return Err(Error::HasChildrenError);
            }

            let num_deleted = diesel::delete(variant_types::table.filter(variant_types::id.eq(&self.id)))
                .execute(conn)?;

            if num_deleted == 0 {
                Err(Error::NotFoundError)
            } else {
                Ok(num_deleted)
            }
        })
    }
}

impl Command for GetVariantTypeCommand {
    type Output = VariantType;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let variant_type = variant_types::table
                .find(&self.id)
                .select(VariantType::as_select())
                .first::<VariantType>(conn)
                .map_err(|_| Error::NotFoundError)?;
            Ok(variant_type)
        })
    }
}

impl Command for ListVariantTypesCommand {
    type Output = Vec<VariantType>;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let results = variant_types::table
                .select(VariantType::as_select())
                .load::<VariantType>(conn)?;
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
