use itertools::Itertools;
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::command::dump_config::DumpConfigJson;
use crate::command::state::AppState;
use crate::db::dump_config::insert_dump_configs;
use crate::db::project::all_projects;
use crate::db::snapshot::{all_snapshot_summaries, delete_snapshot_summary, update_snapshot_summary};
use crate::db::snapshot_result::{find_snapshot_result, update_snapshot_result};
use crate::domain::snapshot::{SnapshotId, SnapshotName, SnapshotSummary};
use crate::domain::snapshot_result::SnapshotResult;
use crate::dump::dump;
use crate::logger;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotSummaryJson {
    pub snapshot_id: SnapshotId,
    pub snapshot_name: SnapshotName,
    pub create_at: String,
}

impl SnapshotSummaryJson {
    fn from(snapshot_summary: SnapshotSummary) -> Self {
        Self { snapshot_id: snapshot_summary.snapshot_id, snapshot_name: snapshot_summary.snapshot_name, create_at: snapshot_summary.create_at }
    }

    fn into(self) -> SnapshotSummary {
        SnapshotSummary::new(&self.snapshot_id, &self.snapshot_name, &self.create_at)
    }
}

#[tauri::command]
pub async fn all_snapshot_summaries_command(app_state: State<'_, AppState>) -> Result<Vec<SnapshotSummaryJson>, String> {
    logger::info("start all_snapshot_summaries_command");

    let conn = app_state.conn.lock().unwrap();
    let project_id = app_state.project_id.lock().unwrap();
    let project_id = project_id.as_ref().unwrap();

    let x = all_snapshot_summaries(&conn, project_id)
        .map(|snapshot_summaries| snapshot_summaries.into_iter().map(SnapshotSummaryJson::from).collect_vec())
        .map_err(|e| e.to_string());
    logger::info("end   all_snapshot_summaries_command");
    x
}

#[tauri::command]
pub async fn update_snapshot_summary_command(app_state: State<'_, AppState>, snapshot_summary_json: SnapshotSummaryJson) -> Result<(), String> {
    logger::info("start update_snapshot_summary_command");

    let conn = app_state.conn.lock().unwrap();

    let x = update_snapshot_summary(&conn, &snapshot_summary_json.into()).map_err(|e| e.to_string());
    logger::info("end   update_snapshot_summary_command");
    x
}

#[tauri::command]
pub async fn delete_snapshot_summary_command(app_state: State<'_, AppState>, snapshot_id: SnapshotId) -> Result<(), String> {
    logger::info("start delete_snapshot_summary_command");

    let conn = app_state.conn.lock().unwrap();

    let x = delete_snapshot_summary(&conn, &snapshot_id).map_err(|e| e.to_string());
    logger::info("end   delete_snapshot_summary_command");
    x
}

#[tauri::command]
pub async fn dump_snapshot_command(
    app_state: State<'_, AppState>,
    snapshot_id: SnapshotId,
    snapshot_name: SnapshotName,
    dump_config_jsons: Vec<DumpConfigJson>,
) -> Result<(), String> {
    logger::info("start dump_snapshot_command");

    let conn = app_state.conn.lock().unwrap();
    let project_id = app_state.project_id.lock().unwrap();
    let project_id = project_id.as_ref().unwrap();

    let projects = all_projects(&conn).map_err(|e| e.to_string())?;
    let project = projects.iter().find(|project| &project.project_id == project_id).unwrap();

    let dump_configs = dump_config_jsons.into_iter().map(|dump_config_json| dump_config_json.into()).collect_vec();

    match dump(&conn, project, &snapshot_id, snapshot_name, &dump_configs) {
        Ok(snapshot_result) => {
            insert_dump_configs(&conn, project_id, &snapshot_result.snapshot_id, &dump_configs).map_err(|e| e.to_string())?;

            logger::info("end   dump_snapshot_command");

            Ok(())
        }
        Err(e) => {
            update_snapshot_result(&conn, &SnapshotResult::failed(&snapshot_id)).map_err(|e| e.to_string())?;
            Err(e.to_string())
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotResultJson {
    pub percent: usize,
    pub done: usize,
    pub total: usize,
    pub status: String,
}

#[tauri::command]
pub async fn get_snapshot_result_command(app_state: State<'_, AppState>, snapshot_id: SnapshotId) -> Result<SnapshotResultJson, String> {
    let read_conn = app_state.read_conn.lock().unwrap();

    let snapshot_result = find_snapshot_result(&read_conn, &snapshot_id).map_err(|e| e.to_string())?;
    Ok(SnapshotResultJson {
        percent: snapshot_result.percent,
        done: snapshot_result.done,
        total: snapshot_result.total,
        status: snapshot_result.status,
    })
}
