pub struct Header {
    pub values: Vec<NamedValue>,
}

pub struct NamedValue {
    pub name: String,
    pub value: Value,
}

pub enum Value {
    Int(i64),
    Real(f64),
    String(String),
    NaN,
}
