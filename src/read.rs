use crate::table::Table;

use std::fs;
use crate::record::Record;

const EOL: &str = "\n";

impl Table {
    pub fn from_csv_file(name: &str, separator: Option<&str>) -> anyhow::Result<Table> {
        let csv = fs::read_to_string(name)?;

        Ok(Table::from_csv(csv, separator))
    }

    pub fn from_csv(csv: impl Into<String>, separator: Option<&str>) -> Self {
        let csv = csv.into();
        let separator = separator.unwrap_or(
            guess_separator(&csv)
                .expect("You did not give me a separator and I could not guess it from the data"),
        );
        let mut table = Table::new("");
        for (index, row) in csv.split(EOL).enumerate() {
            if index == 0 {
                for col in row.split(separator) {
                    table.add_column(col, true);
                }
            } else if row.len() > 0 {
                // skip empty lines
                let mut record = Record::default();
                for value in row.split(separator) {
                    //TODO quoted values
                    record.add_value(value);
                }
                table.add_record(record);
            }
        }
        table
    }
}

fn guess_separator(csv: &String) -> Option<&'static str> {
    let mut tabs = 0;
    let mut semis = 0;
    let mut commas = 0;
    let mut pipes = 0;
    for c in csv.chars() {
        match c {
            '\t' => tabs += 1,
            ';' => semis += 1,
            ',' => commas += 1,
            '|' => pipes += 1,
            _ => {}
        }
    }
    let values = vec![(tabs, 0), (semis, 1), (commas, 2), (pipes, 3)];
    values.iter().max().map(|m| match m.1 {
        0 => "\t",
        1 => ";",
        2 => ",",
        3 => "|",
        _ => "\0", //?
    })
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        guess_separator(&"a,b,c|d".to_string());
    }
}
