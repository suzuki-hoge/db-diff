use anyhow::anyhow;
use diesel::prelude::*;
use diesel::{RunQueryDsl, SqliteConnection};

use crate::db::schema;
use crate::domain::diff::SnapshotDiff;
use crate::domain::snapshot::SnapshotId;

pub fn find_snapshot_diff(conn: &SqliteConnection, snapshot_id1: &SnapshotId, snapshot_id2: &SnapshotId) -> anyhow::Result<Option<SnapshotDiff>> {
    let rows: Vec<String> = schema::snapshot_diffs::table
        .select(schema::snapshot_diffs::data)
        .filter(schema::snapshot_diffs::snapshot_id1.eq(snapshot_id1).and(schema::snapshot_diffs::snapshot_id2.eq(snapshot_id2)))
        .load(conn)
        .map_err(|e| anyhow!(e))?;
    Ok(rows.into_iter().next().map(|data| serde_json::from_str(&data).unwrap()))
}

pub fn insert_snapshot_diff(conn: &SqliteConnection, snapshot_diff: &SnapshotDiff) -> anyhow::Result<()> {
    diesel::insert_into(schema::snapshot_diffs::table)
        .values((
            schema::snapshot_diffs::diff_id.eq(&snapshot_diff.diff_id),
            schema::snapshot_diffs::snapshot_id1.eq(&snapshot_diff.snapshot_id1),
            schema::snapshot_diffs::snapshot_id2.eq(&snapshot_diff.snapshot_id2),
            schema::snapshot_diffs::data.eq(serde_json::to_string(snapshot_diff).unwrap()),
        ))
        .execute(conn)
        .map_err(|e| anyhow!(e))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use diesel::RunQueryDsl;

    use crate::db::create_connection;
    use crate::db::diff::{find_snapshot_diff, insert_snapshot_diff};
    use crate::db::project::insert_project;
    use crate::db::snapshot::insert_snapshot_summary;
    use crate::domain::diff::ColDiff::{Deleted, NoValue};
    use crate::domain::diff::{create_diff_id, SnapshotDiff, TableDiff};
    use crate::domain::project::Rdbms::Mysql;
    use crate::domain::project::{create_project_id, Project};
    use crate::domain::snapshot::ColValue::{SimpleNumber, SimpleString};
    use crate::domain::snapshot::{create_snapshot_id, ColValue, PrimaryColValues, SnapshotSummary};

    fn n(s: &str) -> ColValue {
        SimpleNumber(s.to_string())
    }

    fn s(s: &str) -> ColValue {
        SimpleString(s.to_string())
    }

    #[test]
    fn snapshot_diff() -> anyhow::Result<()> {
        // setup

        let conn = create_connection()?;
        diesel::sql_query("delete from projects").execute(&conn)?;

        let project_id = create_project_id();

        let project = Project::new(&project_id, "test-project", "red", Mysql, "user", "password", "127.0.0.1", "3306", "test-db");
        insert_project(&conn, &project)?;

        let snapshot_id1 = create_snapshot_id();
        let snapshot_id2 = create_snapshot_id();

        let snapshot_summary1 = SnapshotSummary::new(&snapshot_id1, "test1", "2023-07-03 08:17:52");
        let snapshot_summary2 = SnapshotSummary::new(&snapshot_id2, "test2", "2023-07-03 08:42:35");
        insert_snapshot_summary(&conn, &project_id, &snapshot_summary1)?;
        insert_snapshot_summary(&conn, &project_id, &snapshot_summary2)?;

        let _table_name = "items".to_string();

        // find
        let table_snapshot_opt = find_snapshot_diff(&conn, &snapshot_id1, &snapshot_id2)?;
        assert_eq!(None, table_snapshot_opt);

        // insert
        let primary_col_values1 = PrimaryColValues::new(vec![n("1")]);
        let primary_col_values2 = PrimaryColValues::new(vec![n("2")]);
        let mut row_diffs1 = HashMap::new();
        let mut row_diffs2 = HashMap::new();
        row_diffs1.insert(primary_col_values1.as_primary_value(), vec![("name".to_string(), Deleted(s("John")))].into_iter().collect());
        row_diffs2.insert(primary_col_values2.as_primary_value(), vec![("name".to_string(), NoValue)].into_iter().collect());

        let table_diff = TableDiff {
            table_name: "user".to_string(),
            primary_col_values_vec: vec![primary_col_values1, primary_col_values2],
            primary_col_name: "id".to_string(),
            col_names: vec!["name".to_string()],
            row_diffs1,
            row_diffs2,
        };

        let snapshot_diff = SnapshotDiff::new(&create_diff_id(), &snapshot_id1, &snapshot_id2, vec![table_diff]);
        insert_snapshot_diff(&conn, &snapshot_diff)?;

        let table_snapshot_opt = find_snapshot_diff(&conn, &snapshot_id1, &snapshot_id2)?;
        assert_eq!(Some(snapshot_diff), table_snapshot_opt);

        Ok(())
    }
}
