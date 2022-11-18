#[derive(Debug, Clone, PartialEq)]
pub struct Psf<'a> {
    pub header: Header<'a>,
    pub types: Types,
    pub sweeps: Vec<Sweep<'a>>,
    pub traces: Vec<Trace<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Header<'a> {
    pub values: Vec<NamedValue<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NamedValue<'a> {
    pub name: &'a str,
    pub value: Value<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value<'a> {
    Int(i64),
    Real(f64),
    Str(&'a str),
    NaN,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Types {}

#[derive(Debug, Clone, PartialEq)]
pub struct Sweep<'a> {
    pub name: &'a str,
    pub sweep_type: &'a str,
    pub kinds: Vec<Kind<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Kind<'a> {
    Float,
    Double,
    Complex,
    Int,
    Byte,
    Long,
    String,
    Array,
    Struct,
    Prop(Prop<'a>),
    Star,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Trace<'a> {
    Group { name: &'a str, count: i64 },
    Signal { name: &'a str, units: &'a str },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Values {}

#[derive(Debug, Clone, PartialEq)]
pub struct Prop<'a> {
    pub values: Vec<NamedValue<'a>>,
}
