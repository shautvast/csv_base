use std::ops::Add;

use crate::value::{Value, NULL};

const PAGE_SIZE: usize = 4096;

pub enum PageType {
    Root,
    Interior,
    Leaf,
}

pub struct Page {
    pagetype: PageType,
    data: Vec<u8>,
    index_pos: u16,
    data_pos: u16,
    key: usize,
    children: Vec<Page>,
}

impl Page {
    pub fn new(pagetype: PageType) -> Self {
        Self {
            pagetype,
            data: vec![0; PAGE_SIZE],
            index_pos: 0,
            data_pos: (PAGE_SIZE - 1) as u16,
            key: 0,
            children: vec![],
        }
    }

    pub fn add_record(&mut self, record: Record) {}
}

#[derive(Debug, Clone)]
pub struct Record {
    values: Vec<Value>,
}

impl Record {
    pub fn string_len(&self) -> usize {
        self.values.iter().map(Value::string_len).sum()
    }

    pub fn add_value(&mut self, value: impl Into<Value>) {
        self.values.push(value.into());
    }

    pub fn get(&self, index: usize) -> Value {
        self.values.get(index).map(|v| v.clone()).unwrap_or(NULL)
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
