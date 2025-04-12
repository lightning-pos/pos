use std::sync::{Arc, Mutex};
use std::time::Duration;

use libsql::replication::Replicated;

/// Service for handling Turso database synchronization
pub struct TursoSyncService {
    pub db: libsql::Database,
    pub conn: libsql::Connection,
    pub sync_interval: Duration,
}

impl TursoSyncService {
    /// Create a new TursoSyncService
    pub async fn new(db_path: &str, turso_url: &str, turso_token: &str) -> Result<Self, libsql::Error> {
        // Create a database with sync capabilities
        let db = libsql::Builder::new_remote_replica(
                db_path.to_string(),
                turso_url.to_string(),
                turso_token.to_string()
            )
            .sync_interval(Duration::from_secs(300)) // 5 minutes default sync interval
            .build()
            .await?;

        // Create a connection to the database
        let conn = db.connect()?;

        Ok(Self {
            db,
            conn,
            sync_interval: Duration::from_secs(300),
        })
    }

    /// Manually trigger a sync with Turso cloud
    pub async fn sync(&self) -> Result<Replicated, libsql::Error> {
        self.db.sync().await
    }

    /// Run the background sync service
    pub async fn run_sync_service(service: Arc<Mutex<Self>>) {
        use tokio::time::interval;

        let sync_interval = {
            let service = service.lock().unwrap();
            service.sync_interval
        };

        let mut interval = interval(sync_interval);
        loop {
            interval.tick().await;

            // Clone the Arc to avoid holding the MutexGuard across an await point
            let service_clone = Arc::clone(&service);

            // Use a different approach to avoid lifetime issues
            // Create a channel to communicate between threads
            let (tx, rx) = std::sync::mpsc::channel();

            // Spawn a thread to handle the sync operation
            std::thread::spawn(move || {
                // This runs in a separate thread, so we can safely hold the lock
                let service = service_clone.lock().unwrap();

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

            // Handle the result
            if let Err(e) = result {
                eprintln!("Turso sync error: {}", e);
            }
        }
    }
}
