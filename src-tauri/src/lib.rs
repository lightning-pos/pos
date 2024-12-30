mod adapters;
mod core;

use tauri_plugin_sql::{Migration, MigrationKind};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let migrations = vec![
        Migration {
            version: 1,
            description: "Initial migration",
            sql: include_str!("../migrations/0000_wise_ultron.sql"),
            kind: MigrationKind::Up,
        },
        Migration {
            version: 2,
            description: "Add cascade delete for item taxes",
            sql: include_str!("../migrations/0001_curious_mentallo.sql"),
            kind: MigrationKind::Up,
        },
        Migration {
            version: 3,
            description: "Add purchase orders",
            sql: include_str!("../migrations/0002_normal_karnak.sql"),
            kind: MigrationKind::Up,
        },
    ];

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:minnal.db", migrations)
                .build(),
        )
        .plugin(tauri_plugin_fs::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
