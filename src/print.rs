use std::collections::HashMap;

use crate::table::Table;

impl Table {
    /// prints the table contents in nice columns on the command line
    pub fn select(&self, expression: &str) {
        if expression == "*" {
            self.pretty_print_all();
        } else {
            let cols = expression
                .split(",")
                .map(|c| c.trim())
                .collect::<Vec<&str>>();
            cols.iter()
                .filter(|c| !self.has_column(**c))
                .any(|invalid| panic!("{} is not a column in this table", invalid));
            self.pretty_print_select(cols);
        }
    }

    fn pretty_print_all(&self) {
        let column_widths = self.get_column_widths(0, usize::MAX);
        // let total = column_widths.values().iter();
        for col in self.iter_colums() {
            let w = column_widths.get(col).unwrap_or(&0);
            print!("| {:<w$} ", col);
        }
        println!("|");
        for record in self.iter() {
            for col in self.iter_colums() {
                let w = column_widths.get(col).unwrap_or(&0);
                // eprintln!("{}", w);
                print!("| {:<w$} ", record.get(self.get_index(col)).to_string());
            }
            println!("|");
        }
    }

    fn pretty_print_select(&self, columns: Vec<&str>) {
        let column_widths = self.select_column_widths(0, usize::MAX, &columns);
        // let total = column_widths.values().iter();
        for col in self.select_columns(&columns) {
            let w = column_widths.get(col).unwrap_or(&0);
            print!("| {:<w$} ", col);
        }
        println!("|");
        for record in self.iter() {
            for col in self.select_columns(&columns) {
                let w = column_widths.get(col).unwrap_or(&0);
                // eprintln!("{}", w);
                print!("| {:<w$} ", record.get(self.get_index(col)).to_string());
            }
            println!("|");
        }
    }

    /// returns a map of column index -> max length of column name/value in any of the rows
    /// needed for printing nice columns
    /// the following parameters allow for paging views
    /// offset: start at rowindex
    /// nrecords: take n records after offset
    fn get_column_widths(&self, offset: usize, nrecords: usize) -> HashMap<&String, usize> {
        let mut widths = HashMap::new();
        // initialize count with the length of the column name
        for col in self.iter_colums() {
            widths.insert(col, col.len());
        }
        for record in self.iter().skip(offset).take(nrecords) {
            for col in self.iter_colums() {
                let e = widths.get_mut(&col).unwrap();
                let index = self.get_index(col);
                *e = (*e).max(record.get(index).string_len());
            }
        }
        widths
    }

    // returns a map of column index -> max length of column name/value in any of the rows
    /// needed for printing nice columns
    /// the following parameters allow for paging views
    /// offset: start at rowindex
    /// nrecords: take n records after offset
    fn select_column_widths<'a>(
        &'a self,
        offset: usize,
        nrecords: usize,
        columns: &'a Vec<&'a str>,
    ) -> HashMap<&'a str, usize> {
        let mut widths = HashMap::new();
        // initialize count with the length of the column name
        for col in self.select_columns(columns) {
            widths.insert(col, col.len());
        }
        for record in self.iter().skip(offset).take(nrecords) {
            for col in self.select_columns(columns) {
                let e = widths.get_mut(&col).unwrap();
                let index = self.get_index(&col);
                *e = (*e).max(record.get(index).string_len());
            }
        }
        widths
    }
}
