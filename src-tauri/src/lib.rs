pub mod adapters;
pub mod core;
pub mod error;
pub mod schema;

use adapters::incoming::tauri::*;
use core::command::app_service::AppService;
use std::sync::Mutex;

pub struct AppState {
    pub service: Mutex<AppService>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            service: Mutex::new(AppService::new("minnal.db")),
        })
        .invoke_handler(tauri::generate_handler![graphql])
        .plugin(tauri_plugin_fs::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
