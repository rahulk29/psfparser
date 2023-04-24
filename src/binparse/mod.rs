use std::collections::HashMap;

use self::ast::*;

pub mod ast;

#[cfg(test)]
mod tests;

#[repr(u32)]
enum BlockType {
    Null = 3,
    Sweeps = 21,
    SignalRef = 16,
}

fn parse_toc<'a>(data: &'a [u8]) -> Toc {
    let ds = peek_u32(&data[data.len() - 4..]) as usize;
    let n = (data.len() - ds - 12) / 8;

    let toc_ofs = data.len() - 12 - 8 * n;
    let mut toc = Toc::with_capacity(n);

    let mut pkind = None;
    for i in 0..n {
        let kind = peek_u32(&data[toc_ofs + 8 * i..]);
        let kind = SectionKind::from_int(kind);
        let ofs = peek_u32(&data[toc_ofs + 8 * i + 4..]) as usize;

        let entry = TocEntry {
            end: data.len(),
            start: ofs,
        };
        toc.data.insert(kind, entry);

        if let Some(pkind) = pkind {
            let mut ps = toc.data.get_mut(&pkind).unwrap();
            ps.end = ofs;
        }

        pkind = Some(kind);
    }

    toc
}

fn parse_sweep_values<'a, 'b>(file: &'a [u8], entry: &'b TocEntry) -> Vec<SignalRef<'a>> {
    todo!()
}

fn parse_sweeps<'a, 'b>(file: &'a [u8], entry: &'b TocEntry) -> Vec<SignalRef<'a>> {
    let (_, eofs) = parse_int(&file[entry.start + 4..]);

    let mut data = &file[entry.start + 8..eofs as usize];
    let mut values = Vec::new();

    while data.len() > 4 {
        let (d, id) = parse_int(data);
        assert_eq!(id, BlockType::SignalRef as u32);
        let r = parse_signal_ref(d);
        data = r.0;
        values.push(r.1);
    }

    values
}

fn parse_types<'a, 'b>(file: &'a [u8], entry: &'b TocEntry) -> Vec<TypeDef<'a>> {
    let data = &file[entry.start + 8..];
    let (data, block_t) = parse_int(data);
    assert_eq!(block_t, 22);
    let (_, eofs) = parse_int(data);
    let mut data = &file[entry.start + 8 + 8..eofs as usize];

    let mut values = Vec::new();

    while data.len() > 4 {
        let r = parse_type_item(data);
        data = r.0;
        values.push(r.1);
    }

    values
}

fn parse_type_item<'a>(data: &'a [u8]) -> (&'a [u8], TypeDef<'a>) {
    println!("parsing type item");

    let (data, block_t) = parse_int(data);
    assert_eq!(block_t, 16);

    let (data, id) = parse_int(data);
    let (data, name) = parse_string(data);
    let (data, array_t) = parse_int(data);
    let (data, data_type) = parse_int(data);
    let (data, properties) = parse_properties(data);

    (
        data,
        TypeDef {
            id,
            name,
            data_type,
            properties,
        },
    )
}

fn parse_traces<'a, 'b>(file: &'a [u8], entry: &'b TocEntry) -> Vec<Trace<'a>> {
    let data = &file[entry.start + 8..];
    let (data, block_t) = parse_int(data);
    assert_eq!(block_t, 22);
    let (_, eofs) = parse_int(data);
    let mut data = &file[entry.start + 8 + 8..eofs as usize];

    let mut values = Vec::new();

    while data.len() > 4 {
        let r = parse_trace_item(data);
        data = r.0;
        values.push(r.1);
    }

    values
}

fn parse_trace_item<'a>(data: &'a [u8]) -> (&'a [u8], Trace<'a>) {
    let (data, block_t) = parse_int(data);
    println!("parsing trace item");
    match block_t {
        16 => {
            // DataTypeDef
            let (data, signal) = parse_signal_ref(data);
            (data, Trace::Signal(signal))
        }
        17 => {
            // Group
            let (data, group) = parse_group(data);
            (data, Trace::Group(group))
        }
        _ => panic!("Unexpected block type: {block_t}"),
    }
}

