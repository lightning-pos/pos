use chrono::Utc;
use sea_query::{Expr, Query};
use std::collections::HashSet;
use uuid::Uuid;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        commands::{app_service::AppService, Command},
        models::catalog::{
            item_model::{Item, Items}, item_variant_model::{ItemVariant, ItemVariantNewInput, ItemVariantUpdateInput, ItemVariants}, item_variant_value_model::{ItemVariantValue, ItemVariantValues}, variant_value_model::{VariantValue, VariantValues}
        },
        types::db_uuid::DbUuid,
    }, error::{Error, Result}
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

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Verify item exists
        let mut item_query = Query::select();
        let item_stmt = item_query
            .from(Items::Table)
            .column(Items::Id)
            .and_where(Expr::col(Items::Id).eq(self.item_variant.item_id.to_string()));

        let item = service.db_adapter.query_optional::<DbUuid>(&item_stmt).await?;

        println!("Item: {:?}", item);
        if item.is_none() {
            return Err(Error::NotFoundError);
        }

        // Verify all variant values exist and check for duplicate variant types
        let mut variant_type_ids = HashSet::new();

        for variant_value_id in &self.item_variant.variant_value_ids {
            // Get the variant value and its type
            let mut value_query = Query::select();
            let value_stmt = value_query
                .from(VariantValues::Table)
                .columns([
                    VariantValues::Id,
                    VariantValues::VariantTypeId,
                    VariantValues::Value,
                    VariantValues::DisplayOrder,
                    VariantValues::CreatedAt,
                    VariantValues::UpdatedAt,
                ])
                .and_where(Expr::col(VariantValues::Id).eq(variant_value_id.to_string()));

            let variant_value = service.db_adapter.query_optional::<VariantValue>(&value_stmt).await?;
            if variant_value.is_none() {
                return Err(Error::NotFoundError);
            }

            let variant_value = variant_value.unwrap();

            // Check if we already have a value from this variant type
            if !variant_type_ids.insert(variant_value.variant_type_id) {
                // Cannot add multiple values from the same variant type
                return Err(Error::AlreadyExistsError);
            }
        }

        // Check if this is the first variant for this item
        let mut count_query = Query::select();
        let count_stmt = count_query
            .from(ItemVariants::Table)
            .expr(Expr::count(Expr::col(ItemVariants::Id)))
            .and_where(Expr::col(ItemVariants::ItemId).eq(self.item_variant.item_id.to_string()));

        let is_first: i64 = service.db_adapter.query_one(&count_stmt).await?;
        println!("Is first: {}", is_first);

        // If this is the first variant and is_default is not specified, make it default
        let is_default = self.item_variant.is_default.unwrap_or(is_first == 0);

        // If this variant is set as default, unset any existing default
        if is_default {
            let mut update_query = Query::update();
            let update_stmt = update_query
                .table(ItemVariants::Table)
                .value(ItemVariants::IsDefault, false.to_string())
                .and_where(Expr::col(ItemVariants::ItemId).eq(self.item_variant.item_id.to_string()))
                .and_where(Expr::col(ItemVariants::IsDefault).eq(true.to_string()));

            service.db_adapter.update_many(&update_stmt).await?;
            println!("Updated default variant");
        }

        let now = Utc::now().naive_utc();
        let variant_id = Uuid::now_v7().into();
        let new_item_variant = ItemVariant {
            id: variant_id,
            item_id: self.item_variant.item_id,
            sku: self.item_variant.sku.clone(),
            price_adjustment: self.item_variant.price_adjustment,
            is_default,
            created_at: now,
            updated_at: now,
        };

        // Insert the new variant
        let mut insert_query = Query::insert();
        let insert_stmt = insert_query
            .into_table(ItemVariants::Table)
            .columns([
                ItemVariants::Id,
                ItemVariants::ItemId,
                ItemVariants::Sku,
                ItemVariants::PriceAdjustment,
                ItemVariants::IsDefault,
                ItemVariants::CreatedAt,
                ItemVariants::UpdatedAt,
            ])
            .values_panic([
                new_item_variant.id.to_string().into(),
                new_item_variant.item_id.to_string().into(),
                new_item_variant.sku.clone().map_or_else(|| "NULL".into(), |s| s.into()),
                new_item_variant.price_adjustment.map_or_else(|| "NULL".into(), |p| p.to_base_unit().into()),
                new_item_variant.is_default.to_string().into(),
                new_item_variant.created_at.to_string().into(),
                new_item_variant.updated_at.to_string().into(),
            ]);

        service.db_adapter.insert_many(&insert_stmt).await?;
        println!("Inserted new item variant");

        // Associate variant values with this item variant
        for variant_value_id in &self.item_variant.variant_value_ids {
            let mut junction_insert = Query::insert();
            let junction_stmt = junction_insert
                .into_table(ItemVariantValues::Table)
                .columns([
                    ItemVariantValues::ItemVariantId,
                    ItemVariantValues::VariantValueId,
                ])
                .values_panic([
                    new_item_variant.id.to_string().into(),
                    variant_value_id.to_string().into(),
                ]);

            service.db_adapter.insert_many(&junction_stmt).await?;
            println!("Associated variant value: {}", variant_value_id);
        }

        Ok(new_item_variant)
    }
}

