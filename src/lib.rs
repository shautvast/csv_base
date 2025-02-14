pub mod groupby;
pub mod join;
pub mod order;
pub mod print;
pub mod read;
pub mod sql;
pub mod value;

use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
    ops::Add,
};

use value::Value;

pub struct Table {
    name: String,
    cols_by_name: HashMap<String, usize>,
    cols: Vec<String>,
    records: BTreeMap<Key, Record>,
}

impl Table {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            cols_by_name: HashMap::new(),
            cols: vec![],
            records: BTreeMap::new(),
        }
    }

    /// Creates a new table with the same name and columns as self,
    /// but without data
    // Note to self: be careful, might be dangerous to use once tables can be altered.
    // That is not yet implemented. May need full copies
    pub fn empty_copy(&self) -> Self {
        let mut result = Table::new(self.name.clone());
        result.cols_by_name = self.cols_by_name.clone();
        result.cols = self.cols.clone();
        result
    }

    pub fn add_record(&mut self, record: Record) {
        let index = self.records.len();
        self.records.insert(Key::integer(index), record);
    }

    pub fn has_column(&self, name: impl Into<String>) -> bool {
        self.cols_by_name.contains_key(&name.into())
    }

    pub fn add_column(&mut self, name: impl Into<String>, allow_duplicates: bool) {
        let col_index = self.cols.len();
        let orig_name: String = name.into();

        let name = if allow_duplicates {
            // append an index when there are duplicate column names
            let mut col_name = orig_name.to_string();
            let mut index = 2;

            while self.has_column(&col_name) {
                col_name = orig_name.to_string();
                col_name.push_str(format!("{}", index).as_str());
                index += 1;
            }
            col_name
        } else {
            orig_name
        };

        self.cols_by_name.insert(name.clone(), col_index);
        self.cols.push(name);
    }

    fn get_indexes(&self, expression: &str) -> Vec<usize> {
        expression
            .split(",")
            .map(|c| self.get_index(c.trim()))
            .collect::<Vec<usize>>()
    }

    fn get_index(&self, col_name: &str) -> usize {
        *self.cols_by_name.get(col_name).unwrap()
    }

    pub fn iter(&self) -> TableIter {
        self.iter_records()
    }

    pub fn iter_records(&self) -> TableIter {
        TableIter {
            table_iter: self.records.iter(),
        }
    }

    pub fn select_columns<'a>(&'a self, columns: &'a Vec<&'a str>) -> OwnedColIter<'a> {
        OwnedColIter {
            cols: columns,
            index: 0,
        }
    }

    pub fn iter_colums(&self) -> ColIter {
        ColIter {
            cols: &self.cols,
            index: 0,
        }
    }

    pub fn where_clause(&self, colindex: usize, value: &Value) -> Option<&Record> {
        for record in self.iter_records() {
            let r = record.get(colindex);
            if r == value {
                return Some(record);
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct Record {
    values: Vec<Value>,
}

impl Record {
    pub fn len(&self) -> usize {
        self.values.iter().map(Value::len).sum()
    }

    pub fn add_value(&mut self, value: impl Into<Value>) {
        self.values.push(value.into());
    }

    pub fn get(&self, index: usize) -> &Value {
        self.values.get(index).unwrap_or(&Value::NULL)
    }
}

impl Add for &Record {
    type Output = Record;

    fn add(self, rhs: Self) -> Self::Output {
        let mut sum = Record::default();
        sum.values.append(&mut self.values.clone());
        sum.values.append(&mut rhs.values.clone()); // use refs?
        sum
    }
}

impl Default for Record {
    fn default() -> Self {
        Self { values: vec![] }
    }
}

pub struct TableIter<'a> {
    table_iter: std::collections::btree_map::Iter<'a, Key, Record>,
}

impl<'a> Iterator for TableIter<'a> {
    type Item = &'a Record;

    fn next(&mut self) -> Option<Self::Item> {
        self.table_iter.next().map(|e| e.1)
    }
}

pub struct ColIter<'a> {
    cols: &'a Vec<String>,
    index: usize,
}

pub struct OwnedColIter<'a> {
    cols: &'a Vec<&'a str>,
    index: usize,
}

impl<'a> Iterator for ColIter<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(v) = self.cols.get(self.index) {
            self.index += 1;
            Some(v)
        } else {
            None
        }
    }
}

impl<'a> Iterator for OwnedColIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(v) = self.cols.get(self.index) {
            self.index += 1;
            Some(v)
        } else {
            None
        }
    }
}

struct Key {
    values: Vec<Value>,
}

impl Key {
    fn integer(integer: usize) -> Self {
        Self {
            values: vec![Value::Integer(integer as i64)],
        }
    }

    fn compound(keys: Vec<Value>) -> Self {
        Self { values: keys }
    }
}
impl Ord for Key {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl Eq for Key {}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        if self.values.len() != other.values.len() {
            false
        } else {
            for (l, r) in self.values.iter().zip(&other.values) {
                if l != r {
                    return false;
                }
            }
            true
        }
    }
}

impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let len = self.values.len().min(other.values.len());
        for i in 0..len {
            let ord = self
                .values
                .get(i)
                .unwrap()
                .partial_cmp(other.values.get(i).unwrap())
                .unwrap();
            match ord {
                Ordering::Less => {
                    return Some(Ordering::Less);
                }
                Ordering::Greater => {
                    return Some(Ordering::Greater);
                }
                _ => {}
            }
        }
        Some(Ordering::Equal)
    }
}
