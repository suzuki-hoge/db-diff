#[macro_use]
extern crate diesel;

use tauri::Manager;

use crate::command::state::AppState;
use crate::db::migrate_sqlite_if_missing;

mod command;
mod db;
mod domain;
mod dump;
mod logger;
mod workspace;

fn main() -> anyhow::Result<()> {
    match tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            command::project::all_projects_command,
            command::project::test_connection_project_command,
            command::project::insert_project_command,
            command::project::update_project_command,
            command::project::delete_project_command,
            command::project::select_project_command,
            command::dump_config::find_recent_dump_configs_command,
            command::dump_config::find_dump_configs_command,
            command::snapshot::all_snapshot_summaries_command,
            command::snapshot::update_snapshot_summary_command,
            command::snapshot::delete_snapshot_summary_command,
            command::snapshot::dump_snapshot_command,
            command::snapshot::get_snapshot_result_command,
            command::diff::find_snapshot_diff_command,
            command::diff::create_snapshot_diff_command,
        ])
        .setup(|app| {
            migrate_sqlite_if_missing()?;

            let state = AppState::new()?;
            app.manage(state);

            Ok(())
        })
        .run(tauri::generate_context!())
    {
        Ok(_) => {}
        Err(e) => logger::error(e.to_string()),
    };

    Ok(())
}