// GroupDef
fn parse_group<'a>(data: &'a [u8]) -> (&'a [u8], TraceGroup<'a>) {
    println!("parsing group");
    let (data, id) = parse_int(data);
    let (data, name) = parse_string(data);
    let (mut data, count) = parse_int(data);

    println!("Found {count} signals in group");

    let mut signals = Vec::new();
    for i in 0..count {
        let r = parse_int(data);
        let block_t = r.1;
        assert_eq!(block_t, 16);
        let r = parse_signal_ref(r.0);
        data = r.0;
        signals.push(r.1);
    }

    (
        data,
        TraceGroup {
            name,
            count,
            id,
            signals,
        },
    )
}

// data type ref
fn parse_signal_ref<'a>(data: &'a [u8]) -> (&'a [u8], SignalRef<'a>) {
    println!("parsing signal ref");
    let (data, id) = parse_int(data);
    let (data, name) = parse_string(data);
    let (data, unit_id) = parse_int(data);
    let (data, properties) = parse_properties(data);

    (
        data,
        SignalRef {
            id,
            name,
            unit_id,
            properties,
        },
    )
}

fn parse_header<'a, 'b>(file: &'a [u8], entry: &'b TocEntry) -> Header<'a> {
    let (_, eofs) = parse_int(&file[entry.start + 4..]);

    let mut data = &file[entry.start + 8..eofs as usize];
    let mut values = Vec::new();

    while data.len() > 4 {
        let r = parse_named_value(data);
        data = r.0;
        values.push(r.1);
    }

    Header { values }
}

fn parse_end(data: &[u8]) -> (&[u8], ()) {
    (&data[4..], ())
}

fn parse_properties(data: &[u8]) -> (&[u8], Properties) {
    let mut data = data;

    let mut values = Vec::new();

    while {
        data.len() > 4 && {
            let (_, block_t) = parse_int(data);
            block_t >= 33 && block_t <= 35
        }
    } {
        let val;
        (data, val) = parse_named_value(data);
        values.push(val);
    }

    (data, Properties { values })
}

fn parse_named_value(mut data: &[u8]) -> (&[u8], NamedValue) {
    let (data, block_t) = parse_int(data);
    let (data, name) = parse_string(data);

    let (data, value) = match block_t {
        33 => {
            let (data, s) = parse_string(data);
            (data, Value::Str(s))
        }
        34 => {
            let (data, i) = parse_int(data);
            (data, Value::Int(i as i64))
        }
        35 => {
            let (data, i) = parse_float(data);
            (data, Value::Real(i))
        }
        _ => panic!("Unexpected block type: {block_t}"),
    };

    (data, NamedValue { name, value })
}

fn parse_string(mut data: &[u8]) -> (&[u8], &str) {
    let len = read_u32(&mut data) as usize;
    let s = std::str::from_utf8(&data[..len]).unwrap();
    if len % 4 == 0 {
        (&data[len..], s)
    } else {
        (&data[len + 4 - (len % 4)..], s)
    }
}

fn parse_int(mut data: &[u8]) -> (&[u8], u32) {
    let val = read_u32(&mut data);
    (data, val)
}

fn parse_float(mut data: &[u8]) -> (&[u8], f64) {
    let val = read_f64(&mut data);
    (data, val)
}

pub fn peek_u32(input: &[u8]) -> u32 {
    let (bytes, _) = input.split_at(std::mem::size_of::<u32>());
    u32::from_be_bytes(bytes.try_into().unwrap())
}

pub fn read_u32(input: &mut &[u8]) -> u32 {
    let (bytes, rest) = input.split_at(std::mem::size_of::<u32>());
    *input = rest;
    u32::from_be_bytes(bytes.try_into().unwrap())
}

pub fn read_f64(input: &mut &[u8]) -> f64 {
    let (bytes, rest) = input.split_at(std::mem::size_of::<f64>());
    *input = rest;
    f64::from_be_bytes(bytes.try_into().unwrap())
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
enum SectionKind {
    Header,
    Type,
    Sweep,
    Trace,
    Value,
}

impl SectionKind {
    pub fn from_int(value: u32) -> Self {
        use SectionKind::*;
        match value {
            0 => Header,
            1 => Type,
            2 => Sweep,
            3 => Trace,
            4 => Value,
            _ => panic!("Unexpected section number: {value}"),
        }
    }
}

#[derive(Debug, Clone)]
struct TocEntry {
    start: usize,
    /// Not inclusive.
    end: usize,
}

#[derive(Debug, Clone)]
struct Toc {
    data: HashMap<SectionKind, TocEntry>,
}

impl Toc {
    #[inline]
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: HashMap::with_capacity(capacity),
        }
    }
}
