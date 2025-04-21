use juniper::FieldResult;

use crate::{
    core::{
        commands::{
            finance::cost_center_commands::{
                CreateCostCenterCommand, DeleteCostCenterCommand, UpdateCostCenterCommand,
            },
            Command,
        },
        models::finance::cost_center_model::{
            CostCenter, CostCenterNewInput, CostCenterState, CostCenterUpdateInput,
        },
        types::db_uuid::DbUuid,
    },
    AppState,
};

pub async fn create_cost_center(
    name: String,
    code: String,
    description: Option<String>,
    state: Option<CostCenterState>,
    context: &AppState,
) -> FieldResult<CostCenter> {
    let mut service = context.service.lock().await;
    let command = CreateCostCenterCommand {
        cost_center: CostCenterNewInput {
            name,
            code,
            description,
            state,
        },
    };
    let result = command.exec(&mut service).await?;
    Ok(result)
}

pub async fn update_cost_center(
    id: DbUuid,
    name: Option<String>,
    code: Option<String>,
    description: Option<Option<String>>,
    state: Option<CostCenterState>,
    context: &AppState,
) -> FieldResult<CostCenter> {
    let mut service = context.service.lock().await;
    let command = UpdateCostCenterCommand {
        cost_center: CostCenterUpdateInput {
            id,
            name,
            code,
            description,
            state,
        },
    };
    let result = command.exec(&mut service).await?;
    Ok(result)
}

pub async fn delete_cost_center(id: DbUuid, context: &AppState) -> FieldResult<DbUuid> {
    let mut service = context.service.lock().await;
    let command = DeleteCostCenterCommand { id };
    let _ = command.exec(&mut service).await?;
    // Return the id of the deleted cost center
    Ok(id)
}
