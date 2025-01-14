// use crate::core::common::interface::sql::SQLInterface;

use std::error::Error;

use diesel::{sqlite::Sqlite, Connection, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub struct AppService {
    pub conn: SqliteConnection,
}

impl AppService {
    pub fn new(conn: &str) -> Self {
        let mut conn = SqliteConnection::establish(conn).unwrap();
        let migration_result = Self::run_migrations(&mut conn);

        match migration_result {
            Ok(_) => println!("Migration successful"),
            Err(e) => println!("Migration failed: {}", e),
        }
        Self { conn }
    }

    fn run_migrations(
        connection: &mut impl MigrationHarness<Sqlite>,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

        connection.run_pending_migrations(MIGRATIONS)?;

        Ok(())
    }
}
