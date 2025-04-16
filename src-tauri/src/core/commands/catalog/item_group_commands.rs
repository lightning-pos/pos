use chrono::Utc;
use sea_query::{Expr, Query, SqliteQueryBuilder};
use uuid::Uuid;

use crate::{
    adapters::outgoing::database::DatabaseAdapter, core::{
        commands::{app_service::AppService, Command},
        models::catalog::item_group_model::{
            ItemCategories, ItemGroup, ItemGroupNew, ItemGroupState, ItemGroupUpdate,
        },
        types::db_uuid::DbUuid,
    }, error::{Error, Result}
};

// Commands
pub struct CreateItemGroupCommand {
    pub category: ItemGroupNew,
}

pub struct UpdateItemGroupCommand {
    pub category: ItemGroupUpdate,
}

pub struct DeleteItemGroupCommand {
    pub id: DbUuid,
}

// Command Implementations
impl Command for CreateItemGroupCommand {
    type Output = ItemGroup;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.db_adapter.transaction(|db| {
            // Check if a category with the same name already exists
            let select_query = Query::select()
                .from(ItemCategories::Table)
                .column(ItemCategories::Id)
                .and_where(Expr::col(ItemCategories::Name).eq(self.category.name.clone()))
                .to_string(SqliteQueryBuilder);

            let existing = db.query_optional::<ItemGroup>(&select_query, vec![])?;

            if existing.is_some() {
                return Err(Error::UniqueConstraintError);
            }

            let now = Utc::now().naive_utc();
            let item_id = Uuid::now_v7().into();

            let new_cat = ItemGroup {
                id: item_id,
                name: self.category.name.clone(),
                description: self.category.description.clone(),
                state: ItemGroupState::Inactive,
                created_at: now,
                updated_at: now,
            };

            // Insert the new category
            let insert_query = Query::insert()
                .into_table(ItemCategories::Table)
                .columns([
                    ItemCategories::Id,
                    ItemCategories::Name,
                    ItemCategories::Description,
                    ItemCategories::State,
                    ItemCategories::CreatedAt,
                    ItemCategories::UpdatedAt,
                ])
                .values_panic([
                    new_cat.id.to_string().into(),
                    new_cat.name.clone().into(),
                    new_cat.description.clone().map_or_else(|| "NULL".into(), |d| d.into()),
                    new_cat.state.to_string().into(),
                    new_cat.created_at.to_string().into(),
                    new_cat.updated_at.to_string().into(),
                ])
                .to_string(SqliteQueryBuilder);

            db.execute(&insert_query, vec![])?;

            Ok(new_cat)
        })
    }
}

impl Command for UpdateItemGroupCommand {
    type Output = ItemGroup;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.db_adapter.transaction(|db| {
            // Check if the category exists
            let select_query = Query::select()
                .from(ItemCategories::Table)
                .columns([
                    ItemCategories::Id,
                    ItemCategories::Name,
                    ItemCategories::Description,
                    ItemCategories::State,
                    ItemCategories::CreatedAt,
                    ItemCategories::UpdatedAt,
                ])
                .and_where(Expr::col(ItemCategories::Id).eq(self.category.id.to_string()))
                .to_string(SqliteQueryBuilder);

            let existing = db.query_optional::<ItemGroup>(&select_query, vec![])?;

            if existing.is_none() {
                return Err(Error::NotFoundError);
            }

            let now = Utc::now().naive_utc();

            // Build the update query
            let mut update_query = Query::update();
            update_query.table(ItemCategories::Table)
                .and_where(Expr::col(ItemCategories::Id).eq(self.category.id.to_string()))
                .value(ItemCategories::UpdatedAt, now.to_string());

            // Add optional fields if they exist
            if let Some(name) = &self.category.name {
                update_query.value(ItemCategories::Name, name.clone());
            }

            if let Some(description) = &self.category.description {
                match description {
                    Some(desc) => update_query.value(ItemCategories::Description, desc.clone()),
                    None => update_query.value(ItemCategories::Description, "NULL"),
                };
            }

            if let Some(state) = &self.category.state {
                update_query.value(ItemCategories::State, state.to_string());
            }

            let sql = update_query.to_string(SqliteQueryBuilder);
            db.execute(&sql, vec![])?;

            // Retrieve the updated category
            let updated_query = Query::select()
                .from(ItemCategories::Table)
                .columns([
                    ItemCategories::Id,
                    ItemCategories::Name,
                    ItemCategories::Description,
                    ItemCategories::State,
                    ItemCategories::CreatedAt,
                    ItemCategories::UpdatedAt,
                ])
                .and_where(Expr::col(ItemCategories::Id).eq(self.category.id.to_string()))
                .to_string(SqliteQueryBuilder);

            let updated_cat = db.query_one::<ItemGroup>(&updated_query, vec![])?;
            Ok(updated_cat)
        })
    }
}

