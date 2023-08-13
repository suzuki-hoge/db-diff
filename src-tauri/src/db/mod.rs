use std::path::Path;

use anyhow::anyhow;
use diesel::{Connection, SqliteConnection};
use diesel_migrations::run_pending_migrations;

pub mod diff;
pub mod project;
mod schema;
pub mod snapshot;

pub fn migrate_sqlite_if_missing() -> anyhow::Result<()> {
    if !Path::new("database.sqlite").is_file() {
        run_pending_migrations(&create_sqlite_connection()?)?;
    }
    Ok(())
}

pub fn create_sqlite_connection() -> anyhow::Result<SqliteConnection> {
    SqliteConnection::establish("database.sqlite").map_err(|e| anyhow!(e))
}
