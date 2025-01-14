pub mod adapters;
pub mod core;
pub mod error;
pub mod schema;

use adapters::incoming::tauri::*;
use core::command::app_service::AppService;
use std::sync::Mutex;
use tauri_plugin_sql::{Migration, MigrationKind};

pub struct AppState {
    pub service: Mutex<AppService>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let migrations = vec![Migration {
        version: 1,
        description: "Initial migration",
        sql: include_str!("../migrations/2025-01-14-043805_create_catalog_tables/up.sql"),
        kind: MigrationKind::Up,
    }];

    tauri::Builder::default()
        .manage(AppState {
            service: Mutex::new(AppService::new("minnal.db")),
        })
        .invoke_handler(tauri::generate_handler![
            create_item_category,
            update_item_category,
            delete_item_category,
        ])
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:minnal.db", migrations)
                .build(),
        )
        .plugin(tauri_plugin_fs::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
