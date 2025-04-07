use chrono::Utc;
use diesel::{
    Connection, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper,
};
use uuid::Uuid;

use crate::{
    core::{
        commands::{app_service::AppService, Command},
        models::catalog::{
            item_model::Item,
            item_variant_model::{ItemVariant, ItemVariantNewInput, ItemVariantUpdateInput},
            item_variant_value_model::ItemVariantValue,
            variant_value_model::VariantValue,
        },
        types::db_uuid::DbUuid,
    },
    error::{Error, Result},
    schema::{item_variant_values, item_variants, items, variant_values},
};

// Commands
pub struct CreateItemVariantCommand {
    pub item_variant: ItemVariantNewInput,
}

pub struct UpdateItemVariantCommand {
    pub item_variant: ItemVariantUpdateInput,
}

pub struct DeleteItemVariantCommand {
    pub id: DbUuid,
}

pub struct GetItemVariantCommand {
    pub id: DbUuid,
}

pub struct ListItemVariantsCommand {
    pub item_id: Option<DbUuid>,
}

pub struct AssignVariantValueCommand {
    pub item_variant_id: DbUuid,
    pub variant_value_id: DbUuid,
}

pub struct RemoveVariantValueCommand {
    pub item_variant_id: DbUuid,
    pub variant_value_id: DbUuid,
}

// Command Implementations
impl Command for CreateItemVariantCommand {
    type Output = ItemVariant;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            // Verify item exists
            items::table
                .find(&self.item_variant.item_id)
                .select(Item::as_select())
                .get_result::<Item>(conn)?;

            // Verify all variant values exist
            for variant_value_id in &self.item_variant.variant_value_ids {
                variant_values::table
                    .find(variant_value_id)
                    .select(VariantValue::as_select())
                    .get_result::<VariantValue>(conn)?;
            }

            // Check if this is the first variant for this item
            let is_first: i64 = item_variants::table
                .filter(item_variants::item_id.eq(&self.item_variant.item_id))
                .count()
                .get_result(conn)?;

            // If this is the first variant and is_default is not specified, make it default
            let is_default = self.item_variant.is_default.unwrap_or(is_first == 0);

            // If this variant is set as default, unset any existing default
            if is_default {
                diesel::update(item_variants::table)
                    .filter(item_variants::item_id.eq(&self.item_variant.item_id))
                    .filter(item_variants::is_default.eq(true))
                    .set(item_variants::is_default.eq(false))
                    .execute(conn)?;
            }

            let now = Utc::now().naive_utc();
            let new_item_variant = ItemVariant {
                id: Uuid::now_v7().into(),
                item_id: self.item_variant.item_id,
                sku: self.item_variant.sku.clone(),
                price_adjustment: self.item_variant.price_adjustment,
                is_default,
                created_at: now,
                updated_at: now,
            };

            diesel::insert_into(item_variants::table)
                .values(&new_item_variant)
                .execute(conn)?;

            // Associate variant values with this item variant
            for variant_value_id in &self.item_variant.variant_value_ids {
                let item_variant_value = ItemVariantValue {
                    item_variant_id: new_item_variant.id,
                    variant_value_id: *variant_value_id,
                };

                diesel::insert_into(item_variant_values::table)
                    .values(&item_variant_value)
                    .execute(conn)?;
            }

            Ok(new_item_variant)
        })
    }
}

impl Command for UpdateItemVariantCommand {
    type Output = ItemVariant;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            // Verify item variant exists
            let current_variant = item_variants::table
                .find(&self.item_variant.id)
                .select(ItemVariant::as_select())
                .get_result::<ItemVariant>(conn)?;

            let now = Utc::now().naive_utc();

            let mut item_variant = self.item_variant.clone();
            item_variant.updated_at = Some(now);

            // If setting this variant as default, unset any existing default
            if let Some(is_default) = item_variant.is_default {
                if is_default {
                    diesel::update(item_variants::table)
                        .filter(item_variants::item_id.eq(&current_variant.item_id))
                        .filter(item_variants::id.ne(&self.item_variant.id))
                        .filter(item_variants::is_default.eq(true))
                        .set(item_variants::is_default.eq(false))
                        .execute(conn)?;
                }
            }

