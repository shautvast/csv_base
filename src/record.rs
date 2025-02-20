use crate::value::Value;
use crate::varint;
use byteorder::{BigEndian, ByteOrder};
use std::ops::Add;

#[derive(Debug, Clone)]
pub struct Record {
    pub rowid: u64,
    pub(crate) values: Vec<Value>,
}

impl Record {
    pub fn string_len(&self) -> usize {
        self.values.iter().map(Value::string_len).sum()
    }

    pub fn bytes_len(&self) -> u16 {
        let record_length: u16 = self.values.iter().map(Value::bytes_len).sum();
        record_length + 1
    }

    pub fn add_value(&mut self, value: impl Into<Value>) {
        self.values.push(value.into());
    }

    pub fn get(&self, index: usize) -> &Value {
        self.values.get(index).unwrap() //TODO
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

impl From<Record> for Vec<u8> {
    fn from(mut record: Record) -> Vec<u8> {
        let record_length = record.bytes_len();
        let mut length_bytes = varint::write(u64::from(record_length));
        let mut rowid_bytes = varint::write(record.rowid);

        let mut buffer =
            Vec::with_capacity(length_bytes.len() + rowid_bytes.len() + record_length as usize);
        buffer.append(&mut length_bytes);
        buffer.append(&mut rowid_bytes);

        // 'The initial portion of the payload that does not spill to overflow pages.'
        let length_of_encoded_column_types: usize =
            record.values.iter().map(|v| v.datatype_bytes.len()).sum();
        buffer.append(&mut varint::write(
            (length_of_encoded_column_types + 1) as u64,
        ));

        //write all types
        for v in &mut record.values {
            buffer.append(&mut v.datatype_bytes);
        }

        //  write all values
        for v in &mut record.values {
            buffer.append(&mut v.data);
        }
        buffer
    }
}

impl Into<Record> for (u64, &[u8]) {
    fn into(self) -> Record {
        let (len, data) = self;
        let len = len as usize; //meh
        let (mut offset, rowid) = varint::read(data);

        let mut datatypes = vec![];
        
        //read n of fields
        while (offset < len) {
            let (inc, datatype) = varint::read(&data[offset..]);
            datatypes.push(datatype);
            offset += inc;
        }

        let mut values: Vec<Value> = vec![];
        for dt in datatypes {
            match dt {
                13.. if dt % 2 == 0 => {
                    let len = ((dt >> 1) - 13) as usize;
                    if let Ok(text) = String::from_utf8(data[offset..len].to_vec()) {
                        values.push(text.into());
                    }
                    offset += len;
                }
                12.. if dt % 2 == 0 => {
                    let len = ((dt >> 1) - 12) as usize;
                    // no blobs yet
                    offset += len;
                }
                9 => values.push(1.into()),
                8 => values.push(0.into()),
                7 => {
                    values.push(BigEndian::read_f64(&data[offset..offset + 8]).into());
                    offset += 8;
                }
                1..=6 => {
                    let (inc, v) = read_int(&data[offset..], dt);
                    values.push(v.into());
                    offset += inc;
                }
                0 => {
                    values.push(Value::null());
                }
                _ => panic!("unknown datatype"),
            }
        }

        Record { rowid, values }
    }
}

fn read_int(buf: &[u8], datatype: u64) -> (usize, i64) {
    let nb = match datatype {
        6 => 8,
        5 => 6,
        _ => datatype as usize,
    };
    (nb, BigEndian::read_i64(&buf[..nb]))
}

impl Default for Record {
    fn default() -> Self {
        Self {
            rowid: 0,
            values: vec![],
        }
    }
}