impl Command for UpdateItemVariantCommand {
    type Output = ItemVariant;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Verify item variant exists
        let mut variant_query = Query::select();
        let variant_stmt = variant_query
            .from(ItemVariants::Table)
            .columns([
                ItemVariants::Id,
                ItemVariants::ItemId,
                ItemVariants::Sku,
                ItemVariants::PriceAdjustment,
                ItemVariants::IsDefault,
                ItemVariants::CreatedAt,
                ItemVariants::UpdatedAt,
            ])
            .and_where(Expr::col(ItemVariants::Id).eq(self.item_variant.id.to_string()));

        let current_variant = service.db_adapter.query_optional::<ItemVariant>(&variant_stmt).await?;
        if current_variant.is_none() {
            return Err(Error::NotFoundError);
        }

        let current_variant = current_variant.unwrap();
        let now = Utc::now().naive_utc();

        // Build the update query
        let mut update_query = Query::update();
        let mut update_stmt = update_query
            .table(ItemVariants::Table)
            .and_where(Expr::col(ItemVariants::Id).eq(self.item_variant.id.to_string()))
            .value(ItemVariants::UpdatedAt, now.to_string());

        // Add optional fields if they exist
        if let Some(sku) = &self.item_variant.sku {
            match sku {
                Some(s) => update_stmt = update_stmt.value(ItemVariants::Sku, s.clone()),
                None => update_stmt = update_stmt.value(ItemVariants::Sku, "NULL"),
            };
        }

        if let Some(price_adjustment) = &self.item_variant.price_adjustment {
            match price_adjustment {
                Some(p) => update_stmt = update_stmt.value(ItemVariants::PriceAdjustment, p.to_base_unit()),
                None => update_stmt = update_stmt.value(ItemVariants::PriceAdjustment, "NULL"),
            };
        }

        // If setting this variant as default, unset any existing default
        if let Some(is_default) = self.item_variant.is_default {
            update_stmt = update_stmt.value(ItemVariants::IsDefault, is_default.to_string());

            if is_default {
                // Unset any other default variants for this item
                let mut unset_query = Query::update();
                let unset_stmt = unset_query
                    .table(ItemVariants::Table)
                    .value(ItemVariants::IsDefault, false.to_string())
                    .and_where(Expr::col(ItemVariants::ItemId).eq(current_variant.item_id.to_string()))
                    .and_where(Expr::col(ItemVariants::Id).ne(self.item_variant.id.to_string()))
                    .and_where(Expr::col(ItemVariants::IsDefault).eq(true.to_string()));

                service.db_adapter.update_many(&unset_stmt).await?;
            }
        }

        // Execute the update
        service.db_adapter.update_many(&update_stmt).await?;

