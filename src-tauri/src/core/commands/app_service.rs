use diesel::{sqlite::Sqlite, Connection, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::error::Error;

use crate::{
    adapters::outgoing::database::{DatabaseAdapter, SqlxAdapter},
    core::types::db_uuid::DbUuid,
};

pub struct AppService<DB: DatabaseAdapter = SqlxAdapter> {
    pub conn: SqliteConnection,
    pub libsql_conn: Option<libsql::Connection>,
    pub db_adapter: DB,
    pub state: SessionState,
}

pub struct SessionState {
    pub current_user: Option<DbUuid>,
}

impl AppService {
    pub fn new(conn_path: &str) -> Self {
        // Create a synchronous runtime to run async code in a sync function
        let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
        let state = SessionState { current_user: None };
        let mut conn = SqliteConnection::establish(conn_path).unwrap();
        let migration_result = Self::run_migrations(&mut conn);

        match migration_result {
            Ok(_) => println!("Migration successful"),
            Err(e) => {
                eprintln!("Migration failed: {}", e);
                panic!("Failed to run database migrations: {}", e);
            }
        }

        // Create a temporary libsql database and connection for initialization
        // This will be replaced with the proper synced connection in init_libsql_db
        let db = rt.block_on(async {
            libsql::Builder::new_local(conn_path)
                .build()
                .await
                .expect("Failed to build local libsql database")
        });

        let libsql_conn = db.connect()
            .expect("Failed to connect to local libsql database");

        // Create the SqlxAdapter with the temporary connection
        let db_adapter = SqlxAdapter::new(libsql_conn);

        Self {
            conn,
            libsql_conn: None,
            db_adapter,
            state,
        }
    }

    pub async fn init_libsql_db(&mut self, conn_path: &str) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        // Get Turso URL and token from environment variables
        let url = std::env::var("TURSO_URL")
            .map_err(|_| "TURSO_URL environment variable not set")?;
        let token = std::env::var("TURSO_TOKEN")
            .map_err(|_| "TURSO_TOKEN environment variable not set")?;

        // Build the synced database
        let libsql_db = libsql::Builder::new_remote_replica(
            conn_path,
            url,
            token
        ).build().await
         .map_err(|e| format!("Failed to build synced database: {}", e))?;

        // Connect to the database
        let libsql_conn = libsql_db.connect()
            .map_err(|e| format!("Failed to connect to libsql database: {}", e))?;

        // Store the connection
        self.libsql_conn = Some(libsql_conn.clone());

        // Update the database adapter with the synced connection
        // We need to create a new adapter since we can't modify the existing one
        self.db_adapter = SqlxAdapter::new(libsql_conn);

        Ok(())
    }

    fn run_migrations(
        connection: &mut impl MigrationHarness<Sqlite>,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

        connection.run_pending_migrations(MIGRATIONS)?;

        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn setup_service() -> AppService {
        AppService::new(":memory:")
    }
}
