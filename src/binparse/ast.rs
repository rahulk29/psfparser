#[derive(Debug, Clone, PartialEq)]
pub struct PsfAst<'a> {
    pub header: Header<'a>,
    pub types: Vec<TypeDef<'a>>,
    pub sweeps: Vec<Sweep<'a>>,
    pub traces: Vec<Trace<'a>>,
    pub values: Vec<SignalValues<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Header<'a> {
    pub values: Vec<NamedValue<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeDef<'a> {
    pub id: u32,
    pub name: &'a str,
    pub data_type: u32,
    pub properties: Properties<'a>,
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
    Struct(Vec<TypeDef<'a>>),
    Prop(Prop<'a>),
    Star,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Trace<'a> {
    Group(TraceGroup<'a>),
    Signal(SignalRef<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct TraceGroup<'a> {
    pub name: &'a str,
    pub count: u32,
    pub id: u32,
    pub signals: Vec<SignalRef<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Properties<'a> {
    pub values: Vec<NamedValue<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SignalRef<'a> {
    pub id: u32,
    pub name: &'a str,
    pub unit_id: u32,
    pub properties: Properties<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Prop<'a> {
    pub values: Vec<NamedValue<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SignalValues<'a> {
    pub signal: &'a str,
    pub sigtype: Option<&'a str>,
    pub values: Values,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Values {
    Complex(Vec<(f64, f64)>),
    Real(Vec<f64>),
}