        // Retrieve the updated variant
        let mut updated_query = Query::select();
        let updated_stmt = updated_query
            .from(ItemVariants::Table)
            .columns([
                ItemVariants::Id,
                ItemVariants::ItemId,
                ItemVariants::Sku,
                ItemVariants::PriceAdjustment,
                ItemVariants::IsDefault,
                ItemVariants::CreatedAt,
                ItemVariants::UpdatedAt,
            ])
            .and_where(Expr::col(ItemVariants::Id).eq(self.item_variant.id.to_string()));

        let updated_variant = service.db_adapter.query_one::<ItemVariant>(&updated_stmt).await?;
        Ok(updated_variant)
    }
}

impl Command for DeleteItemVariantCommand {
    type Output = usize;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Get the variant to check if it's default and get its item_id
        let mut variant_query = Query::select();
        let variant_stmt = variant_query
            .from(ItemVariants::Table)
            .columns([
                ItemVariants::Id,
                ItemVariants::ItemId,
                ItemVariants::Sku,
                ItemVariants::PriceAdjustment,
                ItemVariants::IsDefault,
                ItemVariants::CreatedAt,
                ItemVariants::UpdatedAt,
            ])
            .and_where(Expr::col(ItemVariants::Id).eq(self.id.to_string()));

        let variant = service.db_adapter.query_optional::<ItemVariant>(&variant_stmt).await?;
        if variant.is_none() {
            return Err(Error::NotFoundError);
        }

        let variant = variant.unwrap();

        // Delete associated variant values first
        let mut delete_values_query = Query::delete();
        let delete_values_stmt = delete_values_query
            .from_table(ItemVariantValues::Table)
            .and_where(Expr::col(ItemVariantValues::ItemVariantId).eq(self.id.to_string()));

        service.db_adapter.delete(&delete_values_stmt).await?;

        // Delete the variant
        let mut delete_variant_query = Query::delete();
        let delete_variant_stmt = delete_variant_query
            .from_table(ItemVariants::Table)
            .and_where(Expr::col(ItemVariants::Id).eq(self.id.to_string()));

        let num_deleted = service.db_adapter.delete(&delete_variant_stmt).await?;

        // If this was the default variant, set another one as default if available
        if variant.is_default {
            // Find another variant for this item
            let mut another_query = Query::select();
            let another_stmt = another_query
                .from(ItemVariants::Table)
                .columns([
                    ItemVariants::Id,
                    ItemVariants::ItemId,
                    ItemVariants::Sku,
                    ItemVariants::PriceAdjustment,
                    ItemVariants::IsDefault,
                    ItemVariants::CreatedAt,
                    ItemVariants::UpdatedAt,
                ])
                .and_where(Expr::col(ItemVariants::ItemId).eq(variant.item_id.to_string()))
                .limit(1);

            let another_variant = service.db_adapter.query_optional::<ItemVariant>(&another_stmt).await?;

            if let Some(another) = another_variant {
                // Set this variant as default
                let mut update_query = Query::update();
                let update_stmt = update_query
                    .table(ItemVariants::Table)
                    .value(ItemVariants::IsDefault, true.to_string())
                    .and_where(Expr::col(ItemVariants::Id).eq(another.id.to_string()));

                service.db_adapter.update_many(&update_stmt).await?;
            }
        }

        if num_deleted == 0 {
            Err(Error::NotFoundError)
        } else {
            Ok(num_deleted as usize)
        }
    }
}

impl Command for GetItemVariantCommand {
    type Output = ItemVariant;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let mut query = Query::select();
        let stmt = query
            .from(ItemVariants::Table)
            .columns([
                ItemVariants::Id,
                ItemVariants::ItemId,
                ItemVariants::Sku,
                ItemVariants::PriceAdjustment,
                ItemVariants::IsDefault,
                ItemVariants::CreatedAt,
                ItemVariants::UpdatedAt,
            ])
            .and_where(Expr::col(ItemVariants::Id).eq(self.id.to_string()));

        let item_variant = service.db_adapter.query_optional::<ItemVariant>(&stmt).await?;
        match item_variant {
            Some(variant) => Ok(variant),
            None => Err(Error::NotFoundError),
        }
    }
}

