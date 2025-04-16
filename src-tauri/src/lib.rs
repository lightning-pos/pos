pub mod adapters;
pub mod core;
pub mod error;
pub mod schema;

use core::commands::app_service::AppService;
use std::sync::Mutex;

pub struct AppState {
    pub service: Mutex<AppService>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize the database path
    let db_path = "minnal.db";

    // Create the app service - it will automatically use Turso if credentials are available
    let app_service = AppService::new(db_path);

    // Create the app state with the service
    let app_state = AppState {
        service: app_service.into(),
    };

    // Build and run the Tauri application
    let app = tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            crate::adapters::incoming::tauri::graphql,
            crate::adapters::incoming::tauri::schema
        ])
        .plugin(tauri_plugin_fs::init())
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(|_, _| {});
}
