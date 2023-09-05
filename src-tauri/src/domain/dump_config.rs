use std::collections::HashMap;

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::domain::schema::{ColName, TableName};

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct DumpConfig {
    pub table_name: TableName,
    pub col_names: Vec<ColName>,
    pub value: String,
}

impl DumpConfig {
    pub fn init<S: Into<String>>(table_name: S, col_names: Vec<S>) -> Self {
        let col_names = col_names.into_iter().map(|col_name| col_name.into()).collect_vec();
        let created = col_names.iter().rev().find(|&col_name| col_name.starts_with("create"));
        let updated = col_names.iter().rev().find(|&col_name| col_name.starts_with("update"));

        let value = match (created, updated) {
            (Some(_), Some(col_name)) => col_name.clone(),
            (Some(col_name), None) => col_name.clone(),
            (None, Some(col_name)) => col_name.clone(),
            (None, None) => "ignore".to_string(),
        };

        Self { table_name: table_name.into(), col_names, value }
    }

    pub fn new<S: Into<String>>(table_name: S, col_names: Vec<S>, value: S) -> Self {
        Self { table_name: table_name.into(), col_names: col_names.into_iter().map(|col_name| col_name.into()).collect(), value: value.into() }
    }

    pub fn merge(def: Vec<Self>, found: Vec<Self>) -> Vec<Self> {
        let saved: HashMap<TableName, String> = found.into_iter().map(|dump_config| (dump_config.table_name, dump_config.value)).collect();

        def.iter().map(|dump_config| dump_config.set_value(saved.get(&dump_config.table_name).unwrap_or(&dump_config.value))).collect_vec()
    }

    fn set_value(&self, value: &str) -> Self {
        Self { table_name: self.table_name.clone(), col_names: self.col_names.clone(), value: value.to_string() }
    }

    pub fn sort(mut dump_configs: Vec<Self>) -> Vec<Self> {
        dump_configs.sort_by_key(|dump_config| dump_config.table_name.clone());
        dump_configs
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::dump_config::DumpConfig;

    #[test]
    fn init() {
        let sut = DumpConfig::init("users", vec!["id", "name"]);
        assert_eq!(sut.value, "ignore");

        let sut = DumpConfig::init("users", vec!["id", "name", "created_at"]);
        assert_eq!(sut.value, "created_at");

        let sut = DumpConfig::init("users", vec!["id", "name", "create_date"]);
        assert_eq!(sut.value, "create_date");

        let sut = DumpConfig::init("users", vec!["id", "name", "create_user"]);
        assert_eq!(sut.value, "create_user");

        let sut = DumpConfig::init("users", vec!["id", "name", "create_user", "create_date"]);
        assert_eq!(sut.value, "create_date");

        let sut = DumpConfig::init("users", vec!["id", "name", "updated_at"]);
        assert_eq!(sut.value, "updated_at");

        let sut = DumpConfig::init("users", vec!["id", "name", "update_date"]);
        assert_eq!(sut.value, "update_date");

        let sut = DumpConfig::init("users", vec!["id", "name", "update_user"]);
        assert_eq!(sut.value, "update_user");

        let sut = DumpConfig::init("users", vec!["id", "name", "update_user", "update_date"]);
        assert_eq!(sut.value, "update_date");

        let sut = DumpConfig::init("users", vec!["id", "name", "create_date", "update_date"]);
        assert_eq!(sut.value, "update_date");

        let sut = DumpConfig::init("users", vec!["id", "name", "create_user", "update_user", "create_date", "update_date"]);
        assert_eq!(sut.value, "update_date");
    }

    #[test]
    fn merge() {
        let def =
            vec![DumpConfig::new("groups", vec!["id", "name"], "limited"), DumpConfig::new("users", vec!["id", "name", "updated_at"], "limited")];
        let found =
            vec![DumpConfig::new("groups", vec!["id", "name"], "limited"), DumpConfig::new("users", vec!["id", "name", "updated_at"], "updated_at")];

        let sut = DumpConfig::merge(def, found);

        assert_eq!("limited", sut[0].value);
        assert_eq!("updated_at", sut[1].value);
    }
}
