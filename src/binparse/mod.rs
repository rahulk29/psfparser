use std::collections::HashMap;

use crate::parser::ast::*;

#[cfg(test)]
mod tests;

fn parse_toc<'a>(data: &'a [u8]) -> Toc {
    let ds = peek_i32(&data[data.len() - 4..]) as usize;
    let n = (data.len() - ds - 12) / 8;

    let toc_ofs = data.len() - 12 - 8 * n;
    let mut toc = Toc::with_capacity(n);

    let mut pkind = None;
    for i in 0..n {
        let kind = peek_i32(&data[toc_ofs + 8 * i..]);
        let kind = SectionKind::from_int(kind);
        let ofs = peek_i32(&data[toc_ofs + 8 * i + 4..]) as usize;

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

fn parse_header<'a, 'b>(file: &'a [u8], entry: &'b TocEntry) -> Header<'a> {
    let (_, eofs) = parse_int(&file[entry.start + 4..]);

    let mut data = &file[entry.start + 8..eofs as usize];
    let mut values = Vec::new();

    while data.len() > 4 {
        let r = parse_named_value(data);
        data = r.0;
        values.push(r.1);
    }

    Header {
        values,
    }
}

fn parse_end(data: &[u8]) -> (&[u8], ()) {
    (&data[4..], ())
}

fn parse_named_value(mut data: &[u8]) -> (&[u8], NamedValue) {
    let (data, block_t) = parse_int(data);
    let (data, name) = parse_string(data);

    let (data, value) = match block_t {
        33 => {
            let (data, s) = parse_string(data);
            (data, Value::Str(s))
        },
        34 => {
            let (data, i) = parse_int(data);
            (data, Value::Int(i as i64))
        },
        35 => {
            let (data, i) = parse_float(data);
            (data, Value::Real(i))
        }
        _ => panic!("Unexpected block type: {block_t}"),
    };

    (data, NamedValue {
        name,
        value,
    })
}

fn parse_string(mut data: &[u8]) -> (&[u8], &str) {
    let len = read_i32(&mut data) as usize;
    let s = std::str::from_utf8(&data[..len]).unwrap();
    if len % 4 == 0 {
        (&data[len..], s)
    } else {
        (&data[len + 4 - (len % 4)..], s)
    }
}

fn parse_int(mut data: &[u8]) -> (&[u8], i32) {
    let val = read_i32(&mut data);
    (data, val)
}

fn parse_float(mut data: &[u8]) -> (&[u8], f64) {
    let val = read_f64(&mut data);
    (data, val)
}

pub fn peek_i32(input: &[u8]) -> i32 {
    let (bytes, _) = input.split_at(std::mem::size_of::<i32>());
    i32::from_be_bytes(bytes.try_into().unwrap())
}

pub fn read_i32(input: &mut &[u8]) -> i32 {
    let (bytes, rest) = input.split_at(std::mem::size_of::<i32>());
    *input = rest;
    i32::from_be_bytes(bytes.try_into().unwrap())
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
    pub fn from_int(value: i32) -> Self {
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

struct TocEntry {
    start: usize,
    /// Not inclusive.
    end: usize,

}

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
