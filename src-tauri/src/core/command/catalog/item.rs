use chrono::Utc;
use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::{
    core::{
        command::{app_service::AppService, Command},
        entities::catalog::{
            item::{Item, NewItem, UpdateItem},
            item_category::ItemCategory,
        },
    },
    error::{Error, Result},
    schema::{item_categories, items},
};

// Commands
pub struct CreateItemCommand {
    pub item: NewItem,
}

pub struct UpdateItemCommand {
    pub item: UpdateItem,
}

pub struct DeleteItemCommand {
    pub id: String,
}

// Command Implementations
impl Command for CreateItemCommand {
    type Output = Item;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            // Verify category exists
            item_categories::table
                .filter(item_categories::id.eq(&self.item.category_id))
                .select(ItemCategory::as_select())
                .get_result::<ItemCategory>(conn)?;

            let now = Utc::now().naive_utc();
            let new_item = Item {
                id: Uuid::now_v7().to_string(),
                name: self.item.name.clone(),
                description: self.item.description.clone(),
                nature: self.item.nature,
                state: self.item.state,
                price: self.item.price,
                category_id: self.item.category_id.clone(),
                created_at: now,
                updated_at: now,
            };

            let res = diesel::insert_into(items::table)
                .values(&new_item)
                .returning(Item::as_returning())
                .get_result(conn)?;

            Ok(res)
        })
    }
}

impl Command for UpdateItemCommand {
    type Output = Item;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            // Verify category exists
            if let Some(cat_id) = self.item.category_id.clone() {
                item_categories::table
                    .filter(item_categories::id.eq(&cat_id))
                    .select(ItemCategory::as_select())
                    .get_result::<ItemCategory>(conn)?;
            }

            // Verify item exists
            items::table
                .find(&self.item.id)
                .select(Item::as_select())
                .get_result::<Item>(conn)?;

            let now = Utc::now().naive_utc();

            let mut item = self.item.clone();
            item.updated_at = Some(now);

            let res = diesel::update(items::table.find(&self.item.id))
                .set(&item)
                .returning(Item::as_returning())
                .get_result(conn)?;

            Ok(res)
        })
    }
}

impl Command for DeleteItemCommand {
    type Output = i32;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let result = diesel::delete(items::table.find(&self.id)).execute(conn)?;

            if result == 0 {
                return Err(Error::NotFoundError);
            }

            Ok(result as i32)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{
        command::catalog::item_category::CreateItemCategoryCommand,
        entities::catalog::{
            item::{ItemNature, ItemState},
            item_category::NewItemCategory,
        },
    };
    use diesel::result::Error::NotFound;
    use uuid::Uuid;

    #[test]
    fn test_create_item() {
        let mut app_service = AppService::new(":memory:");
        let new_cat = NewItemCategory {
            name: String::from("test"),
            description: None,
        };
        let create_cat_command = CreateItemCategoryCommand { category: new_cat };
        let cat = create_cat_command.exec(&mut app_service).unwrap();
        let new_item = NewItem {
            id: Uuid::now_v7().to_string(),
            name: String::from("test"),
            description: Some(String::from("test description")),
            nature: ItemNature::Goods,
            state: ItemState::Active,
            price: 0,
            category_id: cat.id,
        };
        let command = CreateItemCommand { item: new_item };
        let result = command.exec(&mut app_service);

        assert!(result.is_ok());
    }

    #[test]
    fn test_update_item() {
        let mut app_service = AppService::new(":memory:");
        let new_cat = NewItemCategory {
            name: "test".to_string(),
            description: None,
        };
        let create_cat_command = CreateItemCategoryCommand { category: new_cat };
        let cat = create_cat_command.exec(&mut app_service).unwrap();
        let now = Utc::now().naive_utc();
        let item_id = Uuid::now_v7().to_string();
        let new_item = NewItem {
            id: item_id.clone(),
            name: String::from("test"),
            description: Some(String::from("test description")),
            nature: ItemNature::Goods,
            state: ItemState::Active,
            price: 0,
            category_id: cat.id.clone(),
        };

        let create_command = CreateItemCommand { item: new_item };
        let item = create_command.exec(&mut app_service).unwrap();

        let updated_item = UpdateItem {
            id: item.id,
            name: Some(String::from("test2")),
            description: None,
            nature: None,
            state: None,
            price: None,
            category_id: None,
            updated_at: Some(now),
        };

        let update_command = UpdateItemCommand { item: updated_item };
        let result = update_command.exec(&mut app_service);
        assert!(result.is_ok());
    }

    #[test]
    fn test_update_item_does_not_exist() {
        let mut app_service = AppService::new(":memory:");
        let now = Utc::now().naive_utc();
        let item = UpdateItem {
            id: Uuid::now_v7().to_string(),
            name: Some("test".to_string()),
            description: None,
            nature: None,
            state: None,
            price: None,
            category_id: None,
            updated_at: Some(now),
        };

        let command = UpdateItemCommand { item };
        let result = command.exec(&mut app_service);

        assert!(matches!(result, Err(Error::DieselError(NotFound))));
    }

    #[test]
    fn test_delete_item() {
        let mut app_service = AppService::new(":memory:");
        let new_cat = NewItemCategory {
            name: "test".to_string(),
            description: None,
        };
        let create_cat_command = CreateItemCategoryCommand { category: new_cat };
        let cat = create_cat_command.exec(&mut app_service).unwrap();
        let item = NewItem {
            id: Uuid::now_v7().to_string(),
            name: "test".to_string(),
            description: Some("test description".to_string()),
            nature: ItemNature::Goods,
            state: ItemState::Active,
            price: 0,
            category_id: cat.id.clone(),
        };

        let create_command = CreateItemCommand { item: item.clone() };
        let new_item = create_command.exec(&mut app_service).unwrap();

        let delete_command = DeleteItemCommand { id: new_item.id };
        let result = delete_command.exec(&mut app_service);
        assert!(result.is_ok());
    }

    #[test]
    fn test_delete_item_does_not_exist() {
        let mut app_service = AppService::new(":memory:");
        let command = DeleteItemCommand {
            id: Uuid::now_v7().to_string(),
        };
        let result = command.exec(&mut app_service);
        assert!(matches!(result, Err(Error::NotFoundError)));
    }
}
