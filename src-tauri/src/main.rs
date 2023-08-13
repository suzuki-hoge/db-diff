#[macro_use]
extern crate diesel;

use tauri::Manager;

use crate::command::state::AppState;
use crate::db::migrate_sqlite_if_missing;

mod command;
mod db;
mod domain;
mod dump;

fn main() -> anyhow::Result<()> {
    migrate_sqlite_if_missing()?;

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            command::project::all_projects_command,
            command::project::test_connection_project_command,
            command::project::insert_project_command,
            command::project::update_project_command,
            command::project::delete_project_command,
            command::project::select_project_command,
            command::snapshot::all_snapshot_summaries_command,
            command::snapshot::update_snapshot_summary_command,
            command::snapshot::delete_snapshot_summary_command,
            command::snapshot::dump_snapshot_command,
            command::diff::find_snapshot_diff_command,
            command::diff::create_snapshot_diff_command,
        ])
        .setup(|app| {
            let state = AppState::new()?;
            app.manage(state);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
