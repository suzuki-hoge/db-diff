use anyhow::Context;
use diesel::prelude::*;
use diesel::{RunQueryDsl, SqliteConnection};

use schema::snapshot_summaries as snapshot_summaries_table;
use schema::table_snapshots as table_snapshots_table;

use crate::db::schema;
use crate::domain::project::ProjectId;
use crate::domain::schema::TableName;
use crate::domain::snapshot::{SnapshotId, SnapshotName, SnapshotSummary, TableSnapshot};

#[derive(Queryable, Insertable)]
#[table_name = "snapshot_summaries_table"]
struct SnapshotSummaryRecord {
    snapshot_id: SnapshotId,
    project_id: ProjectId,
    snapshot_name: SnapshotName,
    create_at: String,
}

impl SnapshotSummaryRecord {
    fn from(snapshot_summary: &SnapshotSummary, project_id: &ProjectId) -> Self {
        Self {
            snapshot_id: snapshot_summary.snapshot_id.clone(),
            project_id: project_id.clone(),
            snapshot_name: snapshot_summary.snapshot_name.clone(),
            create_at: snapshot_summary.create_at.clone(),
        }
    }

    fn to(self) -> SnapshotSummary {
        SnapshotSummary { snapshot_id: self.snapshot_id, snapshot_name: self.snapshot_name, create_at: self.create_at }
    }
}

pub fn all_snapshot_summaries(conn: &SqliteConnection, project_id: &ProjectId) -> anyhow::Result<Vec<SnapshotSummary>> {
    let rows: Vec<SnapshotSummaryRecord> =
        schema::snapshot_summaries::table.filter(schema::snapshot_summaries::project_id.eq(project_id)).load(conn).context("error")?;
    Ok(rows.into_iter().map(|row| row.to()).collect())
}

pub fn insert_snapshot_summary(conn: &SqliteConnection, project_id: &ProjectId, snapshot_summary: &SnapshotSummary) -> anyhow::Result<()> {
    let record = SnapshotSummaryRecord::from(snapshot_summary, project_id);
    diesel::insert_into(schema::snapshot_summaries::table).values(&record).execute(conn).context("error")?;
    Ok(())
}

pub fn update_snapshot_summary(conn: &SqliteConnection, snapshot_summary: &SnapshotSummary) -> anyhow::Result<()> {
    diesel::update(schema::snapshot_summaries::table.find(&snapshot_summary.snapshot_id))
        .set(schema::snapshot_summaries::snapshot_name.eq(&snapshot_summary.snapshot_name))
        .execute(conn)
        .context("error")?;
    Ok(())
}

pub fn delete_snapshot_summary(conn: &SqliteConnection, snapshot_id: &SnapshotId) -> anyhow::Result<()> {
    diesel::delete(schema::snapshot_summaries::table.find(snapshot_id)).execute(conn).context("error")?;
    Ok(())
}

#[derive(Queryable, Insertable)]
#[table_name = "table_snapshots_table"]
struct TableSnapshotRecord {
    snapshot_id: SnapshotId,
    table_name: TableName,
    data: String,
}

impl TableSnapshotRecord {
    fn from(table_snapshot: &TableSnapshot, fk: &SnapshotId) -> Self {
        Self { snapshot_id: fk.clone(), table_name: table_snapshot.table_name.clone(), data: serde_json::to_string(table_snapshot).unwrap() }
    }

    fn to(self) -> TableSnapshot {
        serde_json::from_str(&self.data).unwrap()
    }
}

pub fn find_table_snapshots(conn: &SqliteConnection, snapshot_id: &SnapshotId) -> anyhow::Result<Vec<TableSnapshot>> {
    let rows: Vec<TableSnapshotRecord> =
        schema::table_snapshots::table.filter(schema::table_snapshots::snapshot_id.eq(snapshot_id)).load(conn).context("error")?;
    Ok(rows.into_iter().map(|row| row.to()).collect())
}

