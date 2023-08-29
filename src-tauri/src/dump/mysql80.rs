use std::collections::HashMap;

use anyhow::anyhow;
use itertools::Itertools;
use mysql::Value::NULL;
use mysql::{from_row, from_value, Conn, Value};

use crate::domain::dump_config::DumpConfig;
use crate::domain::project::Project;
use crate::domain::schema::{ColName, ColSchema, ColSchemata, TableName, TableSchema};
use crate::domain::snapshot::ColValue::*;
use crate::domain::snapshot::{ColValue, RowSnapshot};
use crate::dump::adapter::TargetDbAdapter;
use crate::logger;

pub struct TargetDbMysql80 {
    conn: Conn,
    schema: String,
}

impl TargetDbMysql80 {
    pub fn new(project: &Project) -> anyhow::Result<Self> {
        let conn = project.create_connection()?;
        let schema = project.schema.clone();

        Ok(Self { conn, schema })
    }
}

impl TargetDbAdapter for TargetDbMysql80 {
    fn get_dump_configs(&mut self) -> anyhow::Result<Vec<DumpConfig>> {
        let mut map: HashMap<TableName, Vec<ColName>> = HashMap::new();

        let query = format!(
            "select table_name, column_name from information_schema.columns where table_schema = '{}' order by table_name, ordinal_position",
            self.schema
        );
        logger::info(format!("query: {}", &query));

        let result = self.conn.query(query).map_err(|e| anyhow!(e))?;

        for row in result.map(|x| x.unwrap()) {
            let (table_name, col_name) = from_row::<(TableName, ColName)>(row);
            map.entry(table_name).or_default().push(col_name);
        }

        Ok(map.into_iter().map(|(k, v)| DumpConfig::init(k, v)).collect_vec())
    }

    fn get_table_schemata(&mut self) -> anyhow::Result<Vec<TableSchema>> {
        let query = format!("select table_name from information_schema.tables where table_schema = '{}' order by table_name", self.schema);

        logger::info(format!("query: {}", &query));

        self.conn
            .query(query)
            .map(|result| {
                result
                    .map(|x| x.unwrap())
                    .map(|row| {
                        let table_name = from_row(row);
                        TableSchema { table_name }
                    })
                    .collect()
            })
            .map_err(|e| anyhow!(e))
    }

    fn get_col_schemata(&mut self, table_schema: &TableSchema) -> anyhow::Result<ColSchemata> {
        let query = format!("select column_name, data_type, column_type, column_key from information_schema.columns where table_schema = '{}' and table_name = '{}' order by ordinal_position", self.schema, table_schema.table_name);

        logger::info(format!("query: {}", &query));

        let rows = self.conn.query(query).unwrap().map(|row| row.unwrap()).collect_vec();

        let unique_cols = rows
            .clone()
            .into_iter()
            .flat_map(|row| {
                let (col_name, data_type, col_type, col_key) = from_row::<(String, String, String, String)>(row);
                if &col_key == "PRI" {
                    vec![ColSchema { col_name, data_type, col_type }]
                } else {
                    vec![]
                }
            })
            .collect_vec();

        let cols = rows
            .into_iter()
            .flat_map(|row| {
                let (col_name, data_type, col_type, col_key) = from_row::<(String, String, String, String)>(row);
                if &col_key != "PRI" {
                    vec![ColSchema { col_name, data_type, col_type }]
                } else {
                    vec![]
                }
            })
            .collect_vec();

        Ok(ColSchemata::new(unique_cols, cols))
    }

    fn get_row_snapshots(
        &mut self,
        table_schema: &TableSchema,
        col_schemata: &ColSchemata,
        dump_config_value: &str,
    ) -> anyhow::Result<Vec<RowSnapshot>> {
        let all_cols = col_schemata.get_all_col_refs();

        let col_names = all_cols.iter().map(|col| as_select_col(col)).join(",");
        let order_by = if dump_config_value == "limited" { "".to_string() } else { format!("order by {dump_config_value}") };
        let query = format!("select {} from `{}` {} limit 1000", col_names, table_schema.table_name, order_by);

        logger::info(format!("query: {}", &query));

        self.conn
            .query(query)
            .map(|result| {
                result
                    .map(|x| x.unwrap())
                    .map(|row| {
                        let mut primary_cols = vec![];
                        let mut cols = vec![];

                        for (i, is_primary) in col_schemata.get_indices() {
                            let value: Value = row.get(i).unwrap();
                            let col = if value == NULL { Null } else { parse_col_value(all_cols[i], from_value(value)) };
                            if is_primary {
                                primary_cols.push(col);
                            } else {
                                cols.push(col);
                            }
                        }

                        RowSnapshot::new(primary_cols, cols)
                    })
                    .collect()
            })
            .map_err(|e| anyhow!(e))
    }
}

fn as_select_col(col: &ColSchema) -> String {
    match col.data_type.as_str() {
        "bit" => format!("bin(`{}`)", col.col_name),
        _ => format!("`{}`", col.col_name),
    }
}

