use diesel::{sqlite::Sqlite, Connection, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::error::Error;

use crate::core::types::db_uuid::DbUuid;

pub struct AppService {
    pub conn: SqliteConnection,
    pub state: SessionState,
}

pub struct SessionState {
    pub current_user: Option<DbUuid>,
}

impl AppService {
    pub fn new(conn: &str) -> Self {
        let state = SessionState { current_user: None };
        let mut conn = SqliteConnection::establish(conn).unwrap();
        let migration_result = Self::run_migrations(&mut conn);

        match migration_result {
            Ok(_) => println!("Migration successful"),
            Err(e) => println!("Migration failed: {}", e),
        }
        Self { conn, state }
    }

    fn run_migrations(
        connection: &mut impl MigrationHarness<Sqlite>,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

        connection.run_pending_migrations(MIGRATIONS)?;

        Ok(())
    }
}
