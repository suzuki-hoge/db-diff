use itertools::Itertools;
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::command::state::AppState;
use crate::db::project::{all_projects, delete_project, insert_project, update_project};
use crate::domain::project::Rdbms::Mysql;
use crate::domain::project::{Project, ProjectId};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectJson {
    pub project_id: ProjectId,
    pub rdbms: String,
    pub name: String,
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
pub fn all_projects_command(app_state: State<'_, AppState>) -> Result<Vec<ProjectJson>, String> {
    let mut conn = app_state.conn.lock().unwrap();

    all_projects(&mut conn).map(|projects| projects.into_iter().map(ProjectJson::from).collect_vec()).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn select_project_command(app_state: State<'_, AppState>, project_id: ProjectId) -> Result<(), String> {
    app_state.set_project_id(project_id);

    Ok(())
}

#[tauri::command]
pub fn insert_project_command(app_state: State<'_, AppState>, project_json: ProjectJson) -> Result<(), String> {
    let mut conn = app_state.conn.lock().unwrap();

    insert_project(&mut conn, &project_json.into()).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_project_command(app_state: State<'_, AppState>, project_json: ProjectJson) -> Result<(), String> {
    let mut conn = app_state.conn.lock().unwrap();

    update_project(&mut conn, &project_json.into()).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_project_command(app_state: State<'_, AppState>, project_id: ProjectId) -> Result<(), String> {
    let mut conn = app_state.conn.lock().unwrap();

    delete_project(&mut conn, &project_id).map_err(|e| e.to_string())
}
