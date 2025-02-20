use crate::table::Table;

impl Table {
    // pub fn left_join(&self, right: &Table, left_col: &str, right_col: &str, outer: bool) -> Table {
    //     join(self, right, left_col, right_col, outer)
    // }
    //
    // pub fn right_join(&self, right: &Table, left_col: &str, right_col: &str, outer: bool) -> Table {
    //     join(right, self, right_col, left_col, outer)
    // }
}

// pub fn join(left: &Table, right: &Table, left_col: &str, right_col: &str, outer: bool) -> Table {
    // let mut joined = Table::new("join");
    // left.cols.iter().for_each(|c| joined.add_column(c, true));
    // right.cols.iter().for_each(|c| joined.add_column(c, true));
    // let left_col_index = left.get_index(left_col);
    // let right_col_index = right.get_index(right_col);
    //
    // for record in left.iter_records() {
    //     let lv = record.get(left_col_index);
    //     if let Some(right_record) = right.where_clause(right_col_index, lv) {
    //         joined.add_record(record + right_record);
    //     } else if outer {
    //         joined.add_record(record.clone());
    //     }
    // }
    // joined
// }
