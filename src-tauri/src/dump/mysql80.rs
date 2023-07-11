use anyhow::anyhow;
use itertools::Itertools;
use mysql::Value::NULL;
use mysql::{from_row, from_value, Conn, Value};

use crate::domain::project::Project;
use crate::domain::schema::{ColumnSchema, ColumnSchemata, TableSchema};
use crate::domain::snapshot::ColValue::*;
use crate::domain::snapshot::{ColValue, RowSnapshot};
use crate::dump::adapter::TargetDbAdapter;

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
    fn get_table_schemata(&mut self) -> anyhow::Result<Vec<TableSchema>> {
        self.conn
            .query(format!("select table_name from information_schema.tables where table_schema = '{}' order by table_name", self.schema))
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

    fn get_col_schemata(&mut self, table_schema: &TableSchema) -> anyhow::Result<ColumnSchemata> {
        let unique_cols: Vec<ColumnSchema> = self.conn.query(
            format!("select column_name, data_type, column_type from information_schema.columns where table_schema = '{}' and table_name = '{}' and column_key = 'PRI'", self.schema, table_schema.table_name))
            .map(|result| {
                result
                    .map(|x| x.unwrap())
                    .map(|row| {
                        let (column_name, data_type, column_type) = from_row(row);
                        ColumnSchema { col_name: column_name, data_type, column_type }
                    })
                    .collect_vec()
            }).map_err(|e| anyhow!(e))?;
        // todo: case [ no primary col ]
        let primary_col = unique_cols[0].clone();

        let cols: Vec<ColumnSchema> = self.conn.query(
            format!("select column_name, data_type, column_type from information_schema.columns where table_schema = '{}' and table_name = '{}' and column_key = '' order by ordinal_position", self.schema, table_schema.table_name))
            .map(|result| {
                result
                    .map(|x| x.unwrap())
                    .map(|row| {
                        let (column_name, data_type, column_type) = from_row(row);
                        ColumnSchema { col_name: column_name, data_type, column_type }
                    })
                    .collect_vec()
            }).map_err(|e| anyhow!(e))?;

        Ok(ColumnSchemata::new(primary_col, cols))
    }

    fn get_row_snapshots(&mut self, table_schema: &TableSchema, column_schemata: &ColumnSchemata) -> anyhow::Result<Vec<RowSnapshot>> {
        let cols = column_schemata.get_all_col_refs();

        self.conn
            .query(format!("select {} from {}", cols.iter().map(|col| as_select_col(col)).join(","), table_schema.table_name))
            .map(|result| {
                result
                    .map(|x| x.unwrap())
                    .map(|row| {
                        (0..column_schemata.count())
                            .map(|i| {
                                let value: Value = row.get(i).unwrap();
                                if value == NULL {
                                    Null
                                } else {
                                    parse_col_value(cols[i], from_value(value))
                                }
                            })
                            .collect_vec()
                    })
                    .map(RowSnapshot::new)
                    .collect()
            })
            .map_err(|e| anyhow!(e))
    }
}

fn as_select_col(col: &ColumnSchema) -> String {
    match col.data_type.as_str() {
        "bit" => format!("bin({})", col.col_name),
        _ => col.col_name.to_string(),
    }
}

