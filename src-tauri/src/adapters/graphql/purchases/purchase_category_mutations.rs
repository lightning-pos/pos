use juniper::FieldResult;

use crate::{
    core::{
        commands::{
            purchases::purchase_category_commands::{
                CreatePurchaseCategoryCommand, DeletePurchaseCategoryCommand,
                UpdatePurchaseCategoryCommand,
            },
            Command,
        },
        models::purchases::purchase_category_model::{
            PurchaseCategory, PurchaseCategoryNew, PurchaseCategoryUpdate,
        },
        types::db_uuid::DbUuid,
    },
    AppState,
};

pub fn create_purchase_category(
    name: String,
    description: Option<String>,
    context: &AppState,
) -> FieldResult<PurchaseCategory> {
    let mut service = context.service.lock().unwrap();
    let command = CreatePurchaseCategoryCommand {
        category: PurchaseCategoryNew { name, description },
    };
    let result = command.exec(&mut service)?;
    Ok(result)
}

pub fn update_purchase_category(
    id: DbUuid,
    name: Option<String>,
    description: Option<Option<String>>,
    context: &AppState,
) -> FieldResult<PurchaseCategory> {
    let mut service = context.service.lock().unwrap();
    let command = UpdatePurchaseCategoryCommand {
        category: PurchaseCategoryUpdate {
            id,
            name,
            description,
            state: None,
            updated_at: None,
        },
    };
    let result = command.exec(&mut service)?;
    Ok(result)
}

pub fn delete_purchase_category(id: DbUuid, context: &AppState) -> FieldResult<DbUuid> {
    let mut service = context.service.lock().unwrap();
    let command = DeletePurchaseCategoryCommand { id };
    let _ = command.exec(&mut service)?;
    // Return the id of the deleted category
    Ok(id)
}
