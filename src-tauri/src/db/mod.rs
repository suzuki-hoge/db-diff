use std::fs::{create_dir_all, remove_dir_all, OpenOptions};
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

    let db = workspace_path("database-v0.1.1.sqlite")?;

    if !db.exists() {
        logger::info("create database [ ~/.db-diff/database-v0.1.1.sqlite ]");

        let conn = create_sqlite_connection()?;

        logger::info("put ( or replace ) migration queries [ ~/.db-diff/migrations ]");

        let migrations_dir = workspace_path("migrations")?;

        let _ = remove_dir_all(&migrations_dir);

        put_migrations("up", get_up_bytes())?;
        put_migrations("down", get_down_bytes())?;

        logger::info("migrate database");

        run_pending_migrations_in_directory(&conn, &migrations_dir, &mut stdout())?;

        logger::info("migrate ok");
    } else {
        logger::info("found database [ ~/.db-diff/database-v0.1.1.sqlite ]");
    }

    Ok(())
}

pub fn create_sqlite_connection() -> anyhow::Result<SqliteConnection> {
    let db = workspace_path("database-v0.1.1.sqlite")?;

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
    let dir = workspace_path("migrations")?.join("tables");
    create_dir_all(&dir)?;

    let path = dir.join(format!("{kind}.sql"));
    let mut file = OpenOptions::new().append(true).create(true).open(path)?;

    file.write_all(bytes).map_err(|e| anyhow!(e))?;
    file.flush().map_err(|e| anyhow!(e))
}

#[cfg(unix)]
fn get_up_bytes() -> &'static [u8] {
    include_bytes!("../../migrations/tables/up.sql")
}

#[cfg(unix)]
fn get_down_bytes() -> &'static [u8] {
    include_bytes!("../../migrations/tables/down.sql")
}

#[cfg(windows)]
fn get_up_bytes() -> &'static [u8] {
    include_bytes!("..\\..\\migrations\\tables\\up.sql")
}

#[cfg(windows)]
fn get_down_bytes() -> &'static [u8] {
    include_bytes!("..\\..\\migrations\\tables\\down.sql")
}
