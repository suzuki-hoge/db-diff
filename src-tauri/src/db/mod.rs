use std::fs::{create_dir_all, OpenOptions};
use std::io::{stdout, Write};

use anyhow::anyhow;
use diesel::{Connection, SqliteConnection};
use diesel_migrations::run_pending_migrations_in_directory;

use crate::logger;
use crate::workspace::{workspace_dir, workspace_path};

pub mod diff;
pub mod project;
mod schema;
pub mod snapshot;

pub fn migrate_sqlite_if_missing() -> anyhow::Result<()> {
    setup_dir()?;

    let db = workspace_path("database-v0.1.0.sqlite")?;

    if !db.exists() {
        logger::info("create database [ ~/.db-diff/database-v0.1.0.sqlite ]");

        let conn = create_sqlite_connection()?;

        logger::info("put migration queries [ ~/.db-diff/migrations ]");

        put_migrations("up", include_bytes!("../../migrations/tables/up.sql"))?;
        put_migrations("down", include_bytes!("../../migrations/tables/down.sql"))?;

        logger::info("migrate database");

        run_pending_migrations_in_directory(&conn, &workspace_path("migrations")?, &mut stdout())?;

        logger::info("migrate ok");
    } else {
        logger::info("found database [ ~/.db-diff/database-v0.1.0.sqlite ]");
    }

    Ok(())
}

pub fn create_sqlite_connection() -> anyhow::Result<SqliteConnection> {
    let db = workspace_path("database-v0.1.0.sqlite")?;

    SqliteConnection::establish(db.to_str().unwrap()).map_err(|e| anyhow!(e))
}

fn setup_dir() -> anyhow::Result<()> {
    let path = workspace_dir()?;

    if !path.exists() {
        create_dir_all(path).map_err(|e| anyhow!(e))?;
        logger::info("setup workspace [ ~/.db-diff ]");
    } else {
        logger::info("found workspace [ ~/.db-diff ]");
    }

    Ok(())
}

fn put_migrations(kind: &str, bytes: &[u8]) -> anyhow::Result<()> {
    let dir = workspace_path("migrations/tables")?;
    create_dir_all(dir)?;

    let path = workspace_path(&format!("migrations/tables/{kind}.sql"))?;
    let mut file = OpenOptions::new().append(true).create(true).open(path)?;

    file.write_all(bytes).map_err(|e| anyhow!(e))?;
    file.flush().map_err(|e| anyhow!(e))
}
