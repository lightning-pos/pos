use diesel::{sqlite::Sqlite, Connection, SqliteConnection, RunQueryDsl};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::error::Error;
use std::sync::{Arc, Mutex};

use crate::core::commands::sync_service::TursoSyncService;

use crate::core::types::db_uuid::DbUuid;

pub struct AppService {
    pub conn: SqliteConnection,
    pub state: SessionState,
    pub turso_sync: Option<Arc<Mutex<TursoSyncService>>>,
}

// TursoSyncService moved to sync_service.rs

pub struct SessionState {
    pub current_user: Option<DbUuid>,
}

impl AppService {
    pub fn new(conn_path: &str) -> Self {
        let state = SessionState { current_user: None };
        let mut conn = SqliteConnection::establish(conn_path).unwrap();

        // Enable WAL mode for better concurrency between Diesel and Turso
        diesel::sql_query("PRAGMA journal_mode = WAL;").execute(&mut conn).unwrap();

        let migration_result = Self::run_migrations(&mut conn);

        match migration_result {
            Ok(_) => println!("Migration successful"),
            Err(e) => println!("Migration failed: {}", e),
        }
        Self { conn, state, turso_sync: None }
    }

    fn run_migrations(
        connection: &mut impl MigrationHarness<Sqlite>,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

        connection.run_pending_migrations(MIGRATIONS)?;

        Ok(())
    }

    /// Initialize Turso sync service and attach it to this AppService
    pub async fn init_turso_sync(&mut self, db_path: &str, turso_url: &str, turso_token: &str) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        match TursoSyncService::new(db_path, turso_url, turso_token).await {
            Ok(sync_service) => {
                let sync_service = Arc::new(Mutex::new(sync_service));

                // Start the background sync service
                let sync_service_clone = Arc::clone(&sync_service);

                // Spawn a task that doesn't hold MutexGuard across await points
                tokio::task::spawn(async move {
                    TursoSyncService::run_sync_service(sync_service_clone).await;
                });

                self.turso_sync = Some(sync_service);
                Ok(())
            },
            Err(e) => Err(Box::new(e)),
        }
    }

    /// Manually trigger a sync with Turso
    pub async fn sync_turso(&self) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        if let Some(sync_service) = &self.turso_sync {
            // Use a different approach to avoid lifetime issues
            // Create a channel to communicate between threads
            let (tx, rx) = std::sync::mpsc::channel();

            // Clone the Arc to avoid moving it
            let sync_service_clone = Arc::clone(sync_service);

            // Spawn a thread to handle the sync operation
            std::thread::spawn(move || {
                // This runs in a separate thread, so we can safely hold the lock
                let service = sync_service_clone.lock().unwrap();

                // Create a runtime for this thread
                let rt = tokio::runtime::Runtime::new().unwrap();

                // Run the sync operation in this thread's runtime
                let result = rt.block_on(async {
                    service.db.sync().await
                });

                // Send the result back to the main thread
                let _ = tx.send(result);
            });

            // Receive the result from the other thread
            let result = match rx.recv() {
                Ok(res) => res,
                Err(_) => {
                    // Since we can't easily create a libsql::Error, let's create a dummy sync operation
                    // that will fail in a controlled way
                    let dummy_db = libsql::Builder::new_local(":memory:").build().await.unwrap();
                    // This will fail because we're not connected to any remote
                    dummy_db.sync().await
                }
            };

            // Process the result
            result
                .map(|_| ()) // Discard the Replicated value
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync + 'static>)
        } else {
            Err("Turso sync service not initialized".into())
        }
    }
}

// TursoSyncService implementation moved to sync_service.rs

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn setup_service() -> AppService {
        AppService::new(":memory:")
    }
}
