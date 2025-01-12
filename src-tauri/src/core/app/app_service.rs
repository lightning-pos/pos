// use crate::core::common::interface::sql::SQLInterface;

use diesel::{Connection, RunQueryDsl, SqliteConnection};

pub struct AppService {
    pub conn: SqliteConnection,
}

impl AppService {
    pub fn new(conn: &str) -> Self {
        let mut conn = SqliteConnection::establish(conn).unwrap();
        Self::initialize_schema(&mut conn);
        Self { conn }
    }

    fn initialize_schema(conn: &mut SqliteConnection) {
        // Create item_category table
        let item_category = String::from(
            "CREATE TABLE IF NOT EXISTS item_categories (
            id TEXT NOT NULL PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            state TEXT NOT NULL,
            description TEXT,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        )",
        );

        let _ = diesel::sql_query(item_category).execute(conn);

        // Create item table
        let item = String::from(
            "CREATE TABLE IF NOT EXISTS items (
            id TEXT NOT NULL PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT,
            nature TEXT NOT NULL,
            category_id TEXT NOT NULL,
            state TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            FOREIGN KEY (category_id) REFERENCES item_categories (id)
        )",
        );

        let _ = diesel::sql_query(item).execute(conn);
    }
}