impl Command for ListItemVariantsCommand {
    type Output = Vec<ItemVariant>;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let mut query_builder = Query::select();
        let mut query = query_builder
            .from(ItemVariants::Table)
            .columns([
                ItemVariants::Id,
                ItemVariants::ItemId,
                ItemVariants::Sku,
                ItemVariants::PriceAdjustment,
                ItemVariants::IsDefault,
                ItemVariants::CreatedAt,
                ItemVariants::UpdatedAt,
            ]);

        if let Some(item_id) = &self.item_id {
            query = query.and_where(Expr::col(ItemVariants::ItemId).eq(item_id.to_string()));
        }

        let results = service.db_adapter.query_many::<ItemVariant>(&query).await?;
        Ok(results)
    }
}

impl Command for AssignVariantValueCommand {
    type Output = usize;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Verify item variant exists
        let mut variant_query = Query::select();
        let variant_stmt = variant_query
            .from(ItemVariants::Table)
            .column(ItemVariants::Id)
            .and_where(Expr::col(ItemVariants::Id).eq(self.item_variant_id.to_string()));

        let variant = service.db_adapter.query_optional::<DbUuid>(&variant_stmt).await?;
        println!("Variant: {:?}", variant);
        if variant.is_none() {
            return Err(Error::NotFoundError);
        }

        // Verify variant value exists and get its type
        let mut value_query = Query::select();
        let value_stmt = value_query
            .from(VariantValues::Table)
            .columns([
                VariantValues::Id,
                VariantValues::VariantTypeId,
                VariantValues::Value,
                VariantValues::DisplayOrder,
                VariantValues::CreatedAt,
                VariantValues::UpdatedAt,
            ])
            .and_where(Expr::col(VariantValues::Id).eq(self.variant_value_id.to_string()));

        let new_variant_value = service.db_adapter.query_optional::<VariantValue>(&value_stmt).await?;
        println!("New variant value: {:?}", new_variant_value);
        if new_variant_value.is_none() {
            return Err(Error::NotFoundError);
        }

        let new_variant_value = new_variant_value.unwrap();

        // Check if the association already exists
        let mut exists_query = Query::select();
        let exists_stmt = exists_query
            .from(ItemVariantValues::Table)
            .expr(Expr::count(Expr::col(ItemVariantValues::ItemVariantId)))
            .and_where(Expr::col(ItemVariantValues::ItemVariantId).eq(self.item_variant_id.to_string()))
            .and_where(Expr::col(ItemVariantValues::VariantValueId).eq(self.variant_value_id.to_string()));

        let count: i64 = service.db_adapter.query_one(&exists_stmt).await?;
        println!("Count: {}", count);
        if count > 0 {
            return Ok(0); // Association already exists
        }

        // Check if there's already a value from the same variant type
        // First get all variant values associated with this item variant
        let mut values_query = Query::select();
        let values_stmt = values_query
            .from(ItemVariantValues::Table)
            .columns([
                ItemVariantValues::VariantValueId,
            ])
            .and_where(Expr::col(ItemVariantValues::ItemVariantId).eq(self.item_variant_id.to_string()));

        let variant_value_ids = service.db_adapter.query_many::<DbUuid>(&values_stmt).await?;

        // For each variant value, check its type
        for item_variant_value in variant_value_ids {
            let mut value_type_query = Query::select();
            let value_type_stmt = value_type_query
                .from(VariantValues::Table)
                .columns([
                    VariantValues::VariantTypeId,
                ])
                .and_where(Expr::col(VariantValues::Id).eq(item_variant_value.to_string()));

            let existing_value = service.db_adapter.query_one::<DbUuid>(&value_type_stmt).await?;

            if existing_value == new_variant_value.variant_type_id {
                // Cannot have multiple values from the same variant type
                return Err(Error::AlreadyExistsError);
            }
        }

        // Create the association
        let mut insert_query = Query::insert();
        let insert_stmt = insert_query
            .into_table(ItemVariantValues::Table)
            .columns([
                ItemVariantValues::ItemVariantId,
                ItemVariantValues::VariantValueId,
            ])
            .values_panic([
                self.item_variant_id.to_string().into(),
                self.variant_value_id.to_string().into(),
            ]);

