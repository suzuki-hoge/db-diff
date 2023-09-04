use itertools::Itertools;
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::command::state::AppState;
use crate::db::dump_config::{find_dump_config, find_recent_dump_configs};
use crate::db::project::all_projects;

use crate::domain::dump_config::DumpConfig;

use crate::domain::schema::{ColName, TableName};
use crate::domain::snapshot::SnapshotId;
use crate::dump::get_dump_configs;
use crate::logger;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DumpConfigJson {
    pub table_name: TableName,
    pub col_names: Vec<ColName>,
    pub value: String,
}

impl DumpConfigJson {
    fn from(dump_config: DumpConfig) -> Self {
        Self { table_name: dump_config.table_name, col_names: dump_config.col_names, value: dump_config.value }
    }

    pub fn into(self) -> DumpConfig {
        DumpConfig::new(self.table_name, self.col_names, self.value)
    }
}

#[tauri::command]
pub async fn find_recent_dump_configs_command(app_state: State<'_, AppState>) -> Result<Vec<DumpConfigJson>, String> {
    logger::info("start find_recent_dump_configs_command");

    let conn = app_state.conn.lock().unwrap();
    let project_id = app_state.project_id.lock().unwrap();
    let project_id = project_id.as_ref().unwrap();
    let projects = all_projects(&conn).map_err(|e| e.to_string())?;
    let project = projects.iter().find(|project| &project.project_id == project_id).unwrap();

    let dump_configs = get_dump_configs(project).map_err(|e| e.to_string())?;

    let x = match find_recent_dump_configs(&conn, project_id).map_err(|e| e.to_string())? {
        Some(x) => DumpConfig::merge(dump_configs, x),
        None => dump_configs,
    };

    logger::info("end   find_recent_dump_configs_command");
    Ok(DumpConfig::sort(x).into_iter().map(DumpConfigJson::from).collect_vec())
}

#[tauri::command]
pub async fn find_dump_configs_command(app_state: State<'_, AppState>, snapshot_id: SnapshotId) -> Result<Vec<DumpConfigJson>, String> {
    logger::info("start find_dump_configs_command");

    let conn = app_state.conn.lock().unwrap();

    let x = find_dump_config(&conn, &snapshot_id)
        .map(|dump_configs| dump_configs.into_iter().map(DumpConfigJson::from).collect_vec())
        .map_err(|e| e.to_string());

    logger::info("end   find_dump_configs_command");
    x
}
