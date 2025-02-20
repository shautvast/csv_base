use std::{cmp::Ordering, fmt::Display};

use crate::varint;
use anyhow::anyhow;
use byteorder::{BigEndian, ByteOrder};

#[derive(Debug, Clone, PartialEq, Eq, Ord)]
pub struct Value {
    pub(crate) datatype: u64,
    pub(crate) datatype_bytes: Vec<u8>,
    pub(crate) data: Vec<u8>,
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.datatype {
            13.. if self.datatype % 2 == 1 => Some(self.to_string().cmp(&other.to_string())),
            12.. if self.datatype % 2 == 0 => None, // can't use blob as key
            8..=9 => integer_cmp(self, other),
            7 => {
                let l: anyhow::Result<f64> = self.into();
                let r: anyhow::Result<f64> = other.into();
                if let Ok(l) = l {
                    if let Ok(r) = r {
                        l.partial_cmp(&r)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            1..=6 => integer_cmp(self, other),
            0 => None,
            _ => None,
        }
    }
}

fn integer_cmp(l: &Value, r: &Value) -> Option<Ordering> {
    let l: anyhow::Result<i64> = l.into();
    let r: anyhow::Result<i64> = r.into();
    if let Ok(l) = l {
        if let Ok(r) = r {
            l.partial_cmp(&r)
        } else {
            None
        }
    } else {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Datatype {
    Text,
    Blob,
    Integer,
    Float,
    Null,
}

impl Value {
    fn new(datatype: u64, data: Vec<u8>) -> Self {
        Self {
            datatype,
            data,
            datatype_bytes: varint::write(datatype),
        }
    }

    /// get the length of the encoding of the value
    pub fn bytes_len(&self) -> u16 {
        (self.datatype_bytes.len() + self.data.len()) as u16
    }

    // can this be a constant?
    pub fn null() -> Self {
        Self::new(0, vec![])
    }

    pub fn from_f64(value: f64) -> Self {
        let mut buf = vec![0; 8];
        BigEndian::write_f64(&mut buf, value);
        Self::new(7, buf)
    }

    pub fn from_i64(value: i64) -> Self {
        let (datatype, data) = match value {
            0 => (8, vec![]),
            1 => (9, vec![]),
            _ => {
                let data = as_bytes(value);
                (int_datatype(data.len()), data)
            }
        };
        Self::new(datatype, data)
    }

    pub fn from_text(value: impl Into<String>) -> Self {
        let value: String = value.into();
        let datatype = (13 + value.len() * 2) as u64;
        let data = value.as_bytes().to_vec();
        Self::new(datatype, data)
    }

    pub fn datatype(&self) -> anyhow::Result<Datatype> {
        match self.datatype {
            13.. if self.datatype % 2 == 1 => Ok(Datatype::Text),
            12.. if self.datatype % 2 == 0 => Ok(Datatype::Blob),
            8..=9 => Ok(Datatype::Integer),
            7 => Ok(Datatype::Float),
            1..=6 => Ok(Datatype::Integer),
            0 => Ok(Datatype::Null),
            _ => Err(anyhow!("Illegal type '{}'", self.datatype)),
        }
    }

    pub fn string_len(&self) -> usize {
        match self.datatype {
            13.. if self.datatype % 2 == 1 => ((self.datatype - 13) >> 1) as usize,
            12.. if self.datatype % 2 == 0 => ((self.datatype - 12) >> 1) as usize,
            8..=9 => 1,
            7 => {
                let f = BigEndian::read_f64(&self.data);
                format!("{}", f).len()
            }
            1..=6 => {
                let f = BigEndian::read_i64(&self.data);
                format!("{}", f).len()
            }
            0 => 4, // NULL
            _ => 0, // should be Err
        }
    }
}

fn int_datatype(encoded_len: usize) -> u64 {
    match encoded_len {
        ..5 => encoded_len as u64,
        ..7 => 5,
        _ => 6,
    }
}

fn as_bytes(v: i64) -> Vec<u8> {
    encode(v, encoding_len(v))
}

fn encode(v: i64, len: usize) -> Vec<u8> {
    let mut buf = Vec::with_capacity(len);
    for i in 0..len {
        buf.push((v >> ((len - i - 1) * 8)) as u8);
    }
    buf
}

fn encoding_len(v: i64) -> usize {
    let u = if v < 0 { !v } else { v };
    match u {
        ..128 => 1,
        ..32768 => 2,
        ..8388607 => 3,
        ..2147483648 => 4,
        ..140737488355327 => 6,
        _ => 8,
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self.into();
        write!(f, "{}", s)
    }
}

impl Into<Value> for &str {
    fn into(self) -> Value {
        if let Ok(f) = self.parse::<f64>() {
            Value::from_f64(f)
        } else if let Ok(i) = self.parse::<i64>() {
            Value::from_i64(i)
        } else {
            Value::from_text(strip_quotes(self))
        }
    }
}

impl Into<Value> for String {
    fn into(self) -> Value {
        if let Ok(f) = self.parse::<f64>() {
            Value::from_f64(f)
        } else if let Ok(i) = self.parse::<i64>() {
            Value::from_i64(i)
        } else {
            Value::from_text(strip_quotes(self))
        }
    }
}

impl Into<Value> for f64 {
    fn into(self) -> Value {
        Value::from_f64(self)
    }
}

impl Into<Value> for i64 {
    fn into(self) -> Value {
        Value::from_i64(self)
    }
}

impl Into<Value> for usize {
    fn into(self) -> Value {
        Value::from_i64(self as i64)
    }
}

impl Into<Value> for i32 {
    fn into(self) -> Value {
        Value::from_i64(self as i64)
    }
}

impl Into<String> for Value {
    fn into(self) -> String {
        (&self).into()
    }
}

impl Into<String> for &Value {
    fn into(self) -> String {
        match self.datatype {
            13.. if self.datatype % 2 == 1 => String::from_utf8_lossy(&self.data).into_owned(), // valid?
            12.. if self.datatype % 2 == 0 => String::from_utf8_lossy(&self.data).into_owned(),
            8 => "0".to_string(),
            9 => "1".to_string(),
            7 => {
                let f: anyhow::Result<f64> = self.into();
                format!("{}", f.unwrap())
            }
            1..=6 => {
                let i: anyhow::Result<i64> = self.into();
                format!("{}", i.unwrap())
            }
            0 => "NULL".to_string(),                          // NULL
            _ => format!("Illegal type '{}'", self.datatype), // should be Err
        }
    }
}

impl Into<anyhow::Result<f64>> for Value {
    fn into(self) -> anyhow::Result<f64> {
        (&self).into()
    }
}

impl Into<anyhow::Result<f64>> for &Value {
    fn into(self) -> anyhow::Result<f64> {
        if self.datatype == 7 {
            Ok(BigEndian::read_f64(&self.data))
        } else {
            Err(anyhow!("not a float"))
        }
    }
}

impl Into<anyhow::Result<i64>> for Value {
    fn into(self) -> anyhow::Result<i64> {
        (&self).into()
    }
}

impl Into<anyhow::Result<i64>> for &Value {
    fn into(self) -> anyhow::Result<i64> {
        match self.datatype {
            0 => Err(anyhow!("value is NULL")),
            1 => Ok(BigEndian::read_int(&self.data, 1) as i64),
            2 => Ok(BigEndian::read_int(&self.data, 2) as i64),
            3 => Ok(BigEndian::read_int(&self.data, 3) as i64),
            4 => Ok(BigEndian::read_int(&self.data, 4) as i64),
            5 => Ok(BigEndian::read_int(&self.data, 6) as i64),
            6 => Ok(BigEndian::read_int(&self.data, 8) as i64),
            8 => Ok(0),
            9 => Ok(1),
            _ => Err(anyhow!("not an integer")),
        }
    }
}

fn strip_quotes(text: impl Into<String>) -> String {
    let mut text = text.into();
    if text.starts_with("\"") && text.ends_with("\"") {
        text = text[1..text.len() - 1].to_string();
    }
    text
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_int0() {
        let i: Value = 0.into();
        assert_eq!(i.datatype, 8);
        assert_eq!(i.data, vec![]);
        assert_eq!(i.to_string(), "0");
        assert_eq!(i.string_len(), 1);
        assert_eq!(i.datatype().unwrap(), Datatype::Integer);
    }

    #[test]
    fn test_int1() {
        let i: Value = 1.into();
        assert_eq!(i.datatype, 9);
        assert_eq!(i.data, vec![]);
        assert_eq!(i.to_string(), "1");
        assert_eq!(i.string_len(), 1);
        assert_eq!(i.datatype().unwrap(), Datatype::Integer);
    }

    #[test]
    fn test_int50000() {
        let i: Value = 50000.into();
        assert_eq!(i.datatype, 3);
        assert_eq!(i.data, vec![0, 195, 80]);
        assert_eq!(i.to_string(), "50000");
        // assert_eq!(i.string_len(), 5);
        assert_eq!(i.datatype().unwrap(), Datatype::Integer);
    }

    #[test]
    fn test_float0() {
        let i: Value = 0.0.into();
        assert_eq!(i.datatype, 7);
        assert_eq!(i.data, vec![0; 8]);
        assert_eq!(i.to_string(), "0");
        assert_eq!(i.string_len(), 1);
        assert_eq!(i.datatype().unwrap(), Datatype::Float);
    }

    #[test]
    fn test_float1() {
        let i: Value = 1.0.into();
        assert_eq!(i.datatype, 7);
        assert_eq!(i.data, vec![63, 240, 0, 0, 0, 0, 0, 0]);
        assert_eq!(i.to_string(), "1");
        assert_eq!(i.string_len(), 1);
        assert_eq!(i.datatype().unwrap(), Datatype::Float);
    }

    #[test]
    fn test_float50000() {
        let i: Value = 50000.2.into();
        assert_eq!(i.datatype, 7);
        assert_eq!(i.data, vec![64, 232, 106, 6, 102, 102, 102, 102]);
        assert_eq!(i.to_string(), "50000.2");
        assert_eq!(i.string_len(), 7);
        assert_eq!(i.datatype().unwrap(), Datatype::Float);
    }

    #[test]
    fn test_string() {
        let i: Value = "hello world".into();
        assert_eq!(i.datatype, ("hello world".len() * 2 + 13) as u64);
        assert_eq!(
            i.data,
            vec![104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100]
        );
        assert_eq!(i.to_string(), "hello world");
        assert_eq!(i.string_len(), 11);
        assert_eq!(i.datatype().unwrap(), Datatype::Text);
    }
}
