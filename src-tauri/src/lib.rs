pub mod adapters;
pub mod core;
pub mod error;
pub mod schema;

use adapters::incoming::tauri::*;
use tauri::Manager;
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
    let app_service = AppService::new(db_path);

    // Create the app state with the service
    let app_state = AppState {
        service: Mutex::new(app_service),
    };

    // Build and run the Tauri application
    let app = tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![graphql])
        .plugin(tauri_plugin_fs::init())
        .setup(move |app| {
            // Initialize Turso sync if environment variables are set
            let turso_url = std::env::var("TURSO_URL").ok();
            let turso_token = std::env::var("TURSO_TOKEN").ok();

            println!("TURSO_URL: {:?}", turso_url);
            println!("TURSO_TOKEN: {:?}", turso_token);

            if let (Some(url), Some(token)) = (turso_url, turso_token) {
                // Get a handle to the app state
                let app_handle = app.handle().clone();

                // Spawn a task to initialize Turso sync
                // Create a thread to handle the initialization
                // This approach avoids lifetime issues with app_handle
                let db_path_str = db_path.to_string();
                let url_str = url.to_string();
                let token_str = token.to_string();

                std::thread::spawn(move || {
                    // Get the app state
                    let state_result = app_handle.try_state::<AppState>();

                    if let Some(state) = state_result {
                        // Create a runtime for this thread
                        let rt = tokio::runtime::Runtime::new().unwrap();

                        // Run the initialization in this thread's runtime
                        let result = rt.block_on(async {
                            // Acquire the lock in a separate scope
                            let mut service = state.service.lock().unwrap();

                            // Initialize Turso sync
                            service.init_turso_sync(&db_path_str, &url_str, &token_str).await
                        });

                        // Log the result
                        match result {
                            Ok(_) => println!("Turso sync initialized successfully"),
                            Err(e) => eprintln!("Failed to initialize Turso sync: {}", e),
                        }
                    } else {
                        eprintln!("Failed to get application state");
                    }
                });
            } else {
                println!("Turso sync not configured. Set TURSO_URL and TURSO_TOKEN environment variables to enable sync.");
            }

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(|_, _| {});
}
