use diesel::SqliteConnection;

use crate::db::snapshot::{insert_snapshot_summary, insert_table_snapshot};
use crate::domain::project::Project;
use crate::domain::project::Rdbms::Mysql;
use crate::domain::snapshot::{create_snapshot_id, SnapshotId, SnapshotName, SnapshotSummary, TableSnapshot};
use crate::dump::adapter::TargetDbAdapter;
use crate::dump::mysql80::TargetDbMysql80;

mod adapter;
mod mysql80;

pub fn dump(conn: &SqliteConnection, project: &Project, snapshot_name: SnapshotName) -> anyhow::Result<SnapshotId> {
    let mut adapter = match &project.rdbms {
        Mysql => TargetDbMysql80::new(project),
    }?;

    let snapshot_id = create_snapshot_id();

    // todo: begin

    let snapshot_summary = SnapshotSummary::create(&snapshot_id, &snapshot_name);
    insert_snapshot_summary(conn, &project.project_id, &snapshot_summary)?;

    let table_schemata = adapter.get_table_schemata()?;

    for table_schema in table_schemata {
        let col_schemata = adapter.get_col_schemata(&table_schema)?;

        let row_snapshots = adapter.get_row_snapshots(&table_schema, &col_schemata)?;

        let (primary_col_name, col_names) = col_schemata.get_all_col_names();
        let table_snapshot = TableSnapshot::new(&table_schema.table_name, primary_col_name, col_names, row_snapshots);

        insert_table_snapshot(conn, &snapshot_id, &table_snapshot)?;
    }

    // todo: commit

    Ok(snapshot_id)
}