        service.db_adapter.insert_many(&insert_stmt).await?;

        Ok(1)
    }
}

impl Command for RemoveVariantValueCommand {
    type Output = usize;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Delete the association
        let mut delete_query = Query::delete();
        let delete_stmt = delete_query
            .from_table(ItemVariantValues::Table)
            .and_where(Expr::col(ItemVariantValues::ItemVariantId).eq(self.item_variant_id.to_string()))
            .and_where(Expr::col(ItemVariantValues::VariantValueId).eq(self.variant_value_id.to_string()));

        let deleted_count = service.db_adapter.delete(&delete_stmt).await?;

        Ok(deleted_count as usize)
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
    use crate::core::models::catalog::item_group_model::ItemCategoryNew;
    use crate::core::models::catalog::item_model::{ItemNature, ItemState, NewItem};
    use crate::core::models::catalog::variant_type_model::VariantTypeNewInput;
    use crate::core::models::catalog::variant_value_model::VariantValueNewInput;
    use crate::core::types::money::Money;
    use crate::adapters::outgoing::database::DatabaseAdapter;
    use sea_query::{Expr, Query};
    use tokio;

    async fn create_test_item_category(service: &mut AppService) -> DbUuid {
        let command = CreateItemGroupCommand {
            category: ItemCategoryNew {
                name: "Test Category".to_string(),
                description: None,
            },
        };
        let category = command.exec(service).await.unwrap();
        category.id
    }

    async fn create_test_item(service: &mut AppService) -> Item {
        let category_id = create_test_item_category(service).await;
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
        command.exec(service).await.unwrap()
    }

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

    async fn create_test_variant_value(
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
        command.exec(service).await.unwrap()
    }

    async fn create_test_item_variant(service: &mut AppService) -> ItemVariant {
        let item = create_test_item(service).await;
        let variant_type = create_test_variant_type(service).await;
        let variant_value = create_test_variant_value(service, variant_type.id).await;

        let command = CreateItemVariantCommand {
            item_variant: ItemVariantNewInput {
                item_id: item.id,
                sku: Some("TEST-SKU-001".to_string()),
                price_adjustment: Some(Money::from(100)),
                is_default: Some(true),
                variant_value_ids: vec![variant_value.id],
            },
        };
        command.exec(service).await.unwrap()
    }

    #[tokio::test]
    async fn test_create_item_variant() {
        let mut service = setup_service().await;
        let item = create_test_item(&mut service).await;
        let variant_type = create_test_variant_type(&mut service).await;
        let variant_value = create_test_variant_value(&mut service, variant_type.id).await;

        let command = CreateItemVariantCommand {
            item_variant: ItemVariantNewInput {
                item_id: item.id,
                sku: Some("TEST-SKU-001".to_string()),
                price_adjustment: Some(Money::from(100)),
                is_default: Some(true),
                variant_value_ids: vec![variant_value.id],
            },
        };

        let item_variant = command.exec(&mut service).await.unwrap();
        assert_eq!(item_variant.item_id, item.id);
        assert_eq!(item_variant.sku, Some("TEST-SKU-001".to_string()));
        assert_eq!(item_variant.price_adjustment, Some(Money::from(100)));
        assert!(item_variant.is_default);

        // Verify variant value association
        let mut query = Query::select();
        let query_stmt = query
            .from(ItemVariantValues::Table)
            .columns([
                ItemVariantValues::ItemVariantId,
                ItemVariantValues::VariantValueId,
            ])
            .and_where(Expr::col(ItemVariantValues::ItemVariantId).eq(item_variant.id.to_string()));

        let associations = service.db_adapter.query_many::<ItemVariantValue>(&query_stmt).await.unwrap();

        assert_eq!(associations.len(), 1);
        assert_eq!(associations[0].variant_value_id, variant_value.id);
    }

