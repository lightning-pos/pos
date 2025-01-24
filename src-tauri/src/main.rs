// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use lightning_pos::adapters;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "export-schema" {
        export_schema();
        return;
    }
    lightning_pos::run();
}

fn export_schema() {
    let schema = adapters::incoming::tauri::schema();
    std::fs::write("../src-ui/schema.graphql", schema).expect("Failed to write schema file");
    println!("Schema exported successfully!");
}
