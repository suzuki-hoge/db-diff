use anyhow::anyhow;
use mysql::{Conn, Opts, OptsBuilder};
use r2d2::ManageConnection;
use r2d2_mysql::MysqlConnectionManager;

pub type ProjectId = String;

#[cfg(test)]
pub fn create_project_id() -> ProjectId {
    uuid::Uuid::new_v4().to_string()
}

#[derive(Eq, PartialEq, Debug)]
pub enum Rdbms {
    Mysql,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Project {
    pub project_id: ProjectId,
    pub name: String,
    pub color: String,
    pub rdbms: Rdbms,
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: String,
    pub schema: String,
}

impl Project {
    #[allow(clippy::too_many_arguments)]
    pub fn new<S: Into<String>>(project_id: &ProjectId, name: S, color: S, rdbms: Rdbms, user: S, password: S, host: S, port: S, schema: S) -> Self {
        Self {
            project_id: project_id.clone(),
            name: name.into(),
            color: color.into(),
            rdbms,
            user: user.into(),
            password: password.into(),
            host: host.into(),
            port: port.into(),
            schema: schema.into(),
        }
    }

    pub fn create_connection(&self) -> anyhow::Result<Conn> {
        let url = format!("mysql://{}:{}@{}:{}/{}", self.user, self.password, self.host, self.port, self.schema);
        let opt = Opts::from_url(&url).unwrap();
        let builder = OptsBuilder::from_opts(opt);
        let manager = MysqlConnectionManager::new(builder);
        manager.connect().map_err(|e| anyhow!(e))
    }
}