    #[tokio::test]
    async fn test_update_item_variant() {
        let mut service = setup_service().await;
        let created = create_test_item_variant(&mut service).await;

        let update_command = UpdateItemVariantCommand {
            item_variant: ItemVariantUpdateInput {
                id: created.id,
                sku: Some(Some("UPDATED-SKU".to_string())),
                price_adjustment: Some(Some(Money::from(200))),
                is_default: Some(true),
                updated_at: None,
            },
        };

        let updated = update_command.exec(&mut service).await.unwrap();
        assert_eq!(updated.sku, Some("UPDATED-SKU".to_string()));
        assert_eq!(updated.price_adjustment, Some(Money::from(200)));
        assert!(updated.is_default);
    }

    #[tokio::test]
    async fn test_get_item_variant() {
        let mut service = setup_service().await;
        let created = create_test_item_variant(&mut service).await;

        let get_command = GetItemVariantCommand { id: created.id };
        let retrieved = get_command.exec(&mut service).await.unwrap();
        assert_eq!(retrieved.id, created.id);
        assert_eq!(retrieved.sku, created.sku);
    }

    #[tokio::test]
    async fn test_list_item_variants() {
        let mut service = setup_service().await;
        let item = create_test_item(&mut service).await;
        let variant_type = create_test_variant_type(&mut service).await;
        let variant_value1 = create_test_variant_value(&mut service, variant_type.id).await;
        let variant_value2 = create_test_variant_value(&mut service, variant_type.id).await;

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
        let variant1 = command1.exec(&mut service).await.unwrap();

        let command2 = CreateItemVariantCommand {
            item_variant: ItemVariantNewInput {
                item_id: item.id,
                sku: Some("SKU-2".to_string()),
                price_adjustment: Some(Money::from(200)),
                is_default: Some(false),
                variant_value_ids: vec![variant_value2.id],
            },
        };
        let variant2 = command2.exec(&mut service).await.unwrap();

        // List all variants
        let list_command = ListItemVariantsCommand { item_id: None };
        let all_variants = list_command.exec(&mut service).await.unwrap();
        assert!(all_variants.len() >= 2);

        // List variants for specific item
        let list_command = ListItemVariantsCommand {
            item_id: Some(item.id),
        };
        let item_variants = list_command.exec(&mut service).await.unwrap();
        assert_eq!(item_variants.len(), 2);
        assert!(item_variants.iter().any(|v| v.id == variant1.id));
        assert!(item_variants.iter().any(|v| v.id == variant2.id));
    }

    #[tokio::test]
    async fn test_delete_item_variant() {
        let mut service = setup_service().await;
        let created = create_test_item_variant(&mut service).await;

        let delete_command = DeleteItemVariantCommand { id: created.id };
        let result = delete_command.exec(&mut service).await.unwrap();
        assert_eq!(result, 1);

        // Verify it's gone
        let get_command = GetItemVariantCommand { id: created.id };
        let result = get_command.exec(&mut service).await;
        assert!(result.is_err());

        // Verify associations are gone
        let mut query = Query::select();
        let query_stmt = query
            .from(ItemVariantValues::Table)
            .columns([
                ItemVariantValues::ItemVariantId,
                ItemVariantValues::VariantValueId,
            ])
            .and_where(Expr::col(ItemVariantValues::ItemVariantId).eq(created.id.to_string()));

        let associations = service.db_adapter.query_many::<ItemVariantValue>(&query_stmt).await.unwrap();

        assert_eq!(associations.len(), 0);
    }

