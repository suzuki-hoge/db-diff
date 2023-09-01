use std::collections::HashMap;

use diesel::SqliteConnection;

use crate::db::snapshot::{insert_snapshot_summary, insert_table_snapshots};
use crate::domain::dump_config::DumpConfig;
use crate::domain::project::Project;
use crate::domain::project::Rdbms::Mysql;
use crate::domain::schema::TableName;
use crate::domain::snapshot::{create_snapshot_id, SnapshotId, SnapshotName, SnapshotSummary, TableSnapshot};
use crate::dump::adapter::TargetDbAdapter;
use crate::dump::mysql80::TargetDbMysql80;
use crate::logger;

mod adapter;
mod mysql80;

pub fn get_dump_configs(project: &Project) -> anyhow::Result<Vec<DumpConfig>> {
    let mut adapter = match &project.rdbms {
        Mysql => TargetDbMysql80::new(project),
    }?;

    adapter.get_dump_configs()
}

pub fn dump(conn: &SqliteConnection, project: &Project, snapshot_name: SnapshotName, dump_configs: &[DumpConfig]) -> anyhow::Result<SnapshotId> {
    let mut adapter = match &project.rdbms {
        Mysql => TargetDbMysql80::new(project),
    }?;

    let snapshot_id = create_snapshot_id();

    let snapshot_summary = SnapshotSummary::create(&snapshot_id, &snapshot_name);
    insert_snapshot_summary(conn, &project.project_id, &snapshot_summary)?;

    let dump_configs: HashMap<&TableName, &DumpConfig> = dump_configs.iter().map(|dump_config| (&dump_config.table_name, dump_config)).collect();

    let table_schemata = adapter.get_table_schemata()?;

    for table_schema in table_schemata {
        let dump_config = dump_configs.get(&table_schema.table_name).unwrap();

        if dump_config.value == "ignore" {
            logger::info(format!("ignore: {}", &table_schema.table_name));
            continue;
        }

        let mut table_snapshots = vec![];

        if table_schema.has_any_primary_cols() {
            let row_snapshots = adapter.get_row_snapshots(&table_schema, &dump_config.value)?;

            let (primary_col_name, col_names) = table_schema.get_all_col_names();
            table_snapshots.push(TableSnapshot::new(&table_schema.table_name, primary_col_name, col_names, row_snapshots));
        }

        insert_table_snapshots(conn, &snapshot_id, table_snapshots)?;
    }

    Ok(snapshot_id)
}
