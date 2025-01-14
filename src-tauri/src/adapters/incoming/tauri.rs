use crate::core::command::catalog::item_category::*;
use crate::core::command::Command;
use crate::core::entities::catalog::item_category::ItemCategory;
use crate::error::Result;
use crate::AppState;
use tauri::State;

#[tauri::command]
async fn query(query: serde_json::Value, state: State<'_, AppState>) -> Result<()> {
    todo!()
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
