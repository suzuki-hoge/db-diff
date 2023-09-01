use itertools::Itertools;

pub type TableName = String;

pub type PrimaryColName = String;

pub type ColName = String;

pub type PrimaryValue = String;

pub type Hash = String;

pub struct TableSchema {
    pub table_name: TableName,
}

pub struct ColSchemata {
    pub primary_cols: Vec<ColSchema>,
    pub cols: Vec<ColSchema>,
}

impl ColSchemata {
    pub fn new(primary_cols: Vec<ColSchema>, cols: Vec<ColSchema>) -> Self {
        Self { primary_cols, cols }
    }

    pub fn has_any_primary_cols(&self) -> bool {
        !self.primary_cols.is_empty()
    }

    pub fn get_all_col_names(self) -> (PrimaryColName, Vec<ColName>) {
        let primary_col_name = self.primary_cols.into_iter().map(|primary_col| primary_col.col_name).join("-");
        let col_names = self.cols.into_iter().map(|col| col.col_name).collect();
        (primary_col_name, col_names)
    }

    pub fn get_all_col_refs(&self) -> Vec<&ColSchema> {
        let mut cols = self.primary_cols.iter().collect_vec();
        cols.extend(&self.cols);
        cols
    }

    pub fn get_indices(&self) -> Vec<(usize, bool)> {
        let mut result = vec![];
        for (i, _) in self.primary_cols.iter().enumerate() {
            result.push((i, true));
        }
        for (i, _) in self.cols.iter().enumerate() {
            result.push((self.primary_cols.len() + i, false));
        }
        result
    }
}

#[derive(Clone)]
pub struct ColSchema {
    pub col_name: ColName,
    pub data_type: String,
}