            let res = diesel::update(item_variants::table.find(&self.item_variant.id))
                .set(&item_variant)
                .returning(ItemVariant::as_returning())
                .get_result(conn)?;

            Ok(res)
        })
    }
}

impl Command for DeleteItemVariantCommand {
    type Output = usize;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            // Get the variant to check if it's default and get its item_id
            let variant = item_variants::table
                .find(&self.id)
                .select(ItemVariant::as_select())
                .get_result::<ItemVariant>(conn)
                .map_err(|_| Error::NotFoundError)?;

            // Delete associated variant values first
            diesel::delete(item_variant_values::table)
                .filter(item_variant_values::item_variant_id.eq(&self.id))
                .execute(conn)?;

            // Delete the variant
            let num_deleted =
                diesel::delete(item_variants::table.filter(item_variants::id.eq(&self.id)))
                    .execute(conn)?;

            // If this was the default variant, set another one as default if available
            if variant.is_default {
                let another_variant: Option<ItemVariant> = item_variants::table
                    .filter(item_variants::item_id.eq(&variant.item_id))
                    .first::<ItemVariant>(conn)
                    .optional()?;

                if let Some(another) = another_variant {
                    diesel::update(item_variants::table.find(&another.id))
                        .set(item_variants::is_default.eq(true))
                        .execute(conn)?;
                }
            }

            if num_deleted == 0 {
                Err(Error::NotFoundError)
            } else {
                Ok(num_deleted)
            }
        })
    }
}

impl Command for GetItemVariantCommand {
    type Output = ItemVariant;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let item_variant = item_variants::table
                .find(&self.id)
                .select(ItemVariant::as_select())
                .first::<ItemVariant>(conn)
                .map_err(|_| Error::NotFoundError)?;
            Ok(item_variant)
        })
    }
}

impl Command for ListItemVariantsCommand {
    type Output = Vec<ItemVariant>;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let mut query = item_variants::table.into_boxed();

            if let Some(item_id) = self.item_id {
                query = query.filter(item_variants::item_id.eq(item_id));
            }

            let results = query
                .select(ItemVariant::as_select())
                .load::<ItemVariant>(conn)?;
            Ok(results)
        })
    }
}

impl Command for AssignVariantValueCommand {
    type Output = usize;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            // Verify item variant exists
            item_variants::table
                .find(&self.item_variant_id)
                .select(ItemVariant::as_select())
                .get_result::<ItemVariant>(conn)?;

            // Verify variant value exists
            variant_values::table
                .find(&self.variant_value_id)
                .select(VariantValue::as_select())
                .get_result::<VariantValue>(conn)?;

            // Check if the association already exists
            let exists = item_variant_values::table
                .filter(item_variant_values::item_variant_id.eq(&self.item_variant_id))
                .filter(item_variant_values::variant_value_id.eq(&self.variant_value_id))
                .count()
                .get_result::<i64>(conn)?
                > 0;

            if exists {
                return Ok(0); // Already exists, no changes made
            }

            // Create the association
            let item_variant_value = ItemVariantValue {
                item_variant_id: self.item_variant_id,
                variant_value_id: self.variant_value_id,
            };

            diesel::insert_into(item_variant_values::table)
                .values(&item_variant_value)
                .execute(conn)?;

            Ok(1)
        })
    }
}

impl Command for RemoveVariantValueCommand {
    type Output = usize;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let deleted_count = diesel::delete(item_variant_values::table)
                .filter(item_variant_values::item_variant_id.eq(&self.item_variant_id))
                .filter(item_variant_values::variant_value_id.eq(&self.variant_value_id))
                .execute(conn)?;

