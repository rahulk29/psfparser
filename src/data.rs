#[derive(Debug, Clone, PartialEq)]
pub struct Psf {
    pub header: Header,
    pub types: Types,
    pub sweeps: Vec<Sweep>,
}

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

#[derive(Debug, Clone, PartialEq)]
pub struct Types {
}

#[derive(Debug, Clone, PartialEq)]
pub struct Sweep {
    pub name: String,
    pub sweep_type: String,
    pub kinds: Vec<Kind>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    Float,
    Double,
    Complex,
    Int,
    Byte,
    Long,
    String,
    Array,
    Struct,
    Prop,
    Star,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Traces {

}

#[derive(Debug, Clone, PartialEq)]
pub struct Values {

}
