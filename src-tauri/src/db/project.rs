use anyhow::anyhow;
use mysql::{from_row, Conn};

use crate::domain::project::Rdbms::Mysql;
use crate::domain::project::{Project, ProjectId};

pub fn all_projects(conn: &mut Conn) -> anyhow::Result<Vec<Project>> {
    conn.query("select project_id, name, rdbms, user, password, host, port, `schema` from project order by project_id")
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row| {
                    let (project_id, name, rdbms, user, password, host, port, schema) =
                        from_row::<(ProjectId, String, String, String, String, String, String, String)>(row);
                    let rdbms = match rdbms.as_ref() {
                        "MySQL" => Mysql,
                        _ => unreachable!(),
                    };
                    Project::new(&project_id, name, rdbms, user, password, host, port, schema)
                })
                .collect()
        })
        .map_err(|e| anyhow!(e))
}

pub fn insert_project(conn: &mut Conn, project: &Project) -> anyhow::Result<()> {
    conn.prep_exec(
        "insert into project values (?, ?, ?, ?, ?, ?, ?, ?)",
        (
            &project.project_id,
            &project.name,
            match project.rdbms {
                Mysql => "MySQL",
            },
            &project.user,
            &project.password,
            &project.host,
            &project.port,
            &project.schema,
        ),
    )?;
    Ok(())
}

pub fn update_project(conn: &mut Conn, project: &Project) -> anyhow::Result<()> {
    conn.prep_exec(
        "update project set name = ?, rdbms = ?, user = ?, password = ?, host = ?, port = ?, `schema` = ? where project_id = ?",
        (
            &project.name,
            match project.rdbms {
                Mysql => "MySQL",
            },
            &project.user,
            &project.password,
            &project.host,
            &project.port,
            &project.schema,
            &project.project_id,
        ),
    )?;
    Ok(())
}

pub fn delete_project(conn: &mut Conn, project_id: &ProjectId) -> anyhow::Result<()> {
    conn.prep_exec("delete from project where project_id = ?", vec![project_id])?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::db::create_connection;
    use crate::db::project::{all_projects, delete_project, insert_project, update_project};
    use crate::domain::project::Project;
    use crate::domain::project::Rdbms::Mysql;
    use crate::domain::snapshot::create_snapshot_id;

    #[test]
    fn project() -> anyhow::Result<()> {
        // setup

        let mut conn = create_connection()?;
        conn.prep_exec("delete from project", ())?;

        // all
        let projects = all_projects(&mut conn)?;
        assert_eq!(0, projects.len());

        let project_id = create_snapshot_id();

        // insert
        let project1 = Project::new(&project_id, "test-project", Mysql, "user", "password", "127.0.0.1", "3306", "test-db");
        insert_project(&mut conn, &project1)?;

        let projects = all_projects(&mut conn)?;
        assert_eq!(1, projects.len());
        assert_eq!(&project1, &projects[0]);

        // update
        let project2 = Project::new(&project_id, "test-project-2", Mysql, "user2", "password2", "127.0.0.2", "3307", "test-db2");
        update_project(&mut conn, &project2)?;

        let projects = all_projects(&mut conn)?;
        assert_eq!(1, projects.len());
        assert_eq!(&project2, &projects[0]);

        // delete
        delete_project(&mut conn, &project_id)?;

        let projects = all_projects(&mut conn)?;
        assert_eq!(0, projects.len());

        Ok(())
    }
}
