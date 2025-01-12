// use crate::core::common::interface::sql::SQLInterface;

use diesel::SqliteConnection;

use crate::adapters::outgoing::database::sqlite_adapter::SQLiteAdapter;

pub struct AppService {
    pub conn: SqliteConnection,
}

impl AppService {
    pub fn new(conn: &str) -> Self {
        Self {
            conn: SQLiteAdapter::new(conn).unwrap(),
        }
    }
}