    #[tokio::test]
    async fn test_assign_and_remove_variant_value() {
        let mut service = setup_service().await;
        let item_variant = create_test_item_variant(&mut service).await;
        let variant_type = create_test_variant_type(&mut service).await;
        let new_variant_value = create_test_variant_value(&mut service, variant_type.id).await;

        // Assign new variant value
        let assign_command = AssignVariantValueCommand {
            item_variant_id: item_variant.id,
            variant_value_id: new_variant_value.id,
        };
        let result = assign_command.exec(&mut service).await.unwrap();
        assert_eq!(result, 1);

        // Verify association exists
        let mut query1 = Query::select();
        let query1_stmt = query1
            .from(ItemVariantValues::Table)
            .columns([
                ItemVariantValues::ItemVariantId,
                ItemVariantValues::VariantValueId,
            ])
            .and_where(Expr::col(ItemVariantValues::ItemVariantId).eq(item_variant.id.to_string()))
            .and_where(Expr::col(ItemVariantValues::VariantValueId).eq(new_variant_value.id.to_string()));

        let associations = service.db_adapter.query_many::<ItemVariantValue>(&query1_stmt).await.unwrap();

        assert_eq!(associations.len(), 1);

        // Remove variant value
        let remove_command = RemoveVariantValueCommand {
            item_variant_id: item_variant.id,
            variant_value_id: new_variant_value.id,
        };
        let result = remove_command.exec(&mut service).await.unwrap();
        assert_eq!(result, 1);

        // Verify association is gone
        let mut query2 = Query::select();
        let query2_stmt = query2
            .from(ItemVariantValues::Table)
            .columns([
                ItemVariantValues::ItemVariantId,
                ItemVariantValues::VariantValueId,
            ])
            .and_where(Expr::col(ItemVariantValues::ItemVariantId).eq(item_variant.id.to_string()))
            .and_where(Expr::col(ItemVariantValues::VariantValueId).eq(new_variant_value.id.to_string()));

        let associations = service.db_adapter.query_many::<ItemVariantValue>(&query2_stmt).await.unwrap();

        assert_eq!(associations.len(), 0);
    }

    #[tokio::test]
    async fn test_prevent_duplicate_variant_types_in_create() {
        let mut service = setup_service().await;
        let item = create_test_item(&mut service).await;
        let variant_type = create_test_variant_type(&mut service).await;
        let variant_value1 = create_test_variant_value(&mut service, variant_type.id).await;
        let variant_value2 = create_test_variant_value(&mut service, variant_type.id).await;

        // Try to create a variant with two values from the same type
        let command = CreateItemVariantCommand {
            item_variant: ItemVariantNewInput {
                item_id: item.id,
                sku: Some("TEST-SKU-001".to_string()),
                price_adjustment: Some(Money::from(100)),
                is_default: Some(true),
                variant_value_ids: vec![variant_value1.id, variant_value2.id],
            },
        };

        // This should fail with AlreadyExistsError
        let result = command.exec(&mut service).await;
        assert!(result.is_err());
        match result {
            Err(Error::AlreadyExistsError) => {} // Expected error
            _ => panic!("Expected AlreadyExistsError, got {:?}", result),
        }
    }

    #[tokio::test]
    async fn test_prevent_duplicate_variant_types_in_assign() {
        let mut service = setup_service().await;
        let item = create_test_item(&mut service).await;
        println!("Created item: {:?}", item);
        let variant_type1 = create_test_variant_type(&mut service).await;
        println!("Created variant type: {:?}", variant_type1);
        let variant_value1 = create_test_variant_value(&mut service, variant_type1.id).await;
        println!("Created variant value: {:?}", variant_value1);

        // Create a variant with one value
        let command = CreateItemVariantCommand {
            item_variant: ItemVariantNewInput {
                item_id: item.id,
                sku: Some("TEST-SKU-001".to_string()),
                price_adjustment: Some(Money::from(100)),
                is_default: Some(true),
                variant_value_ids: vec![variant_value1.id],
            },
        };

        let item_variant = command.exec(&mut service).await.unwrap();
        println!("Created item variant: {:?}", item_variant);

        // Try to assign another value from the same type
        let variant_value2 = create_test_variant_value(&mut service, variant_type1.id).await;
        println!("Created variant value 2: {:?}", variant_value2);
        let assign_command = AssignVariantValueCommand {
            item_variant_id: item_variant.id,
            variant_value_id: variant_value2.id,
        };

        // This should fail with AlreadyExistsError
        let result = assign_command.exec(&mut service).await;
        println!("Result: {:?}", result);
        assert!(result.is_err());
        match result {
            Err(Error::AlreadyExistsError) => {} // Expected error
            _ => panic!("Expected AlreadyExistsError, got {:?}", result),
        }
    }
}
