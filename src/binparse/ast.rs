use std::collections::HashMap;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct PsfAst<'a> {
    pub header: Header<'a>,
    pub types: Types<'a>,
    pub sweeps: Vec<SignalRef<'a>>,
    pub traces: Vec<Trace<'a>>,
    pub values: SignalValues,
}

impl<'a> PsfAst<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Header<'a> {
    pub values: HashMap<&'a str, Value<'a>>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Types<'a> {
    pub types: HashMap<TypeId, TypeDef<'a>>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TypeId(pub u32);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TraceId(pub u32);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GroupId(pub u32);

impl From<GroupId> for TraceId {
    fn from(value: GroupId) -> Self {
        Self(value.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeDef<'a> {
    pub id: TypeId,
    pub name: &'a str,
    pub data_type: DataType,
    pub properties: Properties<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NamedValue<'a> {
    pub name: &'a str,
    pub value: Value<'a>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u32)]
pub enum DataType {
    Int8 = 1,
    String = 2,
    Int32 = 5,
    Real = 11,
    Complex = 12,
    Struct = 16,
}

impl DataType {
    pub fn from_u32(value: u32) -> Self {
        match value {
            1 => Self::Int8,
            2 => Self::String,
            5 => Self::Int32,
            11 => Self::Real,
            12 => Self::Complex,
            16 => Self::Struct,
            _ => panic!("Unexpected data type: {value}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value<'a> {
    Int(i64),
    Real(f64),
    Str(&'a str),
    NaN,
}

impl<'a> Value<'a> {
    pub fn int(&self) -> i64 {
        use Value::*;
        match self {
            Int(v) => *v,
            _ => panic!("Failed to unwrap value as integer"),
        }
    }
    pub fn real(&self) -> f64 {
        use Value::*;
        match self {
            Real(v) => *v,
            _ => panic!("Failed to unwrap value as real"),
        }
    }
    pub fn str(&self) -> &'a str {
        use Value::*;
        match self {
            Str(v) => *v,
            _ => panic!("Failed to unwrap value as str"),
        }
    }
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

impl<'a> Trace<'a> {
    pub fn group(&self) -> &TraceGroup {
        match self {
            Self::Group(g) => g,
            _ => panic!("Cannot unwrap signal as group"),
        }
    }

    pub fn signal(&self) -> &SignalRef {
        match self {
            Self::Signal(s) => s,
            _ => panic!("Cannot unwrap group trace as signal"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TraceGroup<'a> {
    pub name: &'a str,
    pub count: u32,
    pub id: GroupId,
    pub signals: Vec<SignalRef<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Properties<'a> {
    pub values: Vec<NamedValue<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SignalRef<'a> {
    pub id: TraceId,
    pub name: &'a str,
    pub type_id: TypeId,
    pub properties: Properties<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Prop<'a> {
    pub values: Vec<NamedValue<'a>>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SignalValues {
    pub values: HashMap<TraceId, Values>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Values {
    Complex(Vec<(f64, f64)>),
    Real(Vec<f64>),
}

impl Values {
    pub fn real(&self) -> &Vec<f64> {
        match self {
            Self::Real(ref v) => v,
            _ => panic!("not a real value vector"),
        }
    }

    pub fn complex(&self) -> &Vec<(f64, f64)> {
        match self {
            Self::Complex(ref v) => v,
            _ => panic!("not a complex value vector"),
        }
    }

    pub fn real_mut(&mut self) -> &mut Vec<f64> {
        match self {
            Self::Real(ref mut v) => v,
            _ => panic!("not a real value vector"),
        }
    }

    pub fn complex_mut(&mut self) -> &mut Vec<(f64, f64)> {
        match self {
            Self::Complex(ref mut v) => v,
            _ => panic!("not a complex value vector"),
        }
    }
}