fn parse_col_value(col_schema: &ColSchema, value: String) -> ColValue {
    match col_schema.data_type.as_str() {
        "tinyint" | "smallint" | "mediumint" | "int" | "bigint" => SimpleNumber(value),
        "decimal" | "float" | "double" => SimpleNumber(value),
        "bit" => BitNumber(value),
        "date" | "time" | "datetime" | "timestamp" | "year" => DateString(value),
        "char" | "varchar" => SimpleString(value),
        "binary" | "varbinary" => BinaryString(value),
        "tinyblob" | "mediumblob" | "blob" | "longblob" => BinaryString(value),
        "tinytext" | "mediumtext" | "text" | "longtext" => SimpleString(value),
        "enum" | "set" => SimpleString(value),
        "json" => JsonString(value),
        _ => ParseError,
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod adapter_tests {
    use itertools::Itertools;

    use crate::domain::dump_config::DumpConfig;
    use crate::domain::project::{create_project_id, Project};
    use crate::domain::project::Rdbms::Mysql;
    use crate::domain::snapshot::ColValue::*;
    use crate::dump::adapter::TargetDbAdapter;
    use crate::dump::mysql80::TargetDbMysql80;

    fn s(s: &str) -> String {
        s.to_string()
    }

    #[test]
    fn get_dump_configs() -> anyhow::Result<()> {
        let project = Project::new(&create_project_id(), "test-project", "red", Mysql, "user","password","127.0.0.1","19001","testdata");

        let mut adapter = TargetDbMysql80::new(&project)?;

        // drop all
        for table_schema in adapter.get_table_schemata()? {
            adapter.conn.prep_exec(format!("drop table {}", table_schema.table_name), ())?;
        }

        adapter.conn.prep_exec("create table 01_number_01_signed ( id int auto_increment, col_tinyint tinyint, col_smallint smallint, col_mediumint mediumint, col_int int, col_bigint bigint, primary key (id) )", ())?;
        adapter.conn.prep_exec("create table 11_string_01_char ( id int auto_increment, col_char char(3), col_varchar varchar(3), primary key (id) )", ())?;

        let sut = DumpConfig::sort(adapter.get_dump_configs()?);

        assert_eq!("01_number_01_signed", sut[0].table_name);
        assert_eq!("id, col_tinyint, col_smallint, col_mediumint, col_int, col_bigint", sut[0].col_names.join(", "));
        assert_eq!("limited", sut[0].value);

        assert_eq!("11_string_01_char", sut[1].table_name);
        assert_eq!("id, col_char, col_varchar", sut[1].col_names.join(", "));
        assert_eq!("limited", sut[1].value);

        Ok(())
    }
    
    #[test]
    fn dump() -> anyhow::Result<()> {
        let project = Project::new(&create_project_id(), "test-project", "red", Mysql, "user","password","127.0.0.1","19001","testdata");

        let mut adapter = TargetDbMysql80::new(&project)?;
        
        // drop all
        for table_schema in adapter.get_table_schemata()? {
            adapter.conn.prep_exec(format!("drop table {}", table_schema.table_name), ())?;
        }

        adapter.conn.prep_exec("create table 01_number_01_signed ( id int auto_increment, col_tinyint tinyint, col_smallint smallint, col_mediumint mediumint, col_int int, col_bigint bigint, primary key (id) )", ())?;
        adapter.conn.prep_exec("insert into 01_number_01_signed values (1, 127, 32767, 8388607, 2147483647, 9223372036854775807), (2, -128, -32768, -8388608, -2147483648, -9223372036854775808)", ())?;

        adapter.conn.prep_exec("create table 02_number_02_unsigned ( id int auto_increment, col_tinyint tinyint unsigned, col_smallint smallint unsigned, col_mediumint mediumint unsigned, col_int int unsigned, col_bigint bigint unsigned, primary key (id) )", ())?;
        adapter.conn.prep_exec("insert into 02_number_02_unsigned values (1, 255, 65535, 16777215, 4294967295, 18446744073709551615), (2, 0, 0, 0, 0, 0)", ())?;

        adapter.conn.prep_exec("create table 03_number_03_fixed ( id int auto_increment, col_decimal decimal(5, 2), col_numeric numeric(5, 2), primary key (id) )", ())?;
        adapter.conn.prep_exec("insert into 03_number_03_fixed values (1, 999.99, 999.99), (2, -999.99, -999.99)", ())?;

        adapter.conn.prep_exec("create table 04_number_04_float ( id int auto_increment, col_float float(5, 2), col_double double(5, 2), primary key (id) )", ())?;
        adapter.conn.prep_exec("insert into 04_number_04_float values (1, 999.99, 999.99), (2, -999.99, -999.99)", ())?;

        adapter.conn.prep_exec("create table 05_number_05_bit ( id int auto_increment, col_bit bit(10), primary key (id) )", ())?;
        adapter.conn.prep_exec("insert into 05_number_05_bit values (1, b'1000000000'), (2, b'0'), (3, 512), (4, 0)", ())?;

        adapter.conn.prep_exec("create table 06_date_01_date ( id int auto_increment, col_date date, primary key (id) )", ())?;
        adapter.conn.prep_exec("insert into 06_date_01_date values (1, '2020-01-01')", ())?;

        adapter.conn.prep_exec("create table 07_date_02_time ( id int auto_increment, col_time time, primary key (id) )", ())?;
        adapter.conn.prep_exec("insert into 07_date_02_time values (1, '00:00:00')", ())?;

        adapter.conn.prep_exec("create table 08_date_03_datetime ( id int auto_increment, col_datetime datetime, primary key (id) )", ())?;
        adapter.conn.prep_exec("insert into 08_date_03_datetime values (1, '2020-01-01 00:00:00')", ())?;

        adapter.conn.prep_exec("create table 09_date_04_timestamp ( id int auto_increment, col_timestamp timestamp, primary key (id) )", ())?;
        adapter.conn.prep_exec("insert into 09_date_04_timestamp values (1, '2020-01-01 00:00:00')", ())?;

        adapter.conn.prep_exec("create table 10_date_05_year ( id int auto_increment, col_year year, primary key (id) )", ())?;
        adapter.conn.prep_exec("insert into 10_date_05_year values (1, 2020)", ())?;

        adapter.conn.prep_exec("create table 11_string_01_char ( id int auto_increment, col_char char(3), col_varchar varchar(3), primary key (id) )", ())?;
        adapter.conn.prep_exec("insert into 11_string_01_char values (1, 'abc', 'abc'), (2, '', ''), (3, null, null)", ())?;

        adapter.conn.prep_exec("create table 12_string_02_binary ( id int auto_increment, col_binary binary(3), col_varbinary varbinary(3), primary key (id) )", ())?;
        adapter.conn.prep_exec("insert into 12_string_02_binary values (1, 'abc', 'abc')", ())?;

        adapter.conn.prep_exec("create table 13_string_03_blob ( id int auto_increment, col_tinyblob tinyblob, col_blob blob, col_mediumblob mediumblob, col_longblob longblob, primary key (id) )", ())?;
        adapter.conn.prep_exec("insert into 13_string_03_blob values (1, 'abc', 'abc', 'abc', 'abc')", ())?;

        adapter.conn.prep_exec("create table 14_string_04_text ( id int auto_increment, col_tinytext tinytext, col_text text, col_mediumtext mediumtext, col_longtext longtext, primary key (id) )", ())?;
        adapter.conn.prep_exec("insert into 14_string_04_text values (1, 'abc', 'abc', 'abc', 'abc')", ())?;

        adapter.conn.prep_exec("create table 15_string_05_enum ( id int auto_increment, col_enum enum ('active', 'inactive'), primary key (id) )", ())?;
        adapter.conn.prep_exec("insert into 15_string_05_enum values (1, 'active'), (2, 'inactive')", ())?;

        adapter.conn.prep_exec("create table 16_string_06_set ( id int auto_increment, col_set set ('pc', 'phone'), primary key (id) )", ())?;
        adapter.conn.prep_exec("insert into 16_string_06_set values (1, 'pc'), (2, 'phone'), (3, 'phone,pc'), (4, 'pc,phone')", ())?;

        adapter.conn.prep_exec("create table 17_string_07_json ( id int auto_increment, col_json json, primary key (id) )", ())?;
        adapter.conn.prep_exec(r#"insert into 17_string_07_json values (1, '{"id": 1, "name": "John"}')"#, ())?;
        adapter.conn.prep_exec(r#"insert into 17_string_07_json values (2, '[1, 2, "foo"]')"#, ())?;
        adapter.conn.prep_exec(r#"insert into 17_string_07_json values (3, '{"items": ["pc", "phone"], "option": {"id": 1}}')"#, ())?;

        adapter.conn.prep_exec("create table 18_key_01_primary ( code int, primary key (code) )", ())?;
        
        adapter.conn.prep_exec("create table 19_key_02_unique ( code int, unique (code) )", ())?;
        
        adapter.conn.prep_exec("create table 20_key_03_unique_not_null ( code int not null, unique (code) )", ())?;
        
        adapter.conn.prep_exec("create table 21_key_04_primary_primary ( code1 int, code2 int, primary key (code1, code2) )", ())?;
        
        adapter.conn.prep_exec("create table 22_key_05_primary_unique ( code1 int, code2 int, primary key (code1), unique (code2) )", ())?;
        
        adapter.conn.prep_exec("create table 23_key_06_primary_unique_not_null ( code1 int, code2 int not null, primary key (code1), unique (code2) )", ())?;
        
        adapter.conn.prep_exec("create table 24_key_07_unique_unique ( code1 int, code2 int, unique (code1), unique (code2) )", ())?;
        
        adapter.conn.prep_exec("create table 25_key_08_unique_not_null_unique ( code1 int not null, code2 int, unique (code1), unique (code2) )", ())?;
        
        adapter.conn.prep_exec("create table 26_key_09_unique_not_null_unique_not_null ( code1 int not null, code2 int not null, unique (code1), unique (code2) )", ())?;
        
        adapter.conn.prep_exec("create table 27_key_10_multi_unique_unique ( code1 int, code2 int, unique (code1, code2) )", ())?;
        
        adapter.conn.prep_exec("create table 28_key_11_multi_unique_not_null_unique ( code1 int not null, code2 int, unique (code1, code2) )", ())?;
        
        adapter.conn.prep_exec("create table 29_key_12_multi_unique_not_null_unique_not_null ( code1 int not null, code2 int not null, unique (code1, code2) )", ())?;
        
        adapter.conn.prep_exec("create table 30_key_13_nothing ( code int )", ())?;
        
        let table_schemata = adapter.get_table_schemata()?;
        
        {
            assert_eq!("01_number_01_signed", table_schemata[0].table_name);
            
            let col_schemata = adapter.get_col_schemata(&table_schemata[0])?;

            assert_eq!(vec!["id"], col_schemata.primary_cols.iter().map(|primary_col|&primary_col.col_name).collect_vec());
            assert_eq!(vec!["col_tinyint", "col_smallint", "col_mediumint", "col_int", "col_bigint"], col_schemata.cols.iter().map(|col| &col.col_name).collect_vec());

            let row_snapshots = adapter.get_row_snapshots(&table_schemata[0], &col_schemata, "limited")?;

            assert_eq!(vec![SimpleNumber(s("127")),  SimpleNumber(s("32767")),  SimpleNumber(s("8388607")),  SimpleNumber(s("2147483647")),  SimpleNumber(s("9223372036854775807"))],  row_snapshots[0].col_values);
            assert_eq!(vec![SimpleNumber(s("-128")), SimpleNumber(s("-32768")), SimpleNumber(s("-8388608")), SimpleNumber(s("-2147483648")), SimpleNumber(s("-9223372036854775808"))], row_snapshots[1].col_values);
        }

        {
            assert_eq!("02_number_02_unsigned", table_schemata[1].table_name);

            let col_schemata = adapter.get_col_schemata(&table_schemata[1])?;

            assert_eq!(vec!["id"], col_schemata.primary_cols.iter().map(|primary_col|&primary_col.col_name).collect_vec());
            assert_eq!(vec!["col_tinyint", "col_smallint", "col_mediumint", "col_int", "col_bigint"], col_schemata.cols.iter().map(|col| &col.col_name).collect_vec());

            let row_snapshots = adapter.get_row_snapshots(&table_schemata[1], &col_schemata, "limited")?;

            assert_eq!(vec![SimpleNumber(s("255")), SimpleNumber(s("65535")), SimpleNumber(s("16777215")), SimpleNumber(s("4294967295")), SimpleNumber(s("18446744073709551615"))], row_snapshots[0].col_values);
            assert_eq!(vec![SimpleNumber(s("0")),   SimpleNumber(s("0")),     SimpleNumber(s("0")),        SimpleNumber(s("0")),          SimpleNumber(s("0"))],                    row_snapshots[1].col_values);
        }

        {
            assert_eq!("03_number_03_fixed", table_schemata[2].table_name);

            let col_schemata = adapter.get_col_schemata(&table_schemata[2])?;

            assert_eq!(vec!["id"], col_schemata.primary_cols.iter().map(|primary_col|&primary_col.col_name).collect_vec());
            assert_eq!(vec!["col_decimal", "col_numeric"], col_schemata.cols.iter().map(|col| &col.col_name).collect_vec());

            let row_snapshots = adapter.get_row_snapshots(&table_schemata[2], &col_schemata, "limited")?;

            assert_eq!(vec![SimpleNumber(s("999.99")),  SimpleNumber(s("999.99"))],  row_snapshots[0].col_values);
            assert_eq!(vec![SimpleNumber(s("-999.99")), SimpleNumber(s("-999.99"))], row_snapshots[1].col_values);
        }

        {
            assert_eq!("04_number_04_float", table_schemata[3].table_name);

            let col_schemata = adapter.get_col_schemata(&table_schemata[3])?;

            assert_eq!(vec!["id"], col_schemata.primary_cols.iter().map(|primary_col|&primary_col.col_name).collect_vec());
            assert_eq!(vec!["col_float", "col_double"], col_schemata.cols.iter().map(|col| &col.col_name).collect_vec());

            let row_snapshots = adapter.get_row_snapshots(&table_schemata[3], &col_schemata, "limited")?;

            assert_eq!(vec![SimpleNumber(s("999.99")),  SimpleNumber(s("999.99"))],  row_snapshots[0].col_values);
            assert_eq!(vec![SimpleNumber(s("-999.99")), SimpleNumber(s("-999.99"))], row_snapshots[1].col_values);
        }

        {
            assert_eq!("05_number_05_bit", table_schemata[4].table_name);

            let col_schemata = adapter.get_col_schemata(&table_schemata[4])?;

            assert_eq!(vec!["id"], col_schemata.primary_cols.iter().map(|primary_col|&primary_col.col_name).collect_vec());
            assert_eq!(vec!["col_bit"], col_schemata.cols.iter().map(|col| &col.col_name).collect_vec());

            let row_snapshots = adapter.get_row_snapshots(&table_schemata[4], &col_schemata, "limited")?;

            assert_eq!(vec![BitNumber(s("1000000000"))], row_snapshots[0].col_values);
            assert_eq!(vec![BitNumber(s("0"))],          row_snapshots[1].col_values);
            assert_eq!(vec![BitNumber(s("1000000000"))], row_snapshots[2].col_values);
            assert_eq!(vec![BitNumber(s("0"))],          row_snapshots[3].col_values);
        }

        {
            assert_eq!("06_date_01_date", table_schemata[5].table_name);

            let col_schemata = adapter.get_col_schemata(&table_schemata[5])?;

            assert_eq!(vec!["id"], col_schemata.primary_cols.iter().map(|primary_col|&primary_col.col_name).collect_vec());
            assert_eq!(vec!["col_date"], col_schemata.cols.iter().map(|col| &col.col_name).collect_vec());

            let row_snapshots = adapter.get_row_snapshots(&table_schemata[5], &col_schemata, "limited")?;

            assert_eq!(vec![DateString(s("2020-01-01"))], row_snapshots[0].col_values);
        }

        {
            assert_eq!("07_date_02_time", table_schemata[6].table_name);

            let col_schemata = adapter.get_col_schemata(&table_schemata[6])?;

            assert_eq!(vec!["id"], col_schemata.primary_cols.iter().map(|primary_col|&primary_col.col_name).collect_vec());
            assert_eq!(vec!["col_time"], col_schemata.cols.iter().map(|col| &col.col_name).collect_vec());

            let row_snapshots = adapter.get_row_snapshots(&table_schemata[6], &col_schemata, "limited")?;

            assert_eq!(vec![DateString(s("00:00:00"))], row_snapshots[0].col_values);
        }

        {
            assert_eq!("08_date_03_datetime", table_schemata[7].table_name);

            let col_schemata = adapter.get_col_schemata(&table_schemata[7])?;

            assert_eq!(vec!["id"], col_schemata.primary_cols.iter().map(|primary_col|&primary_col.col_name).collect_vec());
            assert_eq!(vec!["col_datetime"], col_schemata.cols.iter().map(|col| &col.col_name).collect_vec());

            let row_snapshots = adapter.get_row_snapshots(&table_schemata[7], &col_schemata, "limited")?;

            assert_eq!(vec![DateString(s("2020-01-01 00:00:00"))], row_snapshots[0].col_values);
        }

        {
            assert_eq!("09_date_04_timestamp", table_schemata[8].table_name);

            let col_schemata = adapter.get_col_schemata(&table_schemata[8])?;

            assert_eq!(vec!["id"], col_schemata.primary_cols.iter().map(|primary_col|&primary_col.col_name).collect_vec());
            assert_eq!(vec!["col_timestamp"], col_schemata.cols.iter().map(|col| &col.col_name).collect_vec());

            let row_snapshots = adapter.get_row_snapshots(&table_schemata[8], &col_schemata, "limited")?;

            assert_eq!(vec![DateString(s("2020-01-01 00:00:00"))], row_snapshots[0].col_values);
        }

        {
            assert_eq!("10_date_05_year", table_schemata[9].table_name);

            let col_schemata = adapter.get_col_schemata(&table_schemata[9])?;

            assert_eq!(vec!["id"], col_schemata.primary_cols.iter().map(|primary_col|&primary_col.col_name).collect_vec());
            assert_eq!(vec!["col_year"], col_schemata.cols.iter().map(|col| &col.col_name).collect_vec());

            let row_snapshots = adapter.get_row_snapshots(&table_schemata[9], &col_schemata, "limited")?;

            assert_eq!(vec![DateString(s("2020"))], row_snapshots[0].col_values);
        }

        {
            assert_eq!("11_string_01_char", table_schemata[10].table_name);

            let col_schemata = adapter.get_col_schemata(&table_schemata[10])?;

            assert_eq!(vec!["id"], col_schemata.primary_cols.iter().map(|primary_col|&primary_col.col_name).collect_vec());
            assert_eq!(vec!["col_char", "col_varchar"], col_schemata.cols.iter().map(|col| &col.col_name).collect_vec());

            let row_snapshots = adapter.get_row_snapshots(&table_schemata[10], &col_schemata, "limited")?;

            assert_eq!(vec![SimpleString(s("abc")), SimpleString(s("abc"))], row_snapshots[0].col_values);
            assert_eq!(vec![SimpleString(s("")), SimpleString(s(""))],       row_snapshots[1].col_values);
            assert_eq!(vec![Null, Null],                                     row_snapshots[2].col_values);
        }

        {
            assert_eq!("12_string_02_binary", table_schemata[11].table_name);

            let col_schemata = adapter.get_col_schemata(&table_schemata[11])?;

            assert_eq!(vec!["id"], col_schemata.primary_cols.iter().map(|primary_col|&primary_col.col_name).collect_vec());
            assert_eq!(vec!["col_binary", "col_varbinary"], col_schemata.cols.iter().map(|col| &col.col_name).collect_vec());

            let row_snapshots = adapter.get_row_snapshots(&table_schemata[11], &col_schemata, "limited")?;

            assert_eq!(vec![BinaryString(s("abc")), BinaryString(s("abc"))], row_snapshots[0].col_values);
        }

        {
            assert_eq!("13_string_03_blob", table_schemata[12].table_name);

            let col_schemata = adapter.get_col_schemata(&table_schemata[12])?;

            assert_eq!(vec!["id"], col_schemata.primary_cols.iter().map(|primary_col|&primary_col.col_name).collect_vec());
            assert_eq!(vec!["col_tinyblob", "col_blob", "col_mediumblob", "col_longblob"], col_schemata.cols.iter().map(|col| &col.col_name).collect_vec());

            let row_snapshots = adapter.get_row_snapshots(&table_schemata[12], &col_schemata, "limited")?;

            assert_eq!(vec![BinaryString(s("abc")), BinaryString(s("abc")), BinaryString(s("abc")), BinaryString(s("abc"))], row_snapshots[0].col_values);
        }

        {
            assert_eq!("14_string_04_text", table_schemata[13].table_name);

            let col_schemata = adapter.get_col_schemata(&table_schemata[13])?;

            assert_eq!(vec!["id"], col_schemata.primary_cols.iter().map(|primary_col|&primary_col.col_name).collect_vec());
            assert_eq!(vec!["col_tinytext", "col_text", "col_mediumtext", "col_longtext"], col_schemata.cols.iter().map(|col| &col.col_name).collect_vec());

            let row_snapshots = adapter.get_row_snapshots(&table_schemata[13], &col_schemata, "limited")?;

            assert_eq!(vec![SimpleString(s("abc")), SimpleString(s("abc")), SimpleString(s("abc")), SimpleString(s("abc"))], row_snapshots[0].col_values);
        }

        {
            assert_eq!("15_string_05_enum", table_schemata[14].table_name);

            let col_schemata = adapter.get_col_schemata(&table_schemata[14])?;

            assert_eq!(vec!["id"], col_schemata.primary_cols.iter().map(|primary_col|&primary_col.col_name).collect_vec());
            assert_eq!(vec!["col_enum"], col_schemata.cols.iter().map(|col| &col.col_name).collect_vec());

            let row_snapshots = adapter.get_row_snapshots(&table_schemata[14], &col_schemata, "limited")?;

            assert_eq!(vec![SimpleString(s("active"))],   row_snapshots[0].col_values);
            assert_eq!(vec![SimpleString(s("inactive"))], row_snapshots[1].col_values);
        }

        {
            assert_eq!("16_string_06_set", table_schemata[15].table_name);

            let col_schemata = adapter.get_col_schemata(&table_schemata[15])?;

            assert_eq!(vec!["id"], col_schemata.primary_cols.iter().map(|primary_col|&primary_col.col_name).collect_vec());
            assert_eq!(vec!["col_set"], col_schemata.cols.iter().map(|col| &col.col_name).collect_vec());

            let row_snapshots = adapter.get_row_snapshots(&table_schemata[15], &col_schemata, "limited")?;

            assert_eq!(vec![SimpleString(s("pc"))],       row_snapshots[0].col_values);
            assert_eq!(vec![SimpleString(s("phone"))],    row_snapshots[1].col_values);
            assert_eq!(vec![SimpleString(s("pc,phone"))], row_snapshots[2].col_values);
            assert_eq!(vec![SimpleString(s("pc,phone"))], row_snapshots[3].col_values);
        }

        {
            assert_eq!("17_string_07_json", table_schemata[16].table_name);

            let col_schemata = adapter.get_col_schemata(&table_schemata[16])?;

            assert_eq!(vec!["id"], col_schemata.primary_cols.iter().map(|primary_col|&primary_col.col_name).collect_vec());
            assert_eq!(vec!["col_json"], col_schemata.cols.iter().map(|col| &col.col_name).collect_vec());

            let row_snapshots = adapter.get_row_snapshots(&table_schemata[16], &col_schemata, "limited")?;

            assert_eq!(vec![JsonString(s(r#"{"id": 1, "name": "John"}"#))],                       row_snapshots[0].col_values);
            assert_eq!(vec![JsonString(s(r#"[1, 2, "foo"]"#))],                                   row_snapshots[1].col_values);
            assert_eq!(vec![JsonString(s(r#"{"items": ["pc", "phone"], "option": {"id": 1}}"#))], row_snapshots[2].col_values);
        }

        {
            assert_eq!("18_key_01_primary", table_schemata[17].table_name);

            let col_schemata = adapter.get_col_schemata(&table_schemata[17])?;

            let (primary_col_name, col_names) = col_schemata.get_all_col_names();

            assert_eq!("code", primary_col_name);
            assert_eq!(0, col_names.len());
        }

        {
            assert_eq!("19_key_02_unique", table_schemata[18].table_name);

            let col_schemata = adapter.get_col_schemata(&table_schemata[18])?;

            let (primary_col_name, col_names) = col_schemata.get_all_col_names();

            assert_eq!("", primary_col_name);
            assert_eq!(vec!["code"], col_names);
        }

        {
            assert_eq!("20_key_03_unique_not_null", table_schemata[19].table_name);

            let col_schemata = adapter.get_col_schemata(&table_schemata[19])?;

            let (primary_col_name, col_names) = col_schemata.get_all_col_names();
            
            assert_eq!("code", primary_col_name);
            assert_eq!(0, col_names.len());
        }

        {
            assert_eq!("21_key_04_primary_primary", table_schemata[20].table_name);

            let col_schemata = adapter.get_col_schemata(&table_schemata[20])?;

            let (primary_col_name, col_names) = col_schemata.get_all_col_names();

            assert_eq!("code1-code2", primary_col_name);
            assert_eq!(0, col_names.len());
        }

        {
            assert_eq!("22_key_05_primary_unique", table_schemata[21].table_name);

            let col_schemata = adapter.get_col_schemata(&table_schemata[21])?;

            let (primary_col_name, col_names) = col_schemata.get_all_col_names();

            assert_eq!("code1", primary_col_name);
            assert_eq!(vec!["code2"], col_names);
        }
        
        {
            assert_eq!("23_key_06_primary_unique_not_null", table_schemata[22].table_name);
        
            let col_schemata = adapter.get_col_schemata(&table_schemata[22])?;

            let (primary_col_name, col_names) = col_schemata.get_all_col_names();

            assert_eq!("code1", primary_col_name);
            assert_eq!(vec!["code2"], col_names);
        }

        {
            assert_eq!("24_key_07_unique_unique", table_schemata[23].table_name);

            let col_schemata = adapter.get_col_schemata(&table_schemata[23])?;

            let (primary_col_name, col_names) = col_schemata.get_all_col_names();
            
            assert_eq!("", primary_col_name);
            assert_eq!(vec!["code1", "code2"], col_names);
        }

        {
            assert_eq!("25_key_08_unique_not_null_unique", table_schemata[24].table_name);

            let col_schemata = adapter.get_col_schemata(&table_schemata[24])?;

            let (primary_col_name, col_names) = col_schemata.get_all_col_names();

            assert_eq!("code1", primary_col_name);
            assert_eq!(vec!["code2"], col_names);
        }

        {
            assert_eq!("26_key_09_unique_not_null_unique_not_null", table_schemata[25].table_name);

            let col_schemata = adapter.get_col_schemata(&table_schemata[25])?;

            let (primary_col_name, col_names) = col_schemata.get_all_col_names();

            assert_eq!("code1", primary_col_name);
            assert_eq!(vec!["code2"], col_names);
        }

        {
            assert_eq!("27_key_10_multi_unique_unique", table_schemata[26].table_name);

            let col_schemata = adapter.get_col_schemata(&table_schemata[26])?;

            let (primary_col_name, col_names) = col_schemata.get_all_col_names();
            
            assert_eq!("", primary_col_name);
            assert_eq!(vec!["code1", "code2"], col_names);
        }

        {
            assert_eq!("28_key_11_multi_unique_not_null_unique", table_schemata[27].table_name);

            let col_schemata = adapter.get_col_schemata(&table_schemata[27])?;

            let (primary_col_name, col_names) = col_schemata.get_all_col_names();
            
            assert_eq!("", primary_col_name);
            assert_eq!(vec!["code1", "code2"], col_names);
        }

        {
            assert_eq!("29_key_12_multi_unique_not_null_unique_not_null", table_schemata[28].table_name);

            let col_schemata = adapter.get_col_schemata(&table_schemata[28])?;

            let (primary_col_name, col_names) = col_schemata.get_all_col_names();
            
            assert_eq!("code1-code2", primary_col_name);
            assert_eq!(0, col_names.len());
        }

        {
            assert_eq!("30_key_13_nothing", table_schemata[29].table_name);

            let col_schemata = adapter.get_col_schemata(&table_schemata[29])?;

            let (primary_col_name, col_names) = col_schemata.get_all_col_names();
            
            assert_eq!("", primary_col_name);
            assert_eq!(vec!["code"], col_names);
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod parse_col_value_tests {
    use crate::domain::schema::ColSchema;
    use crate::domain::snapshot::ColValue;
    use crate::dump::mysql80::parse_col_value;

    fn sut(data_type: &str, col_type: &str, value: &str) -> ColValue {
        parse_col_value(
            &ColSchema { col_name: "col_test".to_string(), data_type: data_type.to_string(), col_type: col_type.to_string() },
            value.to_string(),
        )
    }

    #[test]
    fn parse_i_tinyint() {
        let exp = "42";
        assert_eq!(exp, sut("tinyint", "tinyint", "42").as_display_value());
    }

    #[test]
    fn parse_u_tinyint() {
        let exp = "42";
        assert_eq!(exp, sut("tinyint", "tinyint unsigned", "42").as_display_value());
    }

    #[test]
    fn parse_i_smallint() {
        let exp = "42";
        assert_eq!(exp, sut("smallint", "smallint", "42").as_display_value());
    }

    #[test]
    fn parse_u_smallint() {
        let exp = "42";
        assert_eq!(exp, sut("smallint", "smallint unsigned", "42").as_display_value());
    }

    #[test]
    fn parse_i_mediumint() {
        let exp = "42";
        assert_eq!(exp, sut("mediumint", "mediumint", "42").as_display_value());
    }

    #[test]
    fn parse_u_mediumint() {
        let exp = "42";
        assert_eq!(exp, sut("mediumint", "mediumint unsigned", "42").as_display_value());
    }

    #[test]
    fn parse_i_int() {
        let exp = "42";
        assert_eq!(exp, sut("int", "int", "42").as_display_value());
    }

    #[test]
    fn parse_u_int() {
        let exp = "42";
        assert_eq!(exp, sut("int", "int unsigned", "42").as_display_value());
    }

    #[test]
    fn parse_i_bigint() {
        let exp = "42";
        assert_eq!(exp, sut("bigint", "bigint", "42").as_display_value());
    }

    #[test]
    fn parse_u_bigint() {
        let exp = "42";
        assert_eq!(exp, sut("bigint", "bigint unsigned", "42").as_display_value());
    }

    #[test]
    fn parse_decimal() {
        let exp = "42.0";
        assert_eq!(exp, sut("decimal", "decimal(5,2)", "42.0").as_display_value());
    }

    #[test]
    fn parse_float() {
        let exp = "42.0";
        assert_eq!(exp, sut("float", "float(5,2)", "42.0").as_display_value());
    }

    #[test]
    fn parse_double() {
        let exp = "42.0";
        assert_eq!(exp, sut("double", "double(5,2)", "42.0").as_display_value());
    }

    #[test]
    fn parse_bit() {
        let exp = "bit(111)";
        assert_eq!(exp, sut("bit", "bit(3)", "111").as_display_value());
    }

    #[test]
    fn parse_date() {
        let exp = r#""2020-01-01""#;
        assert_eq!(exp, sut("date", "date", "2020-01-01").as_display_value());
    }

    #[test]
    fn parse_time() {
        let exp = r#""12:34:56""#;
        assert_eq!(exp, sut("time", "time", "12:34:56").as_display_value());
    }

    #[test]
    fn parse_datetime() {
        let exp = r#""2020-01-01 12:34:56""#;
        assert_eq!(exp, sut("datetime", "datetime", "2020-01-01 12:34:56").as_display_value());
    }

    #[test]
    fn parse_timestamp() {
        let exp = r#""2020-01-01 12:34:56""#;
        assert_eq!(exp, sut("timestamp", "timestamp", "2020-01-01 12:34:56").as_display_value());
    }

    #[test]
    fn parse_year() {
        let exp = r#""2020""#;
        assert_eq!(exp, sut("year", "year", "2020").as_display_value());
    }

    #[test]
    fn parse_char() {
        let exp = r#""abc""#;
        assert_eq!(exp, sut("char", "char(3)", "abc").as_display_value());
    }

    #[test]
    fn parse_varchar() {
        let exp = r#""abc""#;
        assert_eq!(exp, sut("varchar", "varchar(3)", "abc").as_display_value());
    }

    #[test]
    fn parse_binary() {
        let exp = "binary";
        assert_eq!(exp, sut("binary", "binary(3)", "abc").as_display_value());
    }

    #[test]
    fn parse_varbinary() {
        let exp = "binary";
        assert_eq!(exp, sut("varbinary", "varbinary(3)", "abc").as_display_value());
    }

    #[test]
    fn parse_tinyblob() {
        let exp = "binary";
        assert_eq!(exp, sut("tinyblob", "tinyblob", "abc").as_display_value());
    }

    #[test]
    fn parse_blob() {
        let exp = "binary";
        assert_eq!(exp, sut("blob", "blob", "abc").as_display_value());
    }

    #[test]
    fn parse_mediumblob() {
        let exp = "binary";
        assert_eq!(exp, sut("mediumblob", "mediumblob", "abc").as_display_value());
    }

    #[test]
    fn parse_longblob() {
        let exp = "binary";
        assert_eq!(exp, sut("longblob", "longblob", "abc").as_display_value());
    }

    #[test]
    fn parse_tinytext() {
        let exp = r#""abc""#;
        assert_eq!(exp, sut("tinytext", "tinytext", "abc").as_display_value());
    }

    #[test]
    fn parse_text() {
        let exp = r#""abc""#;
        assert_eq!(exp, sut("text", "text", "abc").as_display_value());
    }

    #[test]
    fn parse_mediumtext() {
        let exp = r#""abc""#;
        assert_eq!(exp, sut("mediumtext", "mediumtext", "abc").as_display_value());
    }

    #[test]
    fn parse_longtext() {
        let exp = r#""abc""#;
        assert_eq!(exp, sut("longtext", "longtext", "abc").as_display_value());
    }

    #[test]
    fn parse_enum() {
        let exp = r#""abc""#;
        assert_eq!(exp, sut("enum", "enum('abc','def')", "abc").as_display_value());
    }

    #[test]
    fn parse_set() {
        let exp = r#""abc,def""#;
        assert_eq!(exp, sut("set", "set('abc','def')", "abc,def").as_display_value());
    }

    #[test]
    fn parse_json() {
        let exp = r#"{"id": 1, "name": "John"}"#;
        assert_eq!(exp, sut("json", "json", r#"{"id": 1, "name": "John"}"#).as_display_value());
    }
}
