use itertools::Itertools;
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::command::state::AppState;
use crate::db::project::{all_projects, delete_project, insert_project, update_project};
use crate::domain::project::Rdbms::Mysql;
use crate::domain::project::{Project, ProjectId};
use crate::logger;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectJson {
    pub project_id: ProjectId,
    pub name: String,
    pub color: String,
    pub rdbms: String,
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: String,
    pub schema: String,
}

impl ProjectJson {
    fn from(project: Project) -> Self {
        Self {
            project_id: project.project_id,
            name: project.name,
            color: project.color,
            rdbms: match project.rdbms {
                Mysql => "MySQL".to_string(),
            },
            user: project.user,
            password: project.password,
            host: project.host,
            port: project.port,
            schema: project.schema,
        }
    }

    fn into(self) -> Project {
        Project::new(
            &self.project_id,
            &self.name,
            &self.color,
            match self.rdbms.as_ref() {
                "MySQL" => Mysql,
                _ => unreachable!(),
            },
            &self.user,
            &self.password,
            &self.host,
            &self.port,
            &self.schema,
        )
    }
}

#[tauri::command]
pub async fn all_projects_command(app_state: State<'_, AppState>) -> Result<Vec<ProjectJson>, String> {
    logger::info("start all_projects_command");

    let conn = app_state.conn.lock().unwrap();

    let x = all_projects(&conn).map(|projects| projects.into_iter().map(ProjectJson::from).collect_vec()).map_err(|e| e.to_string());
    logger::info("end   all_projects_command");
    x
}

#[tauri::command]
pub async fn select_project_command(app_state: State<'_, AppState>, project_id: ProjectId) -> Result<(), String> {
    logger::info("start select_project_command");

    let conn = app_state.conn.lock().unwrap();

    let projects = all_projects(&conn).map_err(|e| e.to_string())?;
    let project = projects.iter().find(|project| project.project_id == project_id).unwrap();

    let x = match project.create_connection() {
        Ok(_) => {
            app_state.set_project_id(project_id);
            Ok(())
        }
        Err(e) => Err(e.to_string()),
    };
    logger::info("end   select_project_command");
    x
}

#[tauri::command]
pub async fn test_connection_project_command(project_json: ProjectJson) -> Result<String, String> {
    logger::info("start test_connection_project_command");

    let project = project_json.into();

    let x = match &project.create_connection() {
        Ok(_) => Ok(project.create_url()),
        Err(_e) => Err(project.create_url()),
    };
    logger::info("end   test_connection_project_command");
    x
}

#[tauri::command]
pub async fn insert_project_command(app_state: State<'_, AppState>, project_json: ProjectJson) -> Result<(), String> {
    logger::info("start insert_project_command");

    let conn = app_state.conn.lock().unwrap();

    let x = insert_project(&conn, &project_json.into()).map_err(|e| e.to_string());
    logger::info("end   insert_project_command");
    x
}

#[tauri::command]
pub async fn update_project_command(app_state: State<'_, AppState>, project_json: ProjectJson) -> Result<(), String> {
    logger::info("start update_project_command");

    let conn = app_state.conn.lock().unwrap();

    let x = update_project(&conn, &project_json.into()).map_err(|e| e.to_string());
    logger::info("end   update_project_command");
    x
}

#[tauri::command]
pub async fn delete_project_command(app_state: State<'_, AppState>, project_id: ProjectId) -> Result<(), String> {
    logger::info("start delete_project_command");

    let conn = app_state.conn.lock().unwrap();

    let x = delete_project(&conn, &project_id).map_err(|e| e.to_string());
    logger::info("end   delete_project_command");
    x
}
