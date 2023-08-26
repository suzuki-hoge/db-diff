use diesel::SqliteConnection;

use crate::db::snapshot::{insert_snapshot_summary, insert_table_snapshot};
use crate::domain::dump_config::DumpConfig;
use crate::domain::project::Project;
use crate::domain::project::Rdbms::Mysql;
use crate::domain::snapshot::{create_snapshot_id, SnapshotId, SnapshotName, SnapshotSummary, TableSnapshot};
use crate::dump::adapter::TargetDbAdapter;
use crate::dump::mysql80::TargetDbMysql80;

mod adapter;
mod mysql80;

pub fn get_dump_configs(project: &Project) -> anyhow::Result<Vec<DumpConfig>> {
    let mut adapter = match &project.rdbms {
        Mysql => TargetDbMysql80::new(project),
    }?;

    adapter.get_dump_configs()
}

pub fn dump(conn: &SqliteConnection, project: &Project, snapshot_name: SnapshotName) -> anyhow::Result<SnapshotId> {
    let mut adapter = match &project.rdbms {
        Mysql => TargetDbMysql80::new(project),
    }?;

    let snapshot_id = create_snapshot_id();

    let snapshot_summary = SnapshotSummary::create(&snapshot_id, &snapshot_name);
    insert_snapshot_summary(conn, &project.project_id, &snapshot_summary)?;

    let table_schemata = adapter.get_table_schemata()?;

    for table_schema in table_schemata {
        let col_schemata = adapter.get_col_schemata(&table_schema)?;

        if col_schemata.has_any_primary_cols() {
            let row_snapshots = adapter.get_row_snapshots(&table_schema, &col_schemata)?;

            let (primary_col_name, col_names) = col_schemata.get_all_col_names();
            let table_snapshot = TableSnapshot::new(&table_schema.table_name, primary_col_name, col_names, row_snapshots);

            insert_table_snapshot(conn, &snapshot_id, &table_snapshot)?;
        }
    }

    Ok(snapshot_id)
}
