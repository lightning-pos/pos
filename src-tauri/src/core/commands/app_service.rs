use diesel::{sqlite::Sqlite, Connection, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::error::Error;

use crate::core::types::db_uuid::DbUuid;

pub struct AppService {
    pub conn: SqliteConnection,
    pub libsql_conn: Option<libsql::Connection>,
    pub state: SessionState,
}

pub struct SessionState {
    pub current_user: Option<DbUuid>,
}

impl AppService {
    pub fn new(conn_path: &str) -> Self {
        let state = SessionState { current_user: None };
        let mut conn = SqliteConnection::establish(conn_path).unwrap();
        let migration_result = Self::run_migrations(&mut conn);

        match migration_result {
            Ok(_) => println!("Migration successful"),
            Err(e) => println!("Migration failed: {}", e),
        }

        Self {
            conn,
            libsql_conn: None,
            state,
        }
    }

    pub async fn init_libsql_db(&mut self, conn_path: &str) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let url = std::env::var("TURSO_URL").ok().unwrap();
        let token = std::env::var("TURSO_TOKEN").ok().unwrap();

        let libsql_db = libsql::Builder::new_synced_database(
            conn_path.to_string(),
            url.to_string(),
            token.to_string()
        ).build().await.unwrap();

        let libsql_conn = libsql_db.connect().unwrap();

        self.libsql_conn = Some(libsql_conn);
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
