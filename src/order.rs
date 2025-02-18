use std::collections::BTreeMap;

use crate::table::{Key, Table};

impl Table {
    pub fn order_by(&self, expression: &str) -> Self {
        let indexes = self.get_column_indexes(expression);
        if self.views.contains_key(expression) {}

        let mut sorted_records = BTreeMap::new();
        for record in self.iter() {
            let key = indexes.iter().map(|i| record.get(*i).clone()).collect();
            sorted_records.insert(Key::compound(key), record.clone());
        }
        let mut ordered = Table::empty_copy(self);
        ordered.records = sorted_records;
        ordered
    }
}
