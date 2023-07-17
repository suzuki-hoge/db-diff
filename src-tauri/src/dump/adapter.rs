use crate::domain::schema::{ColSchemata, TableSchema};
use crate::domain::snapshot::RowSnapshot;

pub trait TargetDbAdapter {
    fn get_table_schemata(&mut self) -> anyhow::Result<Vec<TableSchema>>;

    fn get_col_schemata(&mut self, table_schema: &TableSchema) -> anyhow::Result<ColSchemata>;

    fn get_row_snapshots(&mut self, table_schema: &TableSchema, col_schemata: &ColSchemata) -> anyhow::Result<Vec<RowSnapshot>>;
}
