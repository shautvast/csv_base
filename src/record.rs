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
    /// returns the length of the string representation,
    /// for display purposes
    pub fn string_len(&self) -> usize {
        self.values.iter().map(Value::string_len).sum()
    }

    /// returns the length of the internal byte representation
    pub fn bytes_len(&self) -> u16 {
        let record_length: u16 = self.values.iter().map(Value::bytes_len).sum();
        record_length + 1
    }

    /// pushes a value to the record
    pub fn add_value(&mut self, value: impl Into<Value>) {
        self.values.push(value.into());
    }

    /// gets the value at the column index of the record
    pub fn get(&self, index: usize) -> &Value {
        self.values.get(index).unwrap() //TODO
    }
}

impl Add for &Record {
    type Output = Record;

    /// returns a new records that is the 'join' of the two inputs
    fn add(self, rhs: Self) -> Self::Output {
        let mut sum = Record::default();
        sum.values.append(&mut self.values.clone());
        sum.values.append(&mut rhs.values.clone()); // use refs?
        sum
    }
}

impl From<Record> for Vec<u8> {
    /// returns the byte reprsentation of the record
    /// which will be stored physically in the page (and some day on disk)
    fn from(mut record: Record) -> Vec<u8> {
        let record_length = record.bytes_len(); // len of all the values
        let mut length_bytes = varint::write(u64::from(record_length)); // the length of the above in bytes representation
        let mut rowid_bytes = varint::write(record.rowid); // the bytes representation of the rowid

        let mut buffer =
            Vec::with_capacity(length_bytes.len() + rowid_bytes.len() + record_length as usize);
        buffer.append(&mut length_bytes);
        buffer.append(&mut rowid_bytes);

        // sqlite docs: 'The initial portion of the payload that does not spill to overflow pages.'
        // the length of the byte representation of all value types in the record
        // -> after the record header, first all types (text, int, float etc) for the record are written
        // after that come the values themselves
        // so decoders first read this value to know how many types there are (how many bytes to read to decode the type bytes)
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

/// returns the Record from the byte representation
/// tuple (len, byte buffer)
/// len is the length that was read from the bytes before calling this
// needs improving, for clarity get rid of the tuple
impl Into<Record> for (u64, &[u8]) {
    fn into(self) -> Record {
        let (_, data) = self;
        let mut offset = 0;
        let (inc, rowid) = varint::read(data);
        offset += inc;
        let mut datatypes = vec![];
        let (inc, dt_len) = varint::read(&data[offset..]);
        offset += inc;
        let end_of_dt = offset + dt_len as usize - 1; // why -1?
                                                      //read n of fields

        while offset < end_of_dt {
            //WRONG, read this len first from the buffer
            let (inc, datatype) = varint::read(&data[offset..]);
            datatypes.push(datatype);
            offset += inc;
        }

        // decode the values
        let mut values: Vec<Value> = vec![];
        for dt in datatypes {
            match dt {
                13.. if dt % 2 == 1 => {
                    let len = ((dt - 13) >> 1) as usize;
                    values.push(Value::new(dt, data[offset..offset + len].to_vec()));
                    offset += len;
                }
                12.. if dt % 2 == 0 => {
                    let len = ((dt >> 1) - 12) as usize;
                    values.push(Value::new(dt, data[offset..offset + len].to_vec()));
                    offset += len;
                }
                8 | 9 => values.push(Value::new(dt, vec![])),
                7 => {
                    values.push(Value::new(dt, data[offset..offset + 8].to_vec()));
                    offset += 8;
                }
                1..=6 => {
                    let inc = read_int_len(dt);
                    values.push(Value::new(dt, data[offset..offset + inc].to_vec()));
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

fn read_int_len(datatype: u64) -> usize {
    match datatype {
        6 => 8,
        5 => 6,
        _ => datatype as usize,
    }
}

impl Default for Record {
    fn default() -> Self {
        Self {
            rowid: 0,
            values: vec![],
        }
    }
}
