use anyhow::anyhow;
use diesel::prelude::*;
use diesel::{RunQueryDsl, SqliteConnection};

use crate::db::schema;
use crate::domain::project::Rdbms::Mysql;
use crate::domain::project::{Project, ProjectId};
use schema::projects as projects_table;

#[derive(Queryable, Insertable)]
#[table_name = "projects_table"]
struct ProjectRecord {
    project_id: ProjectId,
    name: String,
    color: String,
    rdbms: String,
    user: String,
    password: String,
    host: String,
    port: String,
    schema: String,
}

impl ProjectRecord {
    fn from(project: &Project) -> Self {
        Self {
            project_id: project.project_id.clone(),
            name: project.name.clone(),
            color: project.color.clone(),
            rdbms: match project.rdbms {
                Mysql => "MySQL",
            }
            .to_string(),
            user: project.user.clone(),
            password: project.password.clone(),
            host: project.host.clone(),
            port: project.port.clone(),
            schema: project.schema.clone(),
        }
    }

    fn to(self) -> Project {
        Project {
            project_id: self.project_id,
            name: self.name,
            color: self.color,
            rdbms: match self.rdbms.as_ref() {
                "MySQL" => Mysql,
                _ => unreachable!(),
            },
            user: self.user,
            password: self.password,
            host: self.host,
            port: self.port,
            schema: self.schema,
        }
    }
}

pub fn all_projects(conn: &SqliteConnection) -> anyhow::Result<Vec<Project>> {
    let rows: Vec<ProjectRecord> = schema::projects::table.load(conn).map_err(|e| anyhow!(e))?;
    Ok(rows.into_iter().map(|row| row.to()).collect())
}

pub fn insert_project(conn: &SqliteConnection, project: &Project) -> anyhow::Result<()> {
    let record = ProjectRecord::from(project);
    diesel::insert_into(schema::projects::table).values(&record).execute(conn).map_err(|e| anyhow!(e))?;
    Ok(())
}

pub fn update_project(conn: &SqliteConnection, project: &Project) -> anyhow::Result<()> {
    let record = ProjectRecord::from(project);
    diesel::update(schema::projects::table.find(&project.project_id))
        .set((
            schema::projects::name.eq(&record.name),
            schema::projects::color.eq(&record.color),
            schema::projects::rdbms.eq(&record.rdbms),
            schema::projects::user.eq(&record.user),
            schema::projects::password.eq(&record.password),
            schema::projects::host.eq(&record.host),
            schema::projects::port.eq(&record.port),
            schema::projects::schema.eq(&record.schema),
        ))
        .execute(conn)
        .map_err(|e| anyhow!(e))?;
    Ok(())
}

pub fn delete_project(conn: &SqliteConnection, project_id: &ProjectId) -> anyhow::Result<()> {
    diesel::delete(schema::projects::table.find(project_id)).execute(conn).map_err(|e| anyhow!(e))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use diesel::RunQueryDsl;

    use crate::db::create_connection;
    use crate::db::project::{all_projects, delete_project, insert_project, update_project};
    use crate::domain::project::Project;
    use crate::domain::project::Rdbms::Mysql;
    use crate::domain::snapshot::create_snapshot_id;

    #[test]
    fn project() -> anyhow::Result<()> {
        // setup

        let conn = create_connection()?;
        diesel::sql_query("delete from projects").execute(&conn)?;

        // all
        let projects = all_projects(&conn)?;
        assert_eq!(0, projects.len());

        let project_id = create_snapshot_id();

        // insert
        let project1 = Project::new(&project_id, "test-project", "red", Mysql, "user", "password", "127.0.0.1", "3306", "test-db");
        insert_project(&conn, &project1)?;

        let projects = all_projects(&conn)?;
        assert_eq!(1, projects.len());
        assert_eq!(&project1, &projects[0]);

        // update
        let project2 = Project::new(&project_id, "test-project-2", "red", Mysql, "user2", "password2", "127.0.0.2", "3307", "test-db2");
        update_project(&conn, &project2)?;

        let projects = all_projects(&conn)?;
        assert_eq!(1, projects.len());
        assert_eq!(&project2, &projects[0]);

        // delete
        delete_project(&conn, &project_id)?;

        let projects = all_projects(&conn)?;
        assert_eq!(0, projects.len());

        Ok(())
    }
}
