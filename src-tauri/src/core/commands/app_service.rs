use crate::{
    adapters::outgoing::database::{DatabaseAdapter, LibSqlAdapter},
    core::types::db_uuid::DbUuid,
};

pub struct AppService<DB: DatabaseAdapter = LibSqlAdapter> {
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

        let db_adapter = LibSqlAdapter::new(conn);

        Self {
            db_adapter,
            state,
        }
    }


}

#[cfg(test)]
pub mod tests {
    use super::*;
    use once_cell::sync::Lazy;
    use std::time::Instant;
    use std::sync::{Arc, Mutex};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::path::Path;
    use std::fs;

    // Counter for unique database file names
    static DB_COUNTER: AtomicUsize = AtomicUsize::new(0);

    // Embed SQL migrations at compile time using include_str!
    static CONSOLIDATED_SCHEMA: &str = include_str!("../../../migrations/2025-04-10-000000_consolidated_schema/up.sql");
    static FIX_FOREIGN_KEY_CONSTRAINTS: &str = include_str!("../../../migrations/2025-04-12-165429_fix_foreign_key_constraints/up.sql");
    static MAKE_SALES_ORDER_USER_FIELDS_REQUIRED: &str = include_str!("../../../migrations/2025-04-12-170507_make_sales_order_user_fields_required/up.sql");

    // Combined SQL for faster execution - combined at compile time
    static COMBINED_MIGRATION_SQL: Lazy<String> = Lazy::new(|| {
        format!("{CONSOLIDATED_SCHEMA}\n{FIX_FOREIGN_KEY_CONSTRAINTS}\n{MAKE_SALES_ORDER_USER_FIELDS_REQUIRED}")
    });

    // Template database with migrations already applied
    static TEMPLATE_DB: Lazy<Arc<Mutex<Option<String>>>> = Lazy::new(|| {
        Arc::new(Mutex::new(None))
    });

    // Initialize template database once
    async fn get_or_create_template_db() -> String {
        let mut template = TEMPLATE_DB.lock().unwrap();

        if let Some(template_path) = template.as_ref() {
            return template_path.clone();
        }

        // Create a temporary directory for test databases if it doesn't exist
        let temp_dir = Path::new("./temp_test_db");
        if !temp_dir.exists() {
            fs::create_dir_all(temp_dir).expect("Failed to create temp directory for test databases");
        }

        // Create template database file
        let template_path = temp_dir.join("template.db").to_str().unwrap().to_string();

        // Remove existing file if it exists
        if Path::new(&template_path).exists() {
            fs::remove_file(&template_path).expect("Failed to remove existing template database");
        }

        // Create and initialize the template database
        let db = libsql::Builder::new_local(&template_path)
            .build()
            .await
            .expect("Failed to build template database");

        let conn = db.connect().expect("Failed to connect to template database");

        // Apply migrations to the template
        if let Err(e) = conn.execute_batch(&COMBINED_MIGRATION_SQL).await {
            panic!("Failed to apply migrations to template: {}", e);
        }

        // Close connection to ensure file is properly written
        drop(conn);
        drop(db);

        // Store template path
        *template = Some(template_path.clone());

        template_path
    }

    pub async fn setup_service() -> AppService {

        // Get or create template database
        let template_path = get_or_create_template_db().await;

        // Create a unique database file for this test
        let test_id = DB_COUNTER.fetch_add(1, Ordering::SeqCst);
        let temp_dir = Path::new("./temp_test_db");
        let test_db_path = temp_dir.join(format!("test_{}.db", test_id)).to_str().unwrap().to_string();

        // Copy template database to test database
        fs::copy(&template_path, &test_db_path).expect("Failed to copy template database");

        // Open the copied database
        let db = libsql::Builder::new_local(&test_db_path)
            .build()
            .await
            .expect("Failed to build test database from template");

        let conn = db.connect().expect("Failed to connect to test database");

        // Create AppService with the initialized database
        let service = AppService {
            db_adapter: LibSqlAdapter::new(conn),
            state: SessionState { current_user: None },
        };

        service
    }

    // Clean up test databases after all tests have run
    #[ctor::dtor]
    fn cleanup_test_databases() {
        let temp_dir = Path::new("./temp_test_db");
        if temp_dir.exists() {
            let _ = fs::remove_dir_all(temp_dir);
        }
    }
}
