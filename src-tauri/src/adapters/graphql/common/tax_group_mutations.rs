use juniper::FieldResult;

use crate::{
    core::{
        commands::{
            common::tax_group_commands::{
                AssignTaxToGroupCommand, CreateTaxGroupCommand, DeleteTaxGroupCommand,
                RemoveTaxFromGroupCommand, UpdateTaxGroupCommand,
            },
            Command,
        },
        models::common::tax_group_model::{TaxGroup, TaxGroupNewInput, TaxGroupUpdateInput},
        types::db_uuid::DbUuid,
    },
    AppState,
};

pub async fn create_tax_group(input: TaxGroupNewInput, context: &AppState) -> FieldResult<TaxGroup> {
    let mut service = context.service.lock().await;
    let res = CreateTaxGroupCommand { tax_group: input }.exec(&mut service).await?;
    Ok(res)
}

pub async fn update_tax_group(input: TaxGroupUpdateInput, context: &AppState) -> FieldResult<TaxGroup> {
    let mut service = context.service.lock().await;
    let res = UpdateTaxGroupCommand { tax_group: input }.exec(&mut service).await?;
    Ok(res)
}

pub async fn delete_tax_group(id: DbUuid, context: &AppState) -> FieldResult<i32> {
    let mut service = context.service.lock().await;
    let res = DeleteTaxGroupCommand { id }.exec(&mut service).await?;
    Ok(res)
}

pub async fn assign_tax_to_group(
    tax_group_id: DbUuid,
    tax_id: DbUuid,
    context: &AppState,
) -> FieldResult<i32> {
    let mut service = context.service.lock().await;
    let res = AssignTaxToGroupCommand {
        tax_group_id,
        tax_id,
    }
    .exec(&mut service).await?;
    Ok(res)
}

pub async fn remove_tax_from_group(
    tax_group_id: DbUuid,
    tax_id: DbUuid,
    context: &AppState,
) -> FieldResult<i32> {
    let mut service = context.service.lock().await;
    let res = RemoveTaxFromGroupCommand {
        tax_group_id,
        tax_id,
    }
    .exec(&mut service).await?;
    Ok(res)
}
