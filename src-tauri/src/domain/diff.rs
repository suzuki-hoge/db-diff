use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::diff::ColDiff::*;
use crate::domain::schema::{ColName, Hash, PrimaryColName, PrimaryValue, TableName};
use crate::domain::snapshot::{ColValue, PrimaryColValues, SnapshotId, TableSnapshot};

pub type DiffId = String;

pub fn create_diff_id() -> DiffId {
    Uuid::new_v4().to_string()
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct SnapshotDiff {
    pub diff_id: DiffId,
    pub snapshot_id1: SnapshotId,
    pub snapshot_id2: SnapshotId,
    pub table_diffs: Vec<TableDiff>,
}

impl SnapshotDiff {
    pub fn new(diff_id: &DiffId, snapshot_id1: &SnapshotId, snapshot_id2: &SnapshotId, table_diffs: Vec<TableDiff>) -> Self {
        Self { diff_id: diff_id.clone(), snapshot_id1: snapshot_id1.clone(), snapshot_id2: snapshot_id2.clone(), table_diffs }
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct TableDiff {
    pub table_name: TableName,
    pub primary_col_values: Vec<PrimaryColValues>,
    pub primary_col_name: PrimaryColName,
    pub col_names: Vec<ColName>,
    pub row_diffs1: HashMap<PrimaryValue, HashMap<ColName, ColDiff>>,
    pub row_diffs2: HashMap<PrimaryValue, HashMap<ColName, ColDiff>>,
}

impl TableDiff {
    pub fn init(table_name: &TableName, primary_col_name: &PrimaryColName, col_names: Vec<&ColName>) -> Self {
        Self {
            table_name: table_name.clone(),
            primary_col_values: vec![],
            primary_col_name: primary_col_name.clone(),
            col_names: col_names.into_iter().cloned().collect(),
            row_diffs1: HashMap::new(),
            row_diffs2: HashMap::new(),
        }
    }

    pub fn empty(&self) -> bool {
        self.row_diffs1.is_empty() && self.row_diffs2.is_empty()
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
pub enum ColDiff {
    NoValue,
    Stay(ColValue),
    Added(ColValue),
    Deleted(ColValue),
}

pub fn create_table_diff(table_snapshot1: Option<&TableSnapshot>, table_snapshot2: Option<&TableSnapshot>) -> TableDiff {
    match (table_snapshot1, table_snapshot2) {
        (Some(table_snapshot1), Some(table_snapshot2)) => take_table_snapshot_diff(table_snapshot1, table_snapshot2),
        (None, Some(table_snapshot2)) => create_missing_pair_diff(table_snapshot2, 2),
        (Some(table_snapshot1), None) => create_missing_pair_diff(table_snapshot1, 1),
        (None, None) => unreachable!(),
    }
}

type Rows<'a> = HashMap<&'a PrimaryColValues, (&'a Hash, Cols<'a>)>;
type Cols<'a> = HashMap<&'a ColName, &'a ColValue>;

fn take_table_snapshot_diff(table_snapshot1: &TableSnapshot, table_snapshot2: &TableSnapshot) -> TableDiff {
    let total_col_names = table_snapshot1.merge_col_names(table_snapshot2);

    let mut table_diff = TableDiff::init(&table_snapshot1.table_name, &table_snapshot1.primary_col_name, total_col_names.clone());

    let rows1 = parse_rows(table_snapshot1);
    let rows2 = parse_rows(table_snapshot2);

    for primary_col_values in table_snapshot1.merge_primary_col_values(table_snapshot2) {
        let row1 = rows1.get(primary_col_values);
        let row2 = rows2.get(primary_col_values);

        match (row1, row2) {
            // 2 つの行の Hash が一致している場合は、スキップする
            (Some((hash1, _)), Some((hash2, _))) if hash1 == hash2 => {}

            // 同一の主キー値が片方にしかない場合は、片方の全列を差分として登録する
            (None, Some((_, cols2))) => {
                table_diff.row_diffs2.insert(
                    primary_col_values.as_primary_value(),
                    cols2.iter().map(|(&col_name, &col_value)| (col_name.clone(), Added(col_value.clone()))).collect(),
                );
                table_diff.primary_col_values.push(primary_col_values.clone());
            }

            // 同一の主キー値が片方にしかない場合は、片方の全列を差分として登録する
            (Some((_, cols1)), None) => {
                table_diff.row_diffs1.insert(
                    primary_col_values.as_primary_value(),
                    cols1.iter().map(|(&col_name, &col_value)| (col_name.clone(), Deleted(col_value.clone()))).collect(),
                );
                table_diff.primary_col_values.push(primary_col_values.clone());
            }

            // 2 つの行の Hash が一致しない場合は、列ごとに差分をとる
            (Some((_, cols1)), Some((_, cols2))) => {
                let get_col_diff_f1 = |col_name| match (cols1.get(col_name), cols2.get(col_name)) {
                    (Some(&col_value1), Some(&col_value2)) if col_value1 == col_value2 => Stay(col_value1.clone()),
                    (Some(&col_value1), _) => Deleted(col_value1.clone()),
                    (None, _) => NoValue,
                };
                let get_col_diff_f2 = |col_name| match (cols1.get(col_name), cols2.get(col_name)) {
                    (Some(&col_value1), Some(&col_value2)) if col_value1 == col_value2 => Stay(col_value2.clone()),
                    (_, Some(&col_value2)) => Added(col_value2.clone()),
                    (_, None) => NoValue,
                };
                table_diff.row_diffs1.insert(
                    primary_col_values.as_primary_value(),
                    total_col_names.iter().map(|&col_name| (col_name.clone(), get_col_diff_f1(col_name))).collect(),
                );
                table_diff.row_diffs2.insert(
                    primary_col_values.as_primary_value(),
                    total_col_names.iter().map(|&col_name| (col_name.clone(), get_col_diff_f2(col_name))).collect(),
                );
                table_diff.primary_col_values.push(primary_col_values.clone());
            }

            (None, None) => unreachable!(),
        };
    }

    table_diff
}

fn parse_rows<'a>(table_snapshot: &'a TableSnapshot) -> Rows<'a> {
    let mut rows: Rows = HashMap::new();

    for row_snapshot in &table_snapshot.row_snapshots {
        let cols: Cols<'a> = table_snapshot.col_names.iter().enumerate().map(|(i, col_name)| (col_name, &row_snapshot.col_values[i])).collect();
        rows.insert(&row_snapshot.primary_col_values, (&row_snapshot.hash, cols));
    }

    rows
}

fn create_missing_pair_diff(table_snapshot: &TableSnapshot, n: usize) -> TableDiff {
    let mut table_diff = TableDiff::init(&table_snapshot.table_name, &table_snapshot.primary_col_name, table_snapshot.col_names.iter().collect());

    let rows = parse_rows(table_snapshot);

    for primary_col_values in table_snapshot.get_primary_col_values() {
        if let Some((_, cols)) = rows.get(primary_col_values) {
            let row_diff = cols
                .iter()
                .map(|(&col_name, &col_value)| (col_name.clone(), if n == 1 { Deleted(col_value.clone()) } else { Added(col_value.clone()) }))
                .collect();

            if n == 1 {
                table_diff.row_diffs1.insert(primary_col_values.as_primary_value(), row_diff);
            } else {
                table_diff.row_diffs2.insert(primary_col_values.as_primary_value(), row_diff);
            }
        }
        table_diff.primary_col_values.push(primary_col_values.clone());
    }

    table_diff
}

#[cfg(test)]
mod tests_create_snapshot_diff {
    use std::collections::HashMap;

    use itertools::Itertools;

    use crate::domain::diff::ColDiff::*;
    use crate::domain::diff::{create_table_diff, ColDiff};
    use crate::domain::schema::{ColName, PrimaryValue};
    use crate::domain::snapshot::ColValue::{SimpleNumber, SimpleString};
    use crate::domain::snapshot::{ColValue, PrimaryColValues, RowSnapshot, TableSnapshot};

    fn n(s: &str) -> ColValue {
        SimpleNumber(s.to_string())
    }

    fn s(s: &str) -> ColValue {
        SimpleString(s.to_string())
    }

    fn mk_table_snapshot(table_name: &str, primary_col_name: &str, col_names: Vec<&str>, row_snapshots: Vec<RowSnapshot>) -> TableSnapshot {
        TableSnapshot::new(
            &table_name.to_string(),
            primary_col_name.to_string(),
            col_names.iter().map(|col_name| col_name.to_string()).collect(),
            row_snapshots,
        )
    }

    fn mk_act<'a>(
        row_diffs: &'a HashMap<PrimaryValue, HashMap<ColName, ColDiff>>,
        primary_col_values: &PrimaryColValues,
        name: &'a str,
    ) -> &'a ColDiff {
        row_diffs.get(&primary_col_values.as_primary_value()).unwrap().get(name).unwrap()
    }

    #[test]
    fn test_row_0_and_row_1() {
        let rows2 = vec![RowSnapshot::new(vec![n("1")], vec![s("John")])];
        let table_snapshot2 = mk_table_snapshot("user", "id", vec!["name"], rows2);

        let act = create_table_diff(None, Some(&table_snapshot2));

        let primary_col_values1 = PrimaryColValues::new(vec![n("1")]);

        assert_eq!(vec![&primary_col_values1], act.primary_col_values.iter().collect_vec());

        assert_eq!(0, act.row_diffs1.len());

        assert_eq!(1, act.row_diffs2.len());
        assert_eq!(&Added(s("John")), mk_act(&act.row_diffs2, &primary_col_values1, "name"));
    }

    #[test]
    fn test_row_1_and_row_0() {
        let rows1 = vec![RowSnapshot::new(vec![n("1")], vec![s("John")])];
        let table_snapshot1 = mk_table_snapshot("user", "id", vec!["name"], rows1);

        let act = create_table_diff(Some(&table_snapshot1), None);

        let primary_col_values1 = PrimaryColValues::new(vec![n("1")]);

        assert_eq!(vec![&primary_col_values1], act.primary_col_values.iter().collect_vec());

        assert_eq!(1, act.row_diffs1.len());
        assert_eq!(&Deleted(s("John")), mk_act(&act.row_diffs1, &primary_col_values1, "name"));

        assert_eq!(0, act.row_diffs2.len());
    }

    #[test]
    fn test_row_1_and_row_1() {
        let rows1 = vec![RowSnapshot::new(vec![n("1")], vec![s("John")])];
        let table_snapshot1 = mk_table_snapshot("user", "id", vec!["name"], rows1);

        let rows2 = vec![RowSnapshot::new(vec![n("1")], vec![s("Jane")])];
        let table_snapshot2 = mk_table_snapshot("user", "id", vec!["name"], rows2);

        let act = create_table_diff(Some(&table_snapshot1), Some(&table_snapshot2));

        let primary_col_values1 = PrimaryColValues::new(vec![n("1")]);

        assert_eq!(vec![&primary_col_values1], act.primary_col_values.iter().collect_vec());

        assert_eq!(1, act.row_diffs1.len());
        assert_eq!(&Deleted(s("John")), mk_act(&act.row_diffs1, &primary_col_values1, "name"));

        assert_eq!(1, act.row_diffs2.len());
        assert_eq!(&Added(s("Jane")), mk_act(&act.row_diffs2, &primary_col_values1, "name"));
    }

    #[test]
    fn test_row_2_and_row_1() {
        let rows1 = vec![RowSnapshot::new(vec![n("1")], vec![s("John")]), RowSnapshot::new(vec![n("2")], vec![s("Jack")])];
        let table_snapshot1 = mk_table_snapshot("user", "id", vec!["name"], rows1);

        let rows2 = vec![RowSnapshot::new(vec![n("1")], vec![s("Jane")])];
        let table_snapshot2 = mk_table_snapshot("user", "id", vec!["name"], rows2);

        let act = create_table_diff(Some(&table_snapshot1), Some(&table_snapshot2));

        let primary_col_values1 = PrimaryColValues::new(vec![n("1")]);
        let primary_col_values2 = PrimaryColValues::new(vec![n("2")]);

        assert_eq!(vec![&primary_col_values1, &primary_col_values2], act.primary_col_values.iter().collect_vec());

        assert_eq!(2, act.row_diffs1.len());
        assert_eq!(&Deleted(s("John")), mk_act(&act.row_diffs1, &primary_col_values1, "name"));
        assert_eq!(&Deleted(s("Jack")), mk_act(&act.row_diffs1, &primary_col_values2, "name"));

        assert_eq!(1, act.row_diffs2.len());
        assert_eq!(&Added(s("Jane")), mk_act(&act.row_diffs2, &primary_col_values1, "name"));
    }

    #[test]
    fn test_row_1_and_row_2() {
        let rows1 = vec![RowSnapshot::new(vec![n("1")], vec![s("John")])];
        let table_snapshot1 = mk_table_snapshot("user", "id", vec!["name"], rows1);

        let rows2 = vec![RowSnapshot::new(vec![n("1")], vec![s("John")]), RowSnapshot::new(vec![n("2")], vec![s("Jack")])];
        let table_snapshot2 = mk_table_snapshot("user", "id", vec!["name"], rows2);

        let act = create_table_diff(Some(&table_snapshot1), Some(&table_snapshot2));

        let primary_col_values2 = PrimaryColValues::new(vec![n("2")]);

        assert_eq!(vec![&primary_col_values2], act.primary_col_values.iter().collect_vec());

        assert_eq!(0, act.row_diffs1.len());

        assert_eq!(1, act.row_diffs2.len());
        assert_eq!(&Added(s("Jack")), mk_act(&act.row_diffs2, &primary_col_values2, "name"));
    }

    #[test]
    fn test_row_1_and_row_1_nomatch_cols() {
        let rows1 = vec![RowSnapshot::new(vec![n("1")], vec![s("John")])];
        let table_snapshot1 = mk_table_snapshot("user", "id", vec!["name"], rows1);

        let rows2 = vec![RowSnapshot::new(vec![n("1")], vec![n("39")])];
        let table_snapshot2 = mk_table_snapshot("user", "id", vec!["age"], rows2);

        let act = create_table_diff(Some(&table_snapshot1), Some(&table_snapshot2));

        let primary_col_values1 = PrimaryColValues::new(vec![n("1")]);

        assert_eq!(vec![&primary_col_values1], act.primary_col_values.iter().collect_vec());

        assert_eq!(1, act.row_diffs1.len());
        assert_eq!(&Deleted(s("John")), mk_act(&act.row_diffs1, &primary_col_values1, "name"));
        assert_eq!(&NoValue, mk_act(&act.row_diffs1, &primary_col_values1, "age"));

        assert_eq!(1, act.row_diffs2.len());
        assert_eq!(&NoValue, mk_act(&act.row_diffs2, &primary_col_values1, "name"));
        assert_eq!(&Added(n("39")), mk_act(&act.row_diffs2, &primary_col_values1, "age"));
    }

    #[test]
    fn test_row_1_and_row_1_primary_value_mismatch() {
        let rows1 = vec![RowSnapshot::new(vec![n("1")], vec![s("John")])];
        let table_snapshot1 = mk_table_snapshot("user", "id", vec!["name"], rows1);

        let rows2 = vec![RowSnapshot::new(vec![n("2")], vec![s("Jane")])];
        let table_snapshot2 = mk_table_snapshot("user", "id", vec!["name"], rows2);

        let act = create_table_diff(Some(&table_snapshot1), Some(&table_snapshot2));

        let primary_col_values1 = PrimaryColValues::new(vec![n("1")]);
        let primary_col_values2 = PrimaryColValues::new(vec![n("2")]);

        assert_eq!(vec![&primary_col_values1, &primary_col_values2], act.primary_col_values.iter().collect_vec());

        assert_eq!(1, act.row_diffs1.len());
        assert_eq!(&Deleted(s("John")), mk_act(&act.row_diffs1, &primary_col_values1, "name"));

        assert_eq!(1, act.row_diffs2.len());
        assert_eq!(&Added(s("Jane")), mk_act(&act.row_diffs2, &primary_col_values2, "name"));
    }

    #[test]
    fn test_row_0_and_row_1_multi_primary_cols() {
        let rows2 = vec![RowSnapshot::new(vec![n("123"), n("789")], vec![s("John")])];
        let table_snapshot2 = mk_table_snapshot("user", "id-code", vec!["name"], rows2);

        let act = create_table_diff(None, Some(&table_snapshot2));

        let primary_col_values1 = PrimaryColValues::new(vec![n("123"), n("789")]);

        assert_eq!(vec![&primary_col_values1], act.primary_col_values.iter().collect_vec());

        assert_eq!(0, act.row_diffs1.len());

        assert_eq!(1, act.row_diffs2.len());
        assert_eq!(&Added(s("John")), mk_act(&act.row_diffs2, &primary_col_values1, "name"));
    }
}