fn parse_col_value(column_schema: &ColumnSchema, value: String) -> ColValue {
    match column_schema.data_type.as_str() {
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

    use crate::domain::project::{create_project_id, Project};
    use crate::domain::project::Rdbms::Mysql;
    use crate::domain::snapshot::ColValue::*;
    use crate::dump::adapter::TargetDbAdapter;
    use crate::dump::mysql80::TargetDbMysql80;

    fn s(s: &str) -> String {
        s.to_string()
    }

    #[test]
    fn test() -> anyhow::Result<()> {
        let project = Project::new(&create_project_id(), "test-project", Mysql, "user","password","127.0.0.1","19001","testdata");

        let mut adapter = TargetDbMysql80::new(&project)?;
        
        // drop all
        for table_schema in adapter.get_table_schemata()? {
            adapter.conn.prep_exec(format!("drop table {}", table_schema.table_name), ())?;
        }

        adapter.conn.prep_exec("create table 01_number_signed ( id int auto_increment, col_tinyint tinyint, col_smallint smallint, col_mediumint mediumint, col_int int, col_bigint bigint, primary key (id) )", ())?;
        adapter.conn.prep_exec("insert into 01_number_signed values (1, 127, 32767, 8388607, 2147483647, 9223372036854775807), (2, -128, -32768, -8388608, -2147483648, -9223372036854775808)", ())?;

        adapter.conn.prep_exec("create table 02_number_unsigned ( id int auto_increment, col_tinyint tinyint unsigned, col_smallint smallint unsigned, col_mediumint mediumint unsigned, col_int int unsigned, col_bigint bigint unsigned, primary key (id) )", ())?;
        adapter.conn.prep_exec("insert into 02_number_unsigned values (1, 255, 65535, 16777215, 4294967295, 18446744073709551615), (2, 0, 0, 0, 0, 0)", ())?;

        adapter.conn.prep_exec("create table 03_number_fixed ( id int auto_increment, col_decimal decimal(5, 2), col_numeric numeric(5, 2), primary key (id) )", ())?;
        adapter.conn.prep_exec("insert into 03_number_fixed values (1, 999.99, 999.99), (2, -999.99, -999.99)", ())?;

        adapter.conn.prep_exec("create table 04_number_float ( id int auto_increment, col_float float(5, 2), col_double double(5, 2), primary key (id) )", ())?;
        adapter.conn.prep_exec("insert into 04_number_float values (1, 999.99, 999.99), (2, -999.99, -999.99)", ())?;

        adapter.conn.prep_exec("create table 05_number_bit ( id int auto_increment, col_bit bit(10), primary key (id) )", ())?;
        adapter.conn.prep_exec("insert into 05_number_bit values (1, b'1000000000'), (2, b'0'), (3, 512), (4, 0)", ())?;

        adapter.conn.prep_exec("create table 06_date_date ( id int auto_increment, col_date date, primary key (id) )", ())?;
        adapter.conn.prep_exec("insert into 06_date_date values (1, '2020-01-01')", ())?;

        adapter.conn.prep_exec("create table 07_date_time ( id int auto_increment, col_time time, primary key (id) )", ())?;
        adapter.conn.prep_exec("insert into 07_date_time values (1, '00:00:00')", ())?;

        adapter.conn.prep_exec("create table 08_date_datetime ( id int auto_increment, col_datetime datetime, primary key (id) )", ())?;
        adapter.conn.prep_exec("insert into 08_date_datetime values (1, '2020-01-01 00:00:00')", ())?;

        adapter.conn.prep_exec("create table 09_date_timestamp ( id int auto_increment, col_timestamp timestamp, primary key (id) )", ())?;
        adapter.conn.prep_exec("insert into 09_date_timestamp values (1, '2020-01-01 00:00:00')", ())?;

        adapter.conn.prep_exec("create table 10_date_year ( id int auto_increment, col_year year, primary key (id) )", ())?;
        adapter.conn.prep_exec("insert into 10_date_year values (1, 2020)", ())?;

        adapter.conn.prep_exec("create table 11_string_char ( id int auto_increment, col_char char(3), col_varchar varchar(3), primary key (id) )", ())?;
        adapter.conn.prep_exec("insert into 11_string_char values (1, 'abc', 'abc'), (2, '', '')", ())?;

        adapter.conn.prep_exec("create table 12_string_binary ( id int auto_increment, col_binary binary(3), col_varbinary varbinary(3), primary key (id) )", ())?;
        adapter.conn.prep_exec("insert into 12_string_binary values (1, 'abc', 'abc')", ())?;

        adapter.conn.prep_exec("create table 13_string_blob ( id int auto_increment, col_tinyblob tinyblob, col_blob blob, col_mediumblob mediumblob, col_longblob longblob, primary key (id) )", ())?;
        adapter.conn.prep_exec("insert into 13_string_blob values (1, 'abc', 'abc', 'abc', 'abc')", ())?;

        adapter.conn.prep_exec("create table 14_string_text ( id int auto_increment, col_tinytext tinytext, col_text text, col_mediumtext mediumtext, col_longtext longtext, primary key (id) )", ())?;
        adapter.conn.prep_exec("insert into 14_string_text values (1, 'abc', 'abc', 'abc', 'abc')", ())?;

        adapter.conn.prep_exec("create table 15_string_enum ( id int auto_increment, col_enum enum ('active', 'inactive'), primary key (id) )", ())?;
        adapter.conn.prep_exec("insert into 15_string_enum values (1, 'active'), (2, 'inactive')", ())?;

        adapter.conn.prep_exec("create table 16_string_set ( id int auto_increment, col_set set ('pc', 'phone'), primary key (id) )", ())?;
        adapter.conn.prep_exec("insert into 16_string_set values (1, 'pc'), (2, 'phone'), (3, 'phone,pc'), (4, 'pc,phone')", ())?;

        adapter.conn.prep_exec("create table 17_json_json ( id int auto_increment, col_json json, primary key (id) )", ())?;
        adapter.conn.prep_exec(r#"insert into 17_json_json values (1, '{"id": 1, "name": "John"}')"#, ())?;
        adapter.conn.prep_exec(r#"insert into 17_json_json values (2, '[1, 2, "foo"]')"#, ())?;
        adapter.conn.prep_exec(r#"insert into 17_json_json values (3, '{"items": ["pc", "phone"], "option": {"id": 1}}')"#, ())?;

        adapter.conn.prep_exec("create table 18_empty ( id int auto_increment, name varchar(3), primary key (id) )", ())?;
        adapter.conn.prep_exec("insert into 18_empty values (1, '')", ())?;
        adapter.conn.prep_exec("insert into 18_empty values (2, null)", ())?;
        
        let table_schemata = adapter.get_table_schemata()?;
        
        {
            assert_eq!("01_number_signed", table_schemata[0].table_name);
            
            let column_schemata = adapter.get_col_schemata(&table_schemata[0])?;

            assert_eq!("id", column_schemata.primary_col.col_name);
            assert_eq!("col_tinyint, col_smallint, col_mediumint, col_int, col_bigint", column_schemata.cols.iter().map(|col| &col.col_name).join(", "));

            let row_snapshots = adapter.get_row_snapshots(&table_schemata[0], &column_schemata)?;

            assert_eq!(vec![SimpleNumber(s("127")),  SimpleNumber(s("32767")),  SimpleNumber(s("8388607")),  SimpleNumber(s("2147483647")),  SimpleNumber(s("9223372036854775807"))],  row_snapshots[0].col_values);
            assert_eq!(vec![SimpleNumber(s("-128")), SimpleNumber(s("-32768")), SimpleNumber(s("-8388608")), SimpleNumber(s("-2147483648")), SimpleNumber(s("-9223372036854775808"))], row_snapshots[1].col_values);
        }

        {
            assert_eq!("02_number_unsigned", table_schemata[1].table_name);

            let column_schemata = adapter.get_col_schemata(&table_schemata[1])?;

            assert_eq!("id", column_schemata.primary_col.col_name);
            assert_eq!("col_tinyint, col_smallint, col_mediumint, col_int, col_bigint", column_schemata.cols.iter().map(|col| &col.col_name).join(", "));

            let row_snapshots = adapter.get_row_snapshots(&table_schemata[1], &column_schemata)?;

            assert_eq!(vec![SimpleNumber(s("255")), SimpleNumber(s("65535")), SimpleNumber(s("16777215")), SimpleNumber(s("4294967295")), SimpleNumber(s("18446744073709551615"))], row_snapshots[0].col_values);
            assert_eq!(vec![SimpleNumber(s("0")),   SimpleNumber(s("0")),     SimpleNumber(s("0")),        SimpleNumber(s("0")),          SimpleNumber(s("0"))],                    row_snapshots[1].col_values);
        }

        {
            assert_eq!("03_number_fixed", table_schemata[2].table_name);

            let column_schemata = adapter.get_col_schemata(&table_schemata[2])?;

            assert_eq!("id", column_schemata.primary_col.col_name);
            assert_eq!("col_decimal, col_numeric", column_schemata.cols.iter().map(|col| &col.col_name).join(", "));

            let row_snapshots = adapter.get_row_snapshots(&table_schemata[2], &column_schemata)?;

            assert_eq!(vec![SimpleNumber(s("999.99")),  SimpleNumber(s("999.99"))],  row_snapshots[0].col_values);
            assert_eq!(vec![SimpleNumber(s("-999.99")), SimpleNumber(s("-999.99"))], row_snapshots[1].col_values);
        }

        {
            assert_eq!("04_number_float", table_schemata[3].table_name);

            let column_schemata = adapter.get_col_schemata(&table_schemata[3])?;

            assert_eq!("id", column_schemata.primary_col.col_name);
            assert_eq!("col_float, col_double", column_schemata.cols.iter().map(|col| &col.col_name).join(", "));

            let row_snapshots = adapter.get_row_snapshots(&table_schemata[3], &column_schemata)?;

            assert_eq!(vec![SimpleNumber(s("999.99")),  SimpleNumber(s("999.99"))],  row_snapshots[0].col_values);
            assert_eq!(vec![SimpleNumber(s("-999.99")), SimpleNumber(s("-999.99"))], row_snapshots[1].col_values);
        }

        {
            assert_eq!("05_number_bit", table_schemata[4].table_name);

            let column_schemata = adapter.get_col_schemata(&table_schemata[4])?;

            assert_eq!("id", column_schemata.primary_col.col_name);
            assert_eq!("col_bit", column_schemata.cols.iter().map(|col| &col.col_name).join(", "));

            let row_snapshots = adapter.get_row_snapshots(&table_schemata[4], &column_schemata)?;

            assert_eq!(vec![BitNumber(s("1000000000"))], row_snapshots[0].col_values);
            assert_eq!(vec![BitNumber(s("0"))],          row_snapshots[1].col_values);
            assert_eq!(vec![BitNumber(s("1000000000"))], row_snapshots[2].col_values);
            assert_eq!(vec![BitNumber(s("0"))],          row_snapshots[3].col_values);
        }

        {
            assert_eq!("06_date_date", table_schemata[5].table_name);

            let column_schemata = adapter.get_col_schemata(&table_schemata[5])?;

            assert_eq!("id", column_schemata.primary_col.col_name);
            assert_eq!("col_date", column_schemata.cols.iter().map(|col| &col.col_name).join(", "));

            let row_snapshots = adapter.get_row_snapshots(&table_schemata[5], &column_schemata)?;

            assert_eq!(vec![DateString(s("2020-01-01"))], row_snapshots[0].col_values);
        }

        {
            assert_eq!("07_date_time", table_schemata[6].table_name);

            let column_schemata = adapter.get_col_schemata(&table_schemata[6])?;

            assert_eq!("id", column_schemata.primary_col.col_name);
            assert_eq!("col_time", column_schemata.cols.iter().map(|col| &col.col_name).join(", "));

            let row_snapshots = adapter.get_row_snapshots(&table_schemata[6], &column_schemata)?;

            assert_eq!(vec![DateString(s("00:00:00"))], row_snapshots[0].col_values);
        }

        {
            assert_eq!("08_date_datetime", table_schemata[7].table_name);

            let column_schemata = adapter.get_col_schemata(&table_schemata[7])?;

            assert_eq!("id", column_schemata.primary_col.col_name);
            assert_eq!("col_datetime", column_schemata.cols.iter().map(|col| &col.col_name).join(", "));

            let row_snapshots = adapter.get_row_snapshots(&table_schemata[7], &column_schemata)?;

            assert_eq!(vec![DateString(s("2020-01-01 00:00:00"))], row_snapshots[0].col_values);
        }

        {
            assert_eq!("09_date_timestamp", table_schemata[8].table_name);

            let column_schemata = adapter.get_col_schemata(&table_schemata[8])?;

            assert_eq!("id", column_schemata.primary_col.col_name);
            assert_eq!("col_timestamp", column_schemata.cols.iter().map(|col| &col.col_name).join(", "));

            let row_snapshots = adapter.get_row_snapshots(&table_schemata[8], &column_schemata)?;

            assert_eq!(vec![DateString(s("2020-01-01 00:00:00"))], row_snapshots[0].col_values);
        }

        {
            assert_eq!("10_date_year", table_schemata[9].table_name);

            let column_schemata = adapter.get_col_schemata(&table_schemata[9])?;

            assert_eq!("id", column_schemata.primary_col.col_name);
            assert_eq!("col_year", column_schemata.cols.iter().map(|col| &col.col_name).join(", "));

            let row_snapshots = adapter.get_row_snapshots(&table_schemata[9], &column_schemata)?;

            assert_eq!(vec![DateString(s("2020"))], row_snapshots[0].col_values);
        }

        {
            assert_eq!("11_string_char", table_schemata[10].table_name);

            let column_schemata = adapter.get_col_schemata(&table_schemata[10])?;

            assert_eq!("id", column_schemata.primary_col.col_name);
            assert_eq!("col_char, col_varchar", column_schemata.cols.iter().map(|col| &col.col_name).join(", "));

            let row_snapshots = adapter.get_row_snapshots(&table_schemata[10], &column_schemata)?;

            assert_eq!(vec![SimpleString(s("abc")), SimpleString(s("abc"))], row_snapshots[0].col_values);
            assert_eq!(vec![SimpleString(s("")), SimpleString(s(""))],       row_snapshots[1].col_values);
        }

        {
            assert_eq!("12_string_binary", table_schemata[11].table_name);

            let column_schemata = adapter.get_col_schemata(&table_schemata[11])?;

            assert_eq!("id", column_schemata.primary_col.col_name);
            assert_eq!("col_binary, col_varbinary", column_schemata.cols.iter().map(|col| &col.col_name).join(", "));

            let row_snapshots = adapter.get_row_snapshots(&table_schemata[11], &column_schemata)?;

            assert_eq!(vec![BinaryString(s("abc")), BinaryString(s("abc"))], row_snapshots[0].col_values);
        }

        {
            assert_eq!("13_string_blob", table_schemata[12].table_name);

            let column_schemata = adapter.get_col_schemata(&table_schemata[12])?;

            assert_eq!("id", column_schemata.primary_col.col_name);
            assert_eq!("col_tinyblob, col_blob, col_mediumblob, col_longblob", column_schemata.cols.iter().map(|col| &col.col_name).join(", "));

            let row_snapshots = adapter.get_row_snapshots(&table_schemata[12], &column_schemata)?;

            assert_eq!(vec![BinaryString(s("abc")), BinaryString(s("abc")), BinaryString(s("abc")), BinaryString(s("abc"))], row_snapshots[0].col_values);
        }

        {
            assert_eq!("14_string_text", table_schemata[13].table_name);

            let column_schemata = adapter.get_col_schemata(&table_schemata[13])?;

            assert_eq!("id", column_schemata.primary_col.col_name);
            assert_eq!("col_tinytext, col_text, col_mediumtext, col_longtext", column_schemata.cols.iter().map(|col| &col.col_name).join(", "));

            let row_snapshots = adapter.get_row_snapshots(&table_schemata[13], &column_schemata)?;

            assert_eq!(vec![SimpleString(s("abc")), SimpleString(s("abc")), SimpleString(s("abc")), SimpleString(s("abc"))], row_snapshots[0].col_values);
        }

        {
            assert_eq!("15_string_enum", table_schemata[14].table_name);

            let column_schemata = adapter.get_col_schemata(&table_schemata[14])?;

            assert_eq!("id", column_schemata.primary_col.col_name);
            assert_eq!("col_enum", column_schemata.cols.iter().map(|col| &col.col_name).join(", "));

            let row_snapshots = adapter.get_row_snapshots(&table_schemata[14], &column_schemata)?;

            assert_eq!(vec![SimpleString(s("active"))],   row_snapshots[0].col_values);
            assert_eq!(vec![SimpleString(s("inactive"))], row_snapshots[1].col_values);
        }

        {
            assert_eq!("16_string_set", table_schemata[15].table_name);

            let column_schemata = adapter.get_col_schemata(&table_schemata[15])?;

            assert_eq!("id", column_schemata.primary_col.col_name);
            assert_eq!("col_set", column_schemata.cols.iter().map(|col| &col.col_name).join(", "));

            let row_snapshots = adapter.get_row_snapshots(&table_schemata[15], &column_schemata)?;

            assert_eq!(vec![SimpleString(s("pc"))],       row_snapshots[0].col_values);
            assert_eq!(vec![SimpleString(s("phone"))],    row_snapshots[1].col_values);
            assert_eq!(vec![SimpleString(s("pc,phone"))], row_snapshots[2].col_values);
            assert_eq!(vec![SimpleString(s("pc,phone"))], row_snapshots[3].col_values);
        }

        {
            assert_eq!("17_json_json", table_schemata[16].table_name);

            let column_schemata = adapter.get_col_schemata(&table_schemata[16])?;

            assert_eq!("id", column_schemata.primary_col.col_name);
            assert_eq!("col_json", column_schemata.cols.iter().map(|col| &col.col_name).join(", "));

            let row_snapshots = adapter.get_row_snapshots(&table_schemata[16], &column_schemata)?;

            assert_eq!(vec![JsonString(s(r#"{"id": 1, "name": "John"}"#))],                       row_snapshots[0].col_values);
            assert_eq!(vec![JsonString(s(r#"[1, 2, "foo"]"#))],                                   row_snapshots[1].col_values);
            assert_eq!(vec![JsonString(s(r#"{"items": ["pc", "phone"], "option": {"id": 1}}"#))], row_snapshots[2].col_values);
        }

        {
            assert_eq!("18_empty", table_schemata[17].table_name);

            let column_schemata = adapter.get_col_schemata(&table_schemata[17])?;

            assert_eq!("id", column_schemata.primary_col.col_name);
            assert_eq!("name", column_schemata.cols.iter().map(|col| &col.col_name).join(", "));

            let row_snapshots = adapter.get_row_snapshots(&table_schemata[17], &column_schemata)?;

            assert_eq!(vec![SimpleString(s(""))], row_snapshots[0].col_values);
            assert_eq!(vec![Null],                row_snapshots[1].col_values);
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod parse_col_value_tests {
    use crate::domain::schema::ColumnSchema;
    use crate::domain::snapshot::ColValue;
    use crate::dump::mysql80::parse_col_value;

    fn sut(data_type: &str, column_type: &str, value: &str) -> ColValue {
        parse_col_value(
            &ColumnSchema { col_name: "col_test".to_string(), data_type: data_type.to_string(), column_type: column_type.to_string() },
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