            Ok(deleted_count)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::commands::app_service::tests::setup_service;
    use crate::core::commands::catalog::{
        item_commands::CreateItemCommand, item_group_commands::CreateItemGroupCommand,
        variant_type_commands::CreateVariantTypeCommand,
        variant_value_commands::CreateVariantValueCommand,
    };
    use crate::core::models::catalog::item_group_model::ItemGroupNew;
    use crate::core::models::catalog::item_model::{ItemNature, ItemState, NewItem};
    use crate::core::models::catalog::variant_type_model::VariantTypeNewInput;
    use crate::core::models::catalog::variant_value_model::VariantValueNewInput;
    use crate::core::types::money::Money;

    fn create_test_item_category(service: &mut AppService) -> DbUuid {
        let command = CreateItemGroupCommand {
            category: ItemGroupNew {
                name: "Test Category".to_string(),
                description: None,
            },
        };
        let category = command.exec(service).unwrap();
        category.id
    }

    fn create_test_item(service: &mut AppService) -> Item {
        let category_id = create_test_item_category(service);
        let command = CreateItemCommand {
            item: NewItem {
                name: "Test Item".to_string(),
                description: None,
                nature: ItemNature::Goods,
                state: ItemState::Active,
                price: Money::from(1000),
                category_id,
                tax_ids: None,
            },
        };
        command.exec(service).unwrap()
    }

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

    fn create_test_variant_value(
        service: &mut AppService,
        variant_type_id: DbUuid,
    ) -> VariantValue {
        let command = CreateVariantValueCommand {
            variant_value: VariantValueNewInput {
                variant_type_id,
                value: "Test Value".to_string(),
                display_order: Some(1),
            },
        };
        command.exec(service).unwrap()
    }

    fn create_test_item_variant(service: &mut AppService) -> ItemVariant {
        let item = create_test_item(service);
        let variant_type = create_test_variant_type(service);
        let variant_value = create_test_variant_value(service, variant_type.id);

        let command = CreateItemVariantCommand {
            item_variant: ItemVariantNewInput {
                item_id: item.id,
                sku: Some("TEST-SKU-001".to_string()),
                price_adjustment: Some(Money::from(100)),
                is_default: Some(true),
                variant_value_ids: vec![variant_value.id],
            },
        };
        command.exec(service).unwrap()
    }

    #[test]
    fn test_create_item_variant() {
        let mut service = setup_service();
        let item = create_test_item(&mut service);
        let variant_type = create_test_variant_type(&mut service);
        let variant_value = create_test_variant_value(&mut service, variant_type.id);

        let command = CreateItemVariantCommand {
            item_variant: ItemVariantNewInput {
                item_id: item.id,
                sku: Some("TEST-SKU-001".to_string()),
                price_adjustment: Some(Money::from(100)),
                is_default: Some(true),
                variant_value_ids: vec![variant_value.id],
            },
        };

        let item_variant = command.exec(&mut service).unwrap();
        assert_eq!(item_variant.item_id, item.id);
        assert_eq!(item_variant.sku, Some("TEST-SKU-001".to_string()));
        assert_eq!(item_variant.price_adjustment, Some(Money::from(100)));
        assert!(item_variant.is_default);

        // Verify variant value association
        let associations = item_variant_values::table
            .filter(item_variant_values::item_variant_id.eq(item_variant.id))
            .load::<ItemVariantValue>(&mut service.conn)
            .unwrap();

        assert_eq!(associations.len(), 1);
        assert_eq!(associations[0].variant_value_id, variant_value.id);
    }

    #[test]
    fn test_update_item_variant() {
        let mut service = setup_service();
        let created = create_test_item_variant(&mut service);

        let update_command = UpdateItemVariantCommand {
            item_variant: ItemVariantUpdateInput {
                id: created.id,
                sku: Some(Some("UPDATED-SKU".to_string())),
                price_adjustment: Some(Some(Money::from(200))),
                is_default: Some(true),
                updated_at: None,
            },
        };

        let updated = update_command.exec(&mut service).unwrap();
        assert_eq!(updated.sku, Some("UPDATED-SKU".to_string()));
        assert_eq!(updated.price_adjustment, Some(Money::from(200)));
        assert!(updated.is_default);
    }

    #[test]
    fn test_get_item_variant() {
        let mut service = setup_service();
        let created = create_test_item_variant(&mut service);

        let get_command = GetItemVariantCommand { id: created.id };
        let retrieved = get_command.exec(&mut service).unwrap();
        assert_eq!(retrieved.id, created.id);
        assert_eq!(retrieved.sku, created.sku);
    }

