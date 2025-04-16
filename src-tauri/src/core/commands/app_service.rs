use crate::{
    adapters::outgoing::database::{DatabaseAdapter, SqlxAdapter},
    core::types::db_uuid::DbUuid,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub struct AppService<DB: DatabaseAdapter = SqlxAdapter> {
    pub db_adapter: DB,
    pub state: SessionState,
}

pub struct SessionState {
    pub current_user: Option<DbUuid>,
}

impl AppService {
    pub fn new(conn_path: &str) -> Self {
        let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
        let state = SessionState { current_user: None };
        let conn = rt.block_on(async {
            let turso_url = std::env::var("TURSO_URL").expect("Failed to get TURSO_URL");
            let turso_token = std::env::var("TURSO_TOKEN").expect("Failed to get TURSO_TOKEN");

            let db = libsql::Builder::new_synced_database(conn_path, turso_url, turso_token)
                    .build()
                    .await
                    .expect("Failed to build synced libsql database");

            db.connect().expect("Failed to connect to libsql database")
        });

        let db_adapter = SqlxAdapter::new(conn);

        Self {
            db_adapter,
            state,
        }
    }


}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn setup_service() -> AppService {
        // Create a simple in-memory database for tests
        let test_db_path = ":memory:";

        // Create a runtime for async operations
        let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");

        // Create the database and get a connection
        let db = rt.block_on(async {
            libsql::Builder::new_local(test_db_path)
                // .skip_saftey_assert(true)
                .build()
                .await
                .expect("Failed to build in-memory libsql database for tests")
        });

        let conn = rt.block_on(async {
            db.connect().expect("Failed to connect to libsql database")
        });

        // Run migrations using a simple approach that avoids threading issues
        rt.block_on(async {
            println!("Running migrations for test database");

            // Define migration directories in order
            let migration_dirs = [
                "migrations/2025-04-10-000000_consolidated_schema",
                "migrations/2025-04-12-165429_fix_foreign_key_constraints",
                "migrations/2025-04-12-170507_make_sales_order_user_fields_required",
            ];

            // Execute each migration file
            for dir in migration_dirs.iter() {
                let up_sql_path = format!("{}/up.sql", dir);
                println!("Applying migration: {}", up_sql_path);

                // Read the SQL file content
                let sql = match std::fs::read_to_string(&up_sql_path) {
                    Ok(content) => content,
                    Err(e) => {
                        eprintln!("Failed to read migration file {}: {}", up_sql_path, e);
                        panic!("Failed to read migration file: {}", e);
                    }
                };

                // Execute the entire SQL file at once
                // This handles complex SQL syntax like triggers with BEGIN/END blocks
                match conn.execute(&sql, Vec::<libsql::Value>::new()).await {
                    Ok(_) => println!("Successfully applied migration: {}", up_sql_path),
                    Err(e) => {
                        eprintln!("Failed to apply migration {}: {}", up_sql_path, e);
                        panic!("Failed to apply migration: {}", e);
                    }
                }
            }

            println!("All migrations applied successfully");
        });

        // Create the AppService with the initialized database
        AppService {
            db_adapter: SqlxAdapter::new(conn),
            state: SessionState { current_user: None },
        }
    }
}
