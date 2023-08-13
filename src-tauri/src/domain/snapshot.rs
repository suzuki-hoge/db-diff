use std::cmp::max;
use std::collections::BTreeSet;

use chrono::Local;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::schema::{ColName, Hash, PrimaryColName, PrimaryValue, TableName};
use crate::domain::snapshot::ColValue::*;

pub type SnapshotId = String;

pub type SnapshotName = String;

pub fn create_snapshot_id() -> SnapshotId {
    Uuid::new_v4().to_string()
}

#[derive(Eq, PartialEq, Debug)]
pub struct SnapshotSummary {
    pub snapshot_id: SnapshotId,
    pub snapshot_name: SnapshotName,
    pub create_at: String,
}

impl SnapshotSummary {
    pub fn create(snapshot_id: &SnapshotId, snapshot_name: &SnapshotName) -> Self {
        let create_at = format!("{}", Local::now().format("%Y-%m-%d %H:%M:%S"));
        Self { snapshot_id: snapshot_id.clone(), snapshot_name: snapshot_name.clone(), create_at }
    }

    pub fn new<S: Into<String>>(snapshot_id: &SnapshotId, snapshot_name: S, create_at: S) -> Self {
        Self { snapshot_id: snapshot_id.clone(), snapshot_name: snapshot_name.into(), create_at: create_at.into() }
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct TableSnapshot {
    pub table_name: TableName,
    pub primary_col_name: PrimaryColName,
    pub col_names: Vec<ColName>,
    pub hash: Hash,
    pub row_snapshots: Vec<RowSnapshot>,
}

impl TableSnapshot {
    pub fn new(table_name: &TableName, primary_col_name: PrimaryColName, col_names: Vec<ColName>, row_snapshots: Vec<RowSnapshot>) -> Self {
        let row_hashes = row_snapshots.iter().map(|row_snapshot| &row_snapshot.hash).join("");
        let hash = format!("{:?}", md5::compute(format!("{}{}{}", primary_col_name, col_names.join(""), row_hashes)));
        Self { table_name: table_name.clone(), primary_col_name, col_names, hash, row_snapshots }
    }

    pub fn get_primary_col_values_vec(&self) -> Vec<PrimaryColValues> {
        self.row_snapshots.iter().map(|row_snapshot| row_snapshot.primary_col_values.clone()).collect()
    }

    pub fn merge_primary_col_values_vec(&self, other: &Self) -> Vec<PrimaryColValues> {
        let mut set = BTreeSet::new();

        for row in &self.row_snapshots {
            set.insert(row.primary_col_values.clone());
        }
        for row in &other.row_snapshots {
            set.insert(row.primary_col_values.clone());
        }

        set.into_iter().collect_vec()
    }

    pub fn merge_col_names(&self, other: &Self) -> Vec<ColName> {
        let mut result = vec![];

        for i in 0..max(self.col_names.len(), other.col_names.len()) {
            if i < self.col_names.len() && !result.contains(&self.col_names[i]) {
                result.push(self.col_names[i].clone());
            }
            if i < other.col_names.len() && !result.contains(&&other.col_names[i]) {
                result.push(other.col_names[i].clone());
            }
        }

        result
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct RowSnapshot {
    pub primary_col_values: PrimaryColValues,
    pub col_values: Vec<ColValue>,
    pub hash: Hash,
}

impl RowSnapshot {
    pub fn new(primary_col_values: Vec<ColValue>, col_values: Vec<ColValue>) -> Self {
        let mut all_col_values = vec![];
        all_col_values.extend(&primary_col_values);
        all_col_values.extend(&col_values);
        let all_col_raw_values = all_col_values.iter().map(|col_value| col_value.as_hash_parts()).join(",");
        let hash = format!("{:?}", md5::compute(all_col_raw_values));

        Self { primary_col_values: PrimaryColValues::new(primary_col_values), col_values, hash }
    }
}

#[derive(Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct PrimaryColValues {
    pub col_values: Vec<ColValue>,
}

impl PrimaryColValues {
    pub fn new(col_values: Vec<ColValue>) -> Self {
        Self { col_values }
    }

    pub fn as_primary_value(&self) -> PrimaryValue {
        self.col_values.iter().map(|col_value| col_value.as_display_value()).join("-")
    }
}

#[derive(Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub enum ColValue {
    SimpleNumber(String),
    BitNumber(String),
    SimpleString(String),
    DateString(String),
    BinaryString(String),
    JsonString(String),
    Null,
    ParseError,
}

impl ColValue {
    pub fn as_display_value(&self) -> String {
        match self {
            SimpleNumber(v) => v.to_string(),
            BitNumber(v) => format!("bit({v})"),
            SimpleString(v) => format!(r#""{v}""#),
            DateString(v) => format!(r#""{v}""#),
            BinaryString(_) => "binary".to_string(),
            JsonString(v) => v.to_string(),
            Null => "<null>".to_string(),
            ParseError => "parse error".to_string(),
        }
    }

    fn as_hash_parts(&self) -> String {
        match self {
            SimpleNumber(v) => v.to_string(),
            BitNumber(v) => v.to_string(),
            SimpleString(v) => v.to_string(),
            DateString(v) => v.to_string(),
            BinaryString(v) => format!("{:?}", md5::compute(v)),
            JsonString(v) => v.to_string(),
            Null => format!("{:?}", md5::compute("<null>")),
            ParseError => "parse error".to_string(),
        }
    }
}
