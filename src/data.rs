#[derive(Debug, Clone, PartialEq)]
pub struct Header {
    pub values: Vec<NamedValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NamedValue {
    pub name: String,
    pub value: Value,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Real(f64),
    String(String),
    NaN,
}
