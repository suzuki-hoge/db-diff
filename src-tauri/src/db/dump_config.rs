use anyhow::anyhow;
use chrono::Local;
use diesel::prelude::*;
use diesel::{RunQueryDsl, SqliteConnection};

use crate::db::schema;
use crate::domain::dump_config::DumpConfig;
use crate::domain::project::ProjectId;
use crate::domain::snapshot::SnapshotId;

pub fn find_recent_dump_configs(conn: &SqliteConnection, project_id: &ProjectId) -> anyhow::Result<Option<Vec<DumpConfig>>> {
    let rows: Vec<String> = schema::dump_configs::table
        .select(schema::dump_configs::data)
        .filter(schema::dump_configs::project_id.eq(&project_id))
        .order(schema::dump_configs::create_at.desc())
        .load(conn)
        .map_err(|e| anyhow!(e))?;
    Ok(rows.into_iter().next().map(|data| DumpConfig::sort(serde_json::from_str(&data).unwrap())))
}

pub fn find_dump_config(conn: &SqliteConnection, snapshot_id: &SnapshotId) -> anyhow::Result<Vec<DumpConfig>> {
    let rows: Vec<String> = schema::dump_configs::table
        .select(schema::dump_configs::data)
        .filter(schema::dump_configs::snapshot_id.eq(&snapshot_id))
        .load(conn)
        .map_err(|e| anyhow!(e))?;
    Ok(rows.into_iter().next().map(|data| DumpConfig::sort(serde_json::from_str(&data).unwrap())).unwrap())
}

pub fn insert_dump_configs(
    conn: &SqliteConnection,
    project_id: &ProjectId,
    snapshot_id: &SnapshotId,
    dump_configs: &[DumpConfig],
) -> anyhow::Result<()> {
    let data = serde_json::to_string(dump_configs).unwrap();
    diesel::insert_into(schema::dump_configs::table)
        .values((
            schema::dump_configs::snapshot_id.eq(snapshot_id),
            schema::dump_configs::project_id.eq(project_id),
            schema::dump_configs::data.eq(data),
            schema::dump_configs::create_at.eq(Local::now().format("%F %T%.6f").to_string()),
        ))
        .execute(conn)
        .map_err(|e| anyhow!(e))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use diesel::RunQueryDsl;
    use itertools::Itertools;

    use crate::db::dump_config::{find_dump_config, find_recent_dump_configs, insert_dump_configs};
    use crate::db::project::insert_project;
    use crate::db::snapshot::insert_snapshot_summary;
    use crate::db::{create_sqlite_connection, migrate_sqlite_if_missing};
    use crate::domain::dump_config::DumpConfig;
    use crate::domain::project::Project;
    use crate::domain::project::Rdbms::Mysql;
    use crate::domain::snapshot::{create_snapshot_id, SnapshotSummary};

    #[test]
    fn dump_config() -> anyhow::Result<()> {
        // setup

        migrate_sqlite_if_missing()?;
        let conn = create_sqlite_connection()?;
        diesel::sql_query("delete from dump_configs").execute(&conn)?;

        let project_id = create_snapshot_id();
        let project = Project::new(&project_id, "test-project", "red", Mysql, "user", "password", "127.0.0.1", "3306", "test-db");
        insert_project(&conn, &project)?;

        // recent
        let dump_configs = find_recent_dump_configs(&conn, &project_id)?;
        assert_eq!(None, dump_configs);

        // insert
        let snapshot_id1 = create_snapshot_id();
        let snapshot_summary1 = SnapshotSummary::new(&snapshot_id1, "test1", "2023-07-03 08:17:52");
        insert_snapshot_summary(&conn, &project_id, &snapshot_summary1)?;

        let dump_configs1 =
            vec![DumpConfig::new("groups", vec!["id", "name"], "limited"), DumpConfig::new("users", vec!["id", "name", "updated_at"], "updated_at")];
        insert_dump_configs(&conn, &project_id, &snapshot_id1, &dump_configs1)?;

        // insert
        let snapshot_id2 = create_snapshot_id();
        let snapshot_summary2 = SnapshotSummary::new(&snapshot_id2, "test1", "2023-07-03 08:17:52");
        insert_snapshot_summary(&conn, &project_id, &snapshot_summary2)?;

        let dump_configs2 =
            vec![DumpConfig::new("groups", vec!["id", "name"], "ignore"), DumpConfig::new("users", vec!["id", "name", "updated_at"], "updated_at")];
        insert_dump_configs(&conn, &project_id, &snapshot_id2, &dump_configs2)?;

        // recent
        let dump_configs = find_recent_dump_configs(&conn, &project_id)?;
        assert_eq!(vec!["ignore", "updated_at"], dump_configs.unwrap().iter().map(|dump_config| &dump_config.value).collect_vec());

        // get
        let dump_configs = find_dump_config(&conn, &snapshot_id1)?;
        assert_eq!(vec!["limited", "updated_at"], dump_configs.iter().map(|dump_config| &dump_config.value).collect_vec());

        Ok(())
    }
}
