use std::collections::HashMap;

use itertools::Itertools;

use serde::{Deserialize, Serialize};
use tauri::State;

use crate::command::state::AppState;
use crate::db::diff::{find_snapshot_diff, insert_snapshot_diff};
use crate::db::snapshot::find_table_snapshots;
use crate::domain::diff::ColDiff::{Added, Deleted, NoValue, Stay};
use crate::domain::diff::{create_diff_id, create_table_diff, ColDiff, DiffId, SnapshotDiff, TableDiff};
use crate::domain::schema::{ColName, PrimaryValue, TableName};
use crate::domain::snapshot::{SnapshotId, TableSnapshot};
use crate::logger;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotDiffJson {
    pub diff_id: DiffId,
    pub snapshot_id1: SnapshotId,
    pub snapshot_id2: SnapshotId,
    pub table_diffs: Vec<TableDiffJson>,
}

impl SnapshotDiffJson {
    fn from(snapshot_diff: SnapshotDiff) -> Self {
        Self {
            diff_id: snapshot_diff.diff_id,
            snapshot_id1: snapshot_diff.snapshot_id1,
            snapshot_id2: snapshot_diff.snapshot_id2,
            table_diffs: snapshot_diff.table_diffs.into_iter().map(TableDiffJson::from).collect(),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableDiffJson {
    pub table_name: TableName,
    pub primary_values: Vec<PrimaryValue>,
    pub primary_col_name: ColName,
    pub col_names: Vec<ColName>,
    pub row_diffs1: HashMap<PrimaryValue, HashMap<ColName, HashMap<String, String>>>,
    pub row_diffs2: HashMap<PrimaryValue, HashMap<ColName, HashMap<String, String>>>,
}

impl TableDiffJson {
    fn from(table_diff: TableDiff) -> Self {
        let mut row_diffs1 = HashMap::new();
        for (primary_value, col) in table_diff.row_diffs1 {
            let mut cols = HashMap::new();
            for (col_name, col_diff) in col {
                cols.insert(col_name, TableDiffJson::map(col_diff));
            }
            row_diffs1.insert(primary_value, cols);
        }

        let mut row_diffs2 = HashMap::new();
        for (primary_value, col) in table_diff.row_diffs2 {
            let mut cols = HashMap::new();
            for (col_name, col_diff) in col {
                cols.insert(col_name, TableDiffJson::map(col_diff));
            }
            row_diffs2.insert(primary_value, cols);
        }

        Self {
            table_name: table_diff.table_name,
            primary_values: table_diff.primary_col_values.into_iter().map(|primary_col_value| primary_col_value.as_primary_value()).collect(),
            primary_col_name: table_diff.primary_col_name,
            col_names: table_diff.col_names,
            row_diffs1,
            row_diffs2,
        }
    }

    fn map(col_diff: ColDiff) -> HashMap<String, String> {
        match col_diff {
            NoValue => vec![],
            Stay(v) => vec![("status".to_string(), "stay".to_string()), ("value".to_string(), v.as_display_value())],
            Added(v) => vec![("status".to_string(), "added".to_string()), ("value".to_string(), v.as_display_value())],
            Deleted(v) => vec![("status".to_string(), "deleted".to_string()), ("value".to_string(), v.as_display_value())],
        }
        .into_iter()
        .collect()
    }
}

#[tauri::command]
pub fn find_snapshot_diff_command(
    app_state: State<'_, AppState>,
    snapshot_id1: SnapshotId,
    snapshot_id2: SnapshotId,
) -> Result<SnapshotDiffJson, String> {
    logger::info("start find_snapshot_diff_command");

    let conn = app_state.conn.lock().unwrap();

    let snapshot_diff = match find_snapshot_diff(&conn, &snapshot_id1, &snapshot_id2).map_err(|e| e.to_string())? {
        Some(snapshot_diff) => Ok(snapshot_diff),
        None => Err("snapshot diff not created".to_string()),
    }?;

    let x = Ok(SnapshotDiffJson::from(snapshot_diff));
    logger::info("end   find_snapshot_diff_command");
    x
}

#[tauri::command]
pub fn create_snapshot_diff_command(
    app_state: State<'_, AppState>,
    snapshot_id1: SnapshotId,
    snapshot_id2: SnapshotId,
) -> Result<SnapshotDiffJson, String> {
    logger::info("start create_snapshot_diff_command");

    let conn = app_state.conn.lock().unwrap();

    let table_snapshots1 = find_table_snapshots(&conn, &snapshot_id1).map_err(|e| e.to_string())?;
    let table_snapshots1: HashMap<&TableName, &TableSnapshot> = table_snapshots1
        .iter()
        .into_group_map_by(|table_snapshot| &table_snapshot.table_name)
        .iter()
        .map(|(&table_name, table_snapshots)| (table_name, table_snapshots[0]))
        .collect();

    let table_snapshots2 = find_table_snapshots(&conn, &snapshot_id2).map_err(|e| e.to_string())?;
    let table_snapshots2: HashMap<&TableName, &TableSnapshot> = table_snapshots2
        .iter()
        .into_group_map_by(|table_snapshot| &table_snapshot.table_name)
        .iter()
        .map(|(&table_name, table_snapshots)| (table_name, table_snapshots[0]))
        .collect();

    let mut table_names1 = table_snapshots1.keys().cloned().collect_vec();
    let mut table_names2 = table_snapshots2.keys().cloned().collect_vec();
    table_names1.append(&mut table_names2);

    let snapshot_diff = SnapshotDiff::new(
        &create_diff_id(),
        &snapshot_id1,
        &snapshot_id2,
        table_names1
            .into_iter()
            .unique()
            .map(|table_name| create_table_diff(table_snapshots1.get(table_name).copied(), table_snapshots2.get(table_name).copied()))
            .filter(|table_diff| !table_diff.empty())
            .collect(),
    );

    insert_snapshot_diff(&conn, &snapshot_diff).map_err(|e| e.to_string())?;

    let x = Ok(SnapshotDiffJson::from(snapshot_diff));
    logger::info("end   create_snapshot_diff_command");
    x
}
