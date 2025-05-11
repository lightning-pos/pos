---
trigger: model_decision
description: 
globs: src-tauri/migrations/**/*.sql
---
The migrations uses only SQLite. All SQL query should be compatible with SQLite.

Never create seed data. It will be created manually by the developers if absolutely required.

To generate a migration files use `diesel migration generate <migration_name>` from `${workspaceRoot}/src-tauri` folder. Never create the migration folders yourself.