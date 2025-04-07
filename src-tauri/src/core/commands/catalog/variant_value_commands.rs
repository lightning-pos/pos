use chrono::Utc;
use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::{
    core::{
        commands::{app_service::AppService, Command},
        models::catalog::variant_value_model::{
            VariantValue, VariantValueNewInput, VariantValueUpdateInput,
        },
        types::db_uuid::DbUuid,
    },
    error::{Error, Result},
    schema::{variant_types, variant_values},
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

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            // Verify variant type exists
            variant_types::table
                .find(&self.variant_value.variant_type_id)
                .select(crate::core::models::catalog::variant_type_model::VariantType::as_select())
                .get_result::<crate::core::models::catalog::variant_type_model::VariantType>(
                    conn,
                )?;

            // Get the max display order for this variant type by querying all values and finding the max
            let values = variant_values::table
                .filter(variant_values::variant_type_id.eq(&self.variant_value.variant_type_id))
                .select(VariantValue::as_select())
                .load::<VariantValue>(conn)?;

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

            let res = diesel::insert_into(variant_values::table)
                .values(&new_variant_value)
                .returning(VariantValue::as_returning())
                .get_result(conn)?;

            Ok(res)
        })
    }
}

impl Command for UpdateVariantValueCommand {
    type Output = VariantValue;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            // Verify variant value exists
            variant_values::table
                .find(&self.variant_value.id)
                .select(VariantValue::as_select())
                .get_result::<VariantValue>(conn)?;

            let now = Utc::now().naive_utc();

            let mut variant_value = self.variant_value.clone();
            variant_value.updated_at = Some(now);

            let res = diesel::update(variant_values::table.find(&self.variant_value.id))
                .set(&variant_value)
                .returning(VariantValue::as_returning())
                .get_result(conn)?;

            Ok(res)
        })
    }
}

impl Command for DeleteVariantValueCommand {
    type Output = usize;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            // Check if this variant value is used in any item variants
            let count: i64 = crate::schema::item_variant_values::table
                .filter(crate::schema::item_variant_values::variant_value_id.eq(&self.id))
                .count()
                .get_result(conn)?;

            if count > 0 {
                return Err(Error::HasChildrenError);
            }

            let num_deleted =
                diesel::delete(variant_values::table.filter(variant_values::id.eq(&self.id)))
                    .execute(conn)?;

            if num_deleted == 0 {
                Err(Error::NotFoundError)
            } else {
                Ok(num_deleted)
            }
        })
    }
}

impl Command for GetVariantValueCommand {
    type Output = VariantValue;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let variant_value = variant_values::table
                .find(&self.id)
                .select(VariantValue::as_select())
                .first::<VariantValue>(conn)
                .map_err(|_| Error::NotFoundError)?;
            Ok(variant_value)
        })
    }
}

impl Command for ListVariantValuesCommand {
    type Output = Vec<VariantValue>;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let mut query = variant_values::table.into_boxed();

            if let Some(type_id) = self.variant_type_id {
                query = query.filter(variant_values::variant_type_id.eq(type_id));
            }

            query = query.order_by(variant_values::display_order.asc());

            let results = query
                .select(VariantValue::as_select())
                .load::<VariantValue>(conn)?;
            Ok(results)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::commands::app_service::tests::setup_service;
    use crate::core::commands::catalog::variant_type_commands::CreateVariantTypeCommand;
    use crate::core::models::catalog::variant_type_model::VariantTypeNewInput;

    fn create_test_variant_type(
        service: &mut AppService,
    ) -> crate::core::models::catalog::variant_type_model::VariantType {
        let command = CreateVariantTypeCommand {
            variant_type: VariantTypeNewInput {
                name: "Test Variant Type".to_string(),
                description: Some("Test Description".to_string()),
            },
        };
        command.exec(service).unwrap()
    }

    fn create_test_variant_value(service: &mut AppService) -> VariantValue {
        let variant_type = create_test_variant_type(service);
        let command = CreateVariantValueCommand {
            variant_value: VariantValueNewInput {
                variant_type_id: variant_type.id,
                value: "Test Value".to_string(),
                display_order: Some(1),
            },
        };
        command.exec(service).unwrap()
    }

    #[test]
    fn test_create_variant_value() {
        let mut service = setup_service();
        let variant_type = create_test_variant_type(&mut service);

        let command = CreateVariantValueCommand {
            variant_value: VariantValueNewInput {
                variant_type_id: variant_type.id,
                value: "Test Value".to_string(),
                display_order: Some(1),
            },
        };

        let variant_value = command.exec(&mut service).unwrap();
        assert_eq!(variant_value.value, "Test Value");
        assert_eq!(variant_value.display_order, 1);
        assert_eq!(variant_value.variant_type_id, variant_type.id);
    }

    #[test]
    fn test_update_variant_value() {
        let mut service = setup_service();
        let created = create_test_variant_value(&mut service);

        let update_command = UpdateVariantValueCommand {
            variant_value: VariantValueUpdateInput {
                id: created.id,
                value: Some("Updated Value".to_string()),
                display_order: Some(2),
                updated_at: None,
            },
        };

        let updated = update_command.exec(&mut service).unwrap();
        assert_eq!(updated.value, "Updated Value");
        assert_eq!(updated.display_order, 2);
    }

    #[test]
    fn test_get_variant_value() {
        let mut service = setup_service();
        let created = create_test_variant_value(&mut service);

        let get_command = GetVariantValueCommand { id: created.id };
        let retrieved = get_command.exec(&mut service).unwrap();
        assert_eq!(retrieved.id, created.id);
        assert_eq!(retrieved.value, created.value);
    }

    #[test]
    fn test_list_variant_values() {
        let mut service = setup_service();
        let variant_type = create_test_variant_type(&mut service);

        // Create multiple values
        let command1 = CreateVariantValueCommand {
            variant_value: VariantValueNewInput {
                variant_type_id: variant_type.id,
                value: "Value 1".to_string(),
                display_order: Some(1),
            },
        };
        let value1 = command1.exec(&mut service).unwrap();

        let command2 = CreateVariantValueCommand {
            variant_value: VariantValueNewInput {
                variant_type_id: variant_type.id,
                value: "Value 2".to_string(),
                display_order: Some(2),
            },
        };
        let value2 = command2.exec(&mut service).unwrap();

        // List all values
        let list_command = ListVariantValuesCommand {
            variant_type_id: None,
        };
        let all_values = list_command.exec(&mut service).unwrap();
        assert!(all_values.len() >= 2);

        // List values for specific type
        let list_command = ListVariantValuesCommand {
            variant_type_id: Some(variant_type.id),
        };
        let type_values = list_command.exec(&mut service).unwrap();
        assert_eq!(type_values.len(), 2);
        assert!(type_values.iter().any(|v| v.id == value1.id));
        assert!(type_values.iter().any(|v| v.id == value2.id));
    }

    #[test]
    fn test_delete_variant_value() {
        let mut service = setup_service();
        let created = create_test_variant_value(&mut service);

        let delete_command = DeleteVariantValueCommand { id: created.id };
        let result = delete_command.exec(&mut service).unwrap();
        assert_eq!(result, 1);

        // Verify it's gone
        let get_command = GetVariantValueCommand { id: created.id };
        let result = get_command.exec(&mut service);
        assert!(result.is_err());
    }
}
