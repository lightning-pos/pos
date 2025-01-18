use crate::adapters::outgoing::graphql::Schema;
use crate::core::command::Command;
use crate::core::entities::catalog::item_category::ItemCategory;
use crate::error::Result;
use crate::AppState;
use crate::{adapters::outgoing::graphql::Query, core::command::catalog::item_category::*};
use juniper::{DefaultScalarValue, EmptyMutation, EmptySubscription, ExecutionError, Variables};
use tauri::State;

#[tauri::command]
pub fn graphql(
    query: String,
    state: State<'_, AppState>,
) -> Result<(juniper::Value, Vec<ExecutionError<DefaultScalarValue>>)> {
    juniper::execute_sync(
        &query,
        None,
        &Schema::new(Query, EmptyMutation::new(), EmptySubscription::new()),
        &Variables::new(),
        &state,
    )
    .map_err(|err| err.into())
}

#[tauri::command]
pub fn create_item_category(
    name: String,
    description: Option<String>,
    state: State<'_, AppState>,
) -> Result<ItemCategory> {
    let mut service = state.service.lock().unwrap();
    CreateItemCategoryCommand { name, description }.exec(&mut service)
}

#[tauri::command]
pub fn update_item_category(
    category: ItemCategory,
    state: State<'_, AppState>,
) -> Result<ItemCategory> {
    let mut service = state.service.lock().unwrap();
    UpdateItemCategoryCommand { category }.exec(&mut service)
}

#[tauri::command]
pub fn delete_item_category(category: ItemCategory, state: State<'_, AppState>) -> Result<()> {
    let mut service = state.service.lock().unwrap();
    DeleteItemCategoryCommand { category }.exec(&mut service)
}
