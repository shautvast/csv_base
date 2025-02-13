use crate::{Record, Table};

impl Table {
    pub fn from_csv(csv: &str, separator: &str) -> Self {
        let mut table = Table::new("test");
        for (index, row) in csv.split("\n").enumerate() {
            if index == 0 {
                for col in row.split(separator) {
                    table.add_column(col, true);
                }
            } else if row.len() > 0 {
                let mut record = Record::default();
                for value in row.split(separator) {
                    record.add_value(value);
                }
                table.add_record(record);
            }
        }
        table
    }
}
