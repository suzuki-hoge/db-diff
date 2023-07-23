use anyhow::anyhow;
use diesel::{Connection, SqliteConnection};

pub mod diff;
pub mod project;
mod schema;
pub mod snapshot;

pub fn create_connection() -> anyhow::Result<SqliteConnection> {
    SqliteConnection::establish("database.sqlite").map_err(|e| anyhow!(e))
}
