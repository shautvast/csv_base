use std::fmt::Display;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Value {
    Text(String),
    Float(f64),
    Integer(i64),
    NULL,
}

impl Value {
    pub fn len(&self) -> usize {
        match self {
            Value::Text(text) => text.len(),
            Value::Float(float) => format!("{}", float).len(),
            Value::Integer(integer) => format!("{}", integer).len(),
            Value::NULL => 0,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Value::Float(float) => format!("{}", float),
            Value::Integer(integer) => format!("{}", integer),
            Value::Text(text) => format!("\"{}\"", text),
            Value::NULL => "NULL".to_string(),
        };
        write!(f, "{}", text)
    }
}

impl Into<Value> for &str {
    fn into(self) -> Value {
        if let Ok(f) = self.parse::<f64>() {
            Value::Float(f)
        } else if let Ok(i) = self.parse::<i64>() {
            Value::Integer(i)
        } else {
            Value::Text(strip_quotes(self))
        }
    }
}

impl Into<Value> for String {
    fn into(self) -> Value {
        if let Ok(f) = self.parse::<f64>() {
            Value::Float(f)
        } else if let Ok(i) = self.parse::<i64>() {
            Value::Integer(i)
        } else {
            Value::Text(strip_quotes(self))
        }
    }
}

impl Into<Value> for f64 {
    fn into(self) -> Value {
        Value::Float(self)
    }
}

impl Into<Value> for i64 {
    fn into(self) -> Value {
        Value::Integer(self)
    }
}

fn strip_quotes(text: impl Into<String>) -> String {
    let mut text = text.into();
    if text.starts_with("\"") && text.ends_with("\"") {
        text = text[1..text.len() - 1].to_string();
    }
    text
}
