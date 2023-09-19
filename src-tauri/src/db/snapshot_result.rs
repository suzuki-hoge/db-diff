use anyhow::anyhow;
use diesel::prelude::*;
use diesel::{RunQueryDsl, SqliteConnection};

use schema::snapshot_results as snapshot_results_table;

use crate::db::schema;
use crate::domain::snapshot::SnapshotId;
use crate::domain::snapshot_result::SnapshotResult;

#[derive(Queryable, Insertable)]
#[table_name = "snapshot_results_table"]
struct SnapshotResultRecord {
    snapshot_id: SnapshotId,
    percent: i32,
    done: i32,
    total: i32,
    status: String,
}

impl SnapshotResultRecord {
    fn from(snapshot_result: &SnapshotResult) -> Self {
        Self {
            snapshot_id: snapshot_result.snapshot_id.clone(),
            percent: snapshot_result.percent as i32,
            done: snapshot_result.done as i32,
            total: snapshot_result.total as i32,
            status: snapshot_result.status.to_string(),
        }
    }

    fn to(self) -> SnapshotResult {
        SnapshotResult {
            snapshot_id: self.snapshot_id,
            percent: self.percent as usize,
            done: self.done as usize,
            total: self.total as usize,
            status: self.status,
        }
    }
}

pub fn find_snapshot_result(conn: &SqliteConnection, snapshot_id: &SnapshotId) -> anyhow::Result<SnapshotResult> {
    let rows: Vec<SnapshotResultRecord> =
        schema::snapshot_results::table.filter(schema::snapshot_results::snapshot_id.eq(snapshot_id)).load(conn).map_err(|e| anyhow!(e))?;
    Ok(rows.into_iter().next().map(|row| row.to()).unwrap_or(SnapshotResult::zero(snapshot_id)))
}

pub fn insert_snapshot_result(conn: &SqliteConnection, snapshot_result: &SnapshotResult) -> anyhow::Result<()> {
    let record = SnapshotResultRecord::from(snapshot_result);
    diesel::insert_into(schema::snapshot_results::table).values(&record).execute(conn).map_err(|e| anyhow!(e))?;
    Ok(())
}

pub fn update_snapshot_result(conn: &SqliteConnection, snapshot_result: &SnapshotResult) -> anyhow::Result<()> {
    let record = SnapshotResultRecord::from(snapshot_result);
    diesel::update(schema::snapshot_results::table.find(&snapshot_result.snapshot_id))
        .set((schema::snapshot_results::percent.eq(&record.percent), schema::snapshot_results::status.eq(&record.status)))
        .execute(conn)
        .map_err(|e| anyhow!(e))?;
    Ok(())
}