pub fn insert_table_snapshot(conn: &SqliteConnection, snapshot_id: &SnapshotId, table_snapshot: &TableSnapshot) -> anyhow::Result<()> {
    let record = TableSnapshotRecord::from(table_snapshot, snapshot_id);
    diesel::insert_into(schema::table_snapshots::table).values(&record).execute(conn).context("error")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use diesel::RunQueryDsl;

    use crate::db::create_connection;
    use crate::db::project::insert_project;
    use crate::db::snapshot::{
        all_snapshot_summaries, delete_snapshot_summary, find_table_snapshots, insert_snapshot_summary, insert_table_snapshot,
        update_snapshot_summary,
    };
    use crate::domain::project::Rdbms::Mysql;
    use crate::domain::project::{create_project_id, Project};
    use crate::domain::snapshot::ColValue::{SimpleNumber, SimpleString};
    use crate::domain::snapshot::{create_snapshot_id, ColValue, RowSnapshot, SnapshotSummary, TableSnapshot};

    #[test]
    fn snapshot_summary() -> anyhow::Result<()> {
        // setup

        let conn = create_connection()?;
        diesel::sql_query("delete from projects").execute(&conn)?;

        let project_id = create_project_id();

        let project = Project::new(&project_id, "test-project", Mysql, "user", "password", "127.0.0.1", "3306", "test-db");
        insert_project(&conn, &project)?;

        // all
        let snapshot_summaries = all_snapshot_summaries(&conn, &project_id)?;
        assert_eq!(0, snapshot_summaries.len());

        let snapshot_id = create_snapshot_id();

        // insert
        let snapshot_summary1 = SnapshotSummary::new(&snapshot_id, "test", "2023-07-03 08:17:52");
        insert_snapshot_summary(&conn, &project_id, &snapshot_summary1)?;

        let snapshot_summaries = all_snapshot_summaries(&conn, &project_id)?;
        assert_eq!(1, snapshot_summaries.len());
        assert_eq!(&snapshot_summary1, &snapshot_summaries[0]);

        // update
        let snapshot_summary2 = SnapshotSummary::new(&snapshot_id, "test2", "2023-07-03 08:17:52");
        update_snapshot_summary(&conn, &snapshot_summary2)?;

        let snapshot_summaries = all_snapshot_summaries(&conn, &project_id)?;
        assert_eq!(1, snapshot_summaries.len());
        assert_eq!(&snapshot_summary2, &snapshot_summaries[0]);

        // delete
        delete_snapshot_summary(&conn, &snapshot_id)?;

        let snapshot_summaries = all_snapshot_summaries(&conn, &project_id)?;
        assert_eq!(0, snapshot_summaries.len());

        Ok(())
    }

    fn n(s: &str) -> ColValue {
        SimpleNumber(s.to_string())
    }

    fn s(s: &str) -> ColValue {
        SimpleString(s.to_string())
    }

    #[test]
    fn table_snapshot() -> anyhow::Result<()> {
        // setup

        let conn = create_connection()?;
        diesel::sql_query("delete from projects").execute(&conn)?;

        let project_id = create_project_id();

        let project = Project::new(&project_id, "test-project", Mysql, "user", "password", "127.0.0.1", "3306", "test-db");
        insert_project(&conn, &project)?;

        let snapshot_id = create_snapshot_id();

        let snapshot_summary = SnapshotSummary::new(&snapshot_id, "test", "2023-07-03 08:17:52");
        insert_snapshot_summary(&conn, &project_id, &snapshot_summary)?;

        let table_name = "items".to_string();

        // find
        let table_snapshots = find_table_snapshots(&conn, &snapshot_id)?;
        assert_eq!(0, table_snapshots.len());

        // insert
        let row_snapshot1 = RowSnapshot::new(vec![n("1")], vec![s("123"), n("1200")]);
        let row_snapshot2 = RowSnapshot::new(vec![n("2")], vec![s("456"), n("560")]);
        let table_snapshot =
            TableSnapshot::new(&table_name, "id".to_string(), vec!["code".to_string(), "price".to_string()], vec![row_snapshot1, row_snapshot2]);
        insert_table_snapshot(&conn, &snapshot_id, &table_snapshot)?;

        let table_snapshots = find_table_snapshots(&conn, &snapshot_id)?;
        assert_eq!(vec![table_snapshot], table_snapshots);

        Ok(())
    }
}
