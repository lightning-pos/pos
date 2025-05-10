use libsql::Connection;

use crate::{
    adapters::outgoing::database::{DatabaseAdapter, LibSqlAdapter},
    core::{db::migrations, types::db_uuid::DbUuid},
};

pub struct AppService<DB: DatabaseAdapter = LibSqlAdapter> {
    pub db_adapter: DB,
    pub state: SessionState,
}

pub struct SessionState {
    pub current_user: Option<DbUuid>,
}

impl AppService {
    pub async fn new(conn_path: &str) -> Self {
        let state = SessionState { current_user: None };
        let turso_url = std::env::var("TURSO_URL").expect("Failed to get TURSO_URL");
        let turso_token = std::env::var("TURSO_TOKEN").expect("Failed to get TURSO_TOKEN");

        let db = libsql::Builder::new_synced_database(conn_path, turso_url, turso_token)
                .build()
                .await
                .expect("Failed to build synced libsql database");

        let conn = db.connect().expect("Failed to connect to libsql database");

        Self::apply_migrations(&conn).await;

        let db_adapter = LibSqlAdapter::new(db, conn);

        Self {
            db_adapter,
            state,
        }
    }


    #[cfg(test)]
    pub async fn new_test(conn_path: &str) -> Self {
        let db = libsql::Builder::new_local(&conn_path)
            .build()
            .await
            .expect("Failed to build test database from template");

        let conn = db.connect().expect("Failed to connect to test database");

        Self::apply_migrations(&conn).await;

        let db_adapter = LibSqlAdapter::new(db, conn);

        Self {
            db_adapter,
            state: SessionState { current_user: None },
        }
    }

    pub async fn apply_migrations(conn: &Connection) {
        // Run migrations using our custom implementation
        match migrations::run_migrations(conn).await {
            Ok(applied) => {
                if applied {
                    eprintln!("Migrations applied successfully");
                } else {
                    eprintln!("No new migrations to apply");
                }
            }
            Err(e) => {
                eprintln!("Migration failed: {}", e);
                panic!()
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub async fn setup_service() -> AppService {
        AppService::new_test(":memory:").await
    }
}
