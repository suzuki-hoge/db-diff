use std::sync::Mutex;

use diesel::SqliteConnection;

use crate::db::create_connection;
use crate::domain::project::ProjectId;

pub struct AppState {
    pub conn: Mutex<SqliteConnection>,
    pub project_id: Mutex<Option<ProjectId>>,
}

impl AppState {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self { conn: Mutex::new(create_connection()?), project_id: Mutex::new(None) })
    }

    pub fn set_project_id(&self, selected_project_id: ProjectId) {
        let mut project_id = self.project_id.lock().unwrap();
        *project_id = Some(selected_project_id);
    }
}
