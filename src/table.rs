use crate::id_sequence::ThreadSafeIdGenerator;
use crate::page::{Page, PageType};
use crate::record::Record;
use crate::value::Value;
use std::cell::RefCell;
use std::rc::Rc;
use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap}
};

#[derive(Debug)]
pub struct View {
    records: BTreeMap<Key, Key>,
}

#[derive(Debug)]
pub struct Table {
    name: String,
    cols_by_name: HashMap<String, usize>,
    pub(crate) cols: Vec<String>,
    pub(crate) root: Rc<RefCell<Page>>,
    pub views: HashMap<String, View>,
    page_ids: ThreadSafeIdGenerator,
    row_ids: ThreadSafeIdGenerator,
    current_page: Rc<RefCell<Page>>,
}

impl Table {
    pub fn new(name: impl Into<String>) -> Self {
        let root = Rc::new(RefCell::new(Page::new(PageType::Root, 0)));
        Self {
            name: name.into(),
            cols_by_name: HashMap::new(),
            cols: vec![],
            root: Rc::clone(&root),
            views: HashMap::new(),
            page_ids: ThreadSafeIdGenerator::new(1),
            row_ids: ThreadSafeIdGenerator::new(0),
            current_page: root,
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
        self.current_page.borrow_mut().insert(record);
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

    pub fn get_column_indexes(&self, expression: &str) -> Vec<usize> {
        expression
            .split(",")
            .map(|c| self.get_index(c.trim()))
            .collect::<Vec<usize>>()
    }

    pub fn get_index(&self, col_name: &str) -> usize {
        *self.cols_by_name.get(col_name).unwrap()
    }

    pub fn iter(&self) -> TableIter {
        TableIter {
            rootPage: Rc::clone(&self.root),
            index: 0,
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

    // pub fn where_clause(&self, colindex: usize, value: &Value) -> Option<&Record> {
    //     for record in self.iter() {
    //         let r = record.get(colindex);
    //         if r == value {
    //             return Some(record);
    //         }
    //     }
    //     None
    // }
}

pub struct TableIter {
    rootPage: Rc<RefCell<Page>>,
    index: usize,
}

impl Iterator for TableIter {
    type Item = Record;

    fn next(&mut self) -> Option<Self::Item> {
        self.rootPage.borrow().get(self.index)
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

#[derive(Debug)]
pub struct Key {
    values: Vec<Value>,
}

impl Key {
    pub fn integer(integer: usize) -> Self {
        Self {
            values: vec![integer.into()],
        }
    }

    pub fn compound(keys: Vec<Value>) -> Self {
        Self { values: keys }
    }
}
impl Ord for Key {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
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
                .partial_cmp(other.values.get(i).unwrap());

            match ord {
                Some(Ordering::Less) => {
                    return Some(Ordering::Less);
                }
                Some(Ordering::Greater) => {
                    return Some(Ordering::Greater);
                }
                _ => {}
            }
        }
        None
    }
}