impl Command for DeleteItemGroupCommand {
    type Output = i32;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.db_adapter.transaction(|db| {
            // Check if category has items
            use crate::core::models::catalog::item_model::Items;

            let count_query = Query::select()
                .from(Items::Table)
                .expr(Expr::count(Expr::col(Items::Id)))
                .and_where(Expr::col(Items::CategoryId).eq(self.id.to_string()))
                .to_string(SqliteQueryBuilder);

            // Execute the count query
            let count_result = db.query_one::<i64>(&count_query, vec![])?;

            if count_result > 0 {
                return Err(Error::ForeignKeyConstraintError);
            }

            // Delete the category
            let delete_query = Query::delete()
                .from_table(ItemCategories::Table)
                .and_where(Expr::col(ItemCategories::Id).eq(self.id.to_string()))
                .to_string(SqliteQueryBuilder);

            let affected_rows = db.execute(&delete_query, vec![])?;

            Ok(affected_rows as i32)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{commands::tests::setup_service, models::catalog::item_group_model::ItemGroupState};
    use diesel::result::Error::NotFound;
    use uuid::Uuid;

    #[test]
    fn test_create_item_category() {
        let mut app_service = setup_service();
        let new_cat = ItemGroupNew {
            name: "test".to_string(),
            description: Some("test description".to_string()),
        };
        let command = CreateItemGroupCommand { category: new_cat };
        let result = command.exec(&mut app_service);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_item_category_already_exists() {
        let mut app_service = setup_service();
        let new_cat = ItemGroupNew {
            name: "test".to_string(),
            description: Some("test description".to_string()),
        };
        let command = CreateItemGroupCommand {
            category: new_cat.clone(),
        };
        let result = command.exec(&mut app_service);

        assert!(result.is_ok());

        let command = CreateItemGroupCommand { category: new_cat };
        let result = command.exec(&mut app_service);

        assert!(matches!(result, Err(Error::UniqueConstraintError)));
    }

    #[test]
    fn test_update_item_category() {
        let mut app_service = setup_service();
        let new_cat = ItemGroupNew {
            name: "test".to_string(),
            description: Some("test description".to_string()),
        };
        let create_command = CreateItemGroupCommand { category: new_cat };
        let category = create_command.exec(&mut app_service).unwrap();

        let updated_category = ItemGroupUpdate {
            id: category.id,
            name: Some("updated test".to_string()),
            description: None,
            state: None,
            updated_at: None,
        };

        let update_command = UpdateItemGroupCommand {
            category: updated_category,
        };
        let result = update_command.exec(&mut app_service);
        assert!(result.is_ok());
        assert!(result.unwrap().name == "updated test");
    }

    #[test]
    fn test_update_item_category_does_not_exist() {
        let mut app_service = setup_service();
        let now = Utc::now().naive_utc();
        let category = ItemGroupUpdate {
            id: Uuid::now_v7().into(),
            name: Some("test".to_string()),
            description: Some(Some("test description".to_string())),
            state: Some(ItemGroupState::Active),
            updated_at: Some(now),
        };

        let command = UpdateItemGroupCommand { category };
        let result = command.exec(&mut app_service);
        assert!(matches!(result, Err(Error::DieselError(NotFound))))
    }

    #[test]
    fn test_delete_item_category() {
        let mut app_service = setup_service();
        let new_cat = ItemGroupNew {
            name: "test".to_string(),
            description: Some("test description".to_string()),
        };

        let create_command = CreateItemGroupCommand { category: new_cat };
        let cat = create_command.exec(&mut app_service).unwrap();

        let delete_command = DeleteItemGroupCommand { id: cat.id };
        let result = delete_command.exec(&mut app_service);
        assert!(result.is_ok());
    }
}
