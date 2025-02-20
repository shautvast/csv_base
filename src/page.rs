use crate::record::Record;
use crate::value::Value;
use crate::varint;
use byteorder::{BigEndian, ByteOrder};

const PAGE_SIZE: usize = 4096;

#[derive(Debug)]
pub enum PageType {
    Root,
    Interior,
    Leaf,
}

#[derive(Debug)]
pub struct Page {
    pagetype: PageType,
    id: usize,
    start: Value,
    end: Value,
    data: Vec<u8>,
    index_pos: u16,
    data_pos: u16,
    key: usize,
    children: Vec<Page>,
    n_records: usize,
}

impl Page {
    pub fn new(pagetype: PageType, id: usize) -> Self {
        Self {
            pagetype,
            id,
            start: Value::null(),
            end: Value::null(),
            data: vec![0; PAGE_SIZE],
            index_pos: 0,
            data_pos: (PAGE_SIZE - 1) as u16,
            key: 0,
            children: vec![],
            n_records: 0,
        }
    }

    pub fn insert(&mut self, record: Record) {
        let bytes: Vec<u8> = record.into();
        self.insert_data(bytes);
        self.insert_index(self.data_pos);
        self.n_records += 1;
    }

    fn insert_data(&mut self, bytes: Vec<u8>) {
        let end = self.data_pos as usize;
        self.data_pos -= bytes.len() as u16;
        self.data.splice(self.data_pos as usize..end, bytes);
    }

    fn insert_index(&mut self, value: u16) {
        let bytes = u16_to_bytes(value);
        let start = self.index_pos as usize;
        self.index_pos += bytes.len() as u16;
        self.data.splice(start..self.index_pos as usize, bytes);
    }

    pub fn get(&self, index: usize) -> Option<Record> {
        if index < self.n_records {
            let index = BigEndian::read_u16(&self.data[index * 2..=index * 2 + 1]);
            let (nbytes, len) = varint::read(&self.data[index as usize..]);
            Some(
                (
                    len,
                    &self.data[nbytes + index as usize..nbytes + index as usize + len as usize],
                )
                    .into(),
            )
        } else {
            None
        }
    }
}

fn u16_to_bytes(value: u16) -> Vec<u8> {
    let mut buf = vec![0; 2];
    BigEndian::write_u16(&mut buf, value);
    buf
}