    #[test]
    fn test_list_item_variants() {
        let mut service = setup_service();
        let item = create_test_item(&mut service);
        let variant_type = create_test_variant_type(&mut service);
        let variant_value1 = create_test_variant_value(&mut service, variant_type.id);
        let variant_value2 = create_test_variant_value(&mut service, variant_type.id);

        // Create multiple variants for the same item
        let command1 = CreateItemVariantCommand {
            item_variant: ItemVariantNewInput {
                item_id: item.id,
                sku: Some("SKU-1".to_string()),
                price_adjustment: Some(Money::from(100)),
                is_default: Some(true),
                variant_value_ids: vec![variant_value1.id],
            },
        };
        let variant1 = command1.exec(&mut service).unwrap();

        let command2 = CreateItemVariantCommand {
            item_variant: ItemVariantNewInput {
                item_id: item.id,
                sku: Some("SKU-2".to_string()),
                price_adjustment: Some(Money::from(200)),
                is_default: Some(false),
                variant_value_ids: vec![variant_value2.id],
            },
        };
        let variant2 = command2.exec(&mut service).unwrap();

        // List all variants
        let list_command = ListItemVariantsCommand { item_id: None };
        let all_variants = list_command.exec(&mut service).unwrap();
        assert!(all_variants.len() >= 2);

        // List variants for specific item
        let list_command = ListItemVariantsCommand {
            item_id: Some(item.id),
        };
        let item_variants = list_command.exec(&mut service).unwrap();
        assert_eq!(item_variants.len(), 2);
        assert!(item_variants.iter().any(|v| v.id == variant1.id));
        assert!(item_variants.iter().any(|v| v.id == variant2.id));
    }

    #[test]
    fn test_delete_item_variant() {
        let mut service = setup_service();
        let created = create_test_item_variant(&mut service);

        let delete_command = DeleteItemVariantCommand { id: created.id };
        let result = delete_command.exec(&mut service).unwrap();
        assert_eq!(result, 1);

        // Verify it's gone
        let get_command = GetItemVariantCommand { id: created.id };
        let result = get_command.exec(&mut service);
        assert!(result.is_err());

        // Verify associations are gone
        let associations = item_variant_values::table
            .filter(item_variant_values::item_variant_id.eq(created.id))
            .load::<ItemVariantValue>(&mut service.conn)
            .unwrap();

        assert_eq!(associations.len(), 0);
    }

    #[test]
    fn test_assign_and_remove_variant_value() {
        let mut service = setup_service();
        let item_variant = create_test_item_variant(&mut service);
        let variant_type = create_test_variant_type(&mut service);
        let new_variant_value = create_test_variant_value(&mut service, variant_type.id);

        // Assign new variant value
        let assign_command = AssignVariantValueCommand {
            item_variant_id: item_variant.id,
            variant_value_id: new_variant_value.id,
        };
        let result = assign_command.exec(&mut service).unwrap();
        assert_eq!(result, 1);

        // Verify association exists
        let associations = item_variant_values::table
            .filter(item_variant_values::item_variant_id.eq(item_variant.id))
            .filter(item_variant_values::variant_value_id.eq(new_variant_value.id))
            .load::<ItemVariantValue>(&mut service.conn)
            .unwrap();

        assert_eq!(associations.len(), 1);

        // Remove variant value
        let remove_command = RemoveVariantValueCommand {
            item_variant_id: item_variant.id,
            variant_value_id: new_variant_value.id,
        };
        let result = remove_command.exec(&mut service).unwrap();
        assert_eq!(result, 1);

        // Verify association is gone
        let associations = item_variant_values::table
            .filter(item_variant_values::item_variant_id.eq(item_variant.id))
            .filter(item_variant_values::variant_value_id.eq(new_variant_value.id))
            .load::<ItemVariantValue>(&mut service.conn)
            .unwrap();

        assert_eq!(associations.len(), 0);
    }
}
