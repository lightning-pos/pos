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

    // Create the app service
    let mut app_service = AppService::new(db_path);

    // Initialize libsql_db synchronously using a blocking runtime
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        // Initialize libsql_db and panic if it fails
        if let Err(e) = app_service.init_libsql_db(db_path).await {
            eprintln!("Failed to initialize libsql_db: {}", e);
            // We don't panic here because the database adapter is already initialized in AppService::new
            // The libsql connection is optional, but the SQLx adapter is required
        }
    });

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
