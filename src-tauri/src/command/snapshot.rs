use itertools::Itertools;
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::command::state::AppState;
use crate::db::project::all_projects;
use crate::db::snapshot::{all_snapshot_summaries, delete_snapshot_summary, update_snapshot_summary};
use crate::domain::snapshot::{SnapshotId, SnapshotName, SnapshotSummary};
use crate::dump::dump;

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
pub fn all_snapshot_summaries_command(app_state: State<'_, AppState>) -> Result<Vec<SnapshotSummaryJson>, String> {
    let mut conn = app_state.conn.lock().unwrap();
    let project_id = app_state.project_id.lock().unwrap();
    let project_id = project_id.as_ref().unwrap();

    all_snapshot_summaries(&mut conn, project_id)
        .map(|snapshot_summaries| snapshot_summaries.into_iter().map(SnapshotSummaryJson::from).collect_vec())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_snapshot_summary_command(app_state: State<'_, AppState>, snapshot_summary_json: SnapshotSummaryJson) -> Result<(), String> {
    let mut conn = app_state.conn.lock().unwrap();

    update_snapshot_summary(&mut conn, &snapshot_summary_json.into()).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_snapshot_summary_command(app_state: State<'_, AppState>, snapshot_id: SnapshotId) -> Result<(), String> {
    let mut conn = app_state.conn.lock().unwrap();

    delete_snapshot_summary(&mut conn, &snapshot_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn dump_snapshot_command(app_state: State<'_, AppState>, snapshot_name: SnapshotName) -> Result<(), String> {
    let mut conn = app_state.conn.lock().unwrap();
    let project_id = app_state.project_id.lock().unwrap();
    let project_id = project_id.as_ref().unwrap();

    let projects = all_projects(&mut conn).map_err(|e| e.to_string())?;
    let project = projects.iter().find(|project| &project.project_id == project_id).unwrap();

    dump(&mut conn, project, snapshot_name).map_err(|e| e.to_string())?;

    Ok(())
}
