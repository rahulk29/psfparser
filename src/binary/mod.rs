use std::collections::HashMap;

use self::ast::*;

pub mod ast;

#[cfg(test)]
mod tests;

use crate::Result;

pub fn parse(input: &[u8]) -> Result<PsfAst> {
    let mut parser = PsfParser::new(input);
    parser.parse();
    Ok(parser.into_inner())
}

pub struct PsfParser<'a> {
    data: &'a [u8],
    toc: Option<Toc>,
    ast: PsfAst<'a>,

    // Trace ID to offset
    offsets: HashMap<TraceId, u32>,
}

impl<'a> PsfParser<'a> {
    pub fn new(file: &'a [u8]) -> Self {
        Self {
            data: file,
            toc: None,
            ast: PsfAst::default(),
            offsets: HashMap::new(),
        }
    }

    pub fn parse(&mut self) {
        self.parse_toc();
        self.parse_header();
        self.parse_types();
        self.parse_sweeps();
        self.parse_traces();
        self.parse_values();
    }

    #[inline]
    pub fn into_inner(self) -> PsfAst<'a> {
        self.ast
    }

    fn toc(&mut self) -> &Toc {
        match self.toc {
            Some(ref toc) => toc,
            None => {
                self.parse_toc();
                self.toc.as_ref().unwrap()
            }
        }
    }

    fn parse_toc(&mut self) {
        let toc = parse_toc(self.data);
        self.toc = Some(toc);
    }

    fn windowed(&self) -> bool {
        self.ast.header.values.contains_key("PSF window size")
    }

    fn window_size(&self) -> i64 {
        let v = self.ast.header.values.get("PSF window size").unwrap();
        v.int()
    }

    fn num_traces(&self) -> i64 {
        let v = self.ast.header.values.get("PSF traces").unwrap();
        v.int()
    }

    fn sweep_points(&self) -> i64 {
        let v = self.ast.header.values.get("PSF sweep points").unwrap();
        v.int()
    }

    fn parse_values(&mut self) {
        assert!(
            self.windowed(),
            "Binary PSF parser only supports windowed PSF files"
        );
        let entry = self.toc().section(SectionKind::Value);
        let (data, _eofs) = parse_int(&self.data[entry.start + 4..]);

        let window_size = self.window_size();
        let num_traces = self.num_traces();
        let sweep_points = self.sweep_points();

        let mut ofs = 0;
        for trace in self.ast.traces.iter() {
            for signal in trace.group().signals.iter() {
                self.offsets.insert(signal.id, ofs);
                ofs += window_size as u32;
            }
        }

        let (data, block_t) = parse_int(data);
        assert_eq!(block_t, 20);
        let data = parse_zero_pad(data);

        let mut data = data;
        let mut count = 0;
        while count < sweep_points {
            let block_init;
            (data, block_init) = parse_int(data);
            let window_count = block_init & 0xffff;

            let swp_sig = &self.ast.sweeps[0];
            let swp_name = swp_sig.name;
            println!("swp_name = {swp_name}");
            let swp_vec = self
                .ast
                .values
                .values
                .entry(swp_sig.id)
                .or_insert(Values::Real(vec![]));
            let swp_vec = swp_vec.real_mut();

            for _ in 0..window_count {
                let v;
                (data, v) = parse_float(data);
                swp_vec.push(v);
            }

            for group in self.ast.traces.iter() {
                for sig in group.group().signals.iter() {
                    let idx =
                        (self.offsets[&sig.id] + (window_size as u32 - window_count * 8)) as usize;
                    let data_type = self.ast.types.types[&sig.type_id].data_type;
                    let mut databuf = &data[idx..];

                    assert_ne!(swp_name, sig.name);

                    match data_type {
                        DataType::Real => {
                            let values = self
                                .ast
                                .values
                                .values
                                .entry(sig.id)
                                .or_insert(Values::Real(vec![]));
                            let values = values.real_mut();
                            for _ in 0..window_count {
                                let v = read_f64(&mut databuf);
                                values.push(v);
                            }
                        }
                        DataType::Complex => {
                            let values = self
                                .ast
                                .values
                                .values
                                .entry(sig.id)
                                .or_insert(Values::Complex(vec![]));
                            let values = values.complex_mut();
                            for _ in 0..window_count {
                                let real = read_f64(&mut databuf);
                                let imag = read_f64(&mut databuf);
                                values.push((real, imag));
                            }
                        }
                        _ => panic!("Unsupported data type: {data_type:?}"),
                    };
                }
            }

            data = &data[(num_traces * window_size) as usize..];
            count += window_count as i64;
        }
    }

    fn parse_types(&mut self) {
        self.ast.types = parse_types(self.data, &self.toc().section(SectionKind::Type));
    }

    fn parse_sweeps(&mut self) {
        self.ast.sweeps = parse_sweeps(self.data, &self.toc().section(SectionKind::Sweep));
    }

    fn parse_traces(&mut self) {
        self.ast.traces = parse_traces(self.data, &self.toc().section(SectionKind::Trace));
    }

    fn parse_header(&mut self) {
        self.ast.header = parse_header(self.data, &self.toc().section(SectionKind::Header));
    }
}

fn parse_toc(data: &[u8]) -> Toc {
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

fn parse_zero_pad(data: &[u8]) -> &[u8] {
    let (data, len) = parse_int(data);
    &data[4 + len as usize..]
}

fn parse_sweeps<'a>(file: &'a [u8], entry: &TocEntry) -> Vec<SignalRef<'a>> {
    let (_, eofs) = parse_int(&file[entry.start + 4..]);

    let mut data = &file[entry.start + 8..eofs as usize];
    let mut values = Vec::new();

    while data.len() > 4 {
        let (d, id) = parse_int(data);
        assert_eq!(id, 16);
        let r = parse_signal_ref(d);
        data = r.0;
        values.push(r.1);
    }

    values
}

fn parse_types<'a>(file: &'a [u8], entry: &TocEntry) -> Types<'a> {
    let data = &file[entry.start + 8..];
    let (data, block_t) = parse_int(data);
    assert_eq!(block_t, 22);
    let (_, eofs) = parse_int(data);
    let mut data = &file[entry.start + 8 + 8..eofs as usize];

    let mut types = HashMap::new();

    while data.len() > 4 {
        let r = parse_type_item(data);
        data = r.0;
        types.insert(r.1.id, r.1);
    }

    Types { types }
}

fn parse_type_item(data: &[u8]) -> (&[u8], TypeDef<'_>) {
    let (data, block_t) = parse_int(data);
    assert_eq!(block_t, 16);

    let (data, id) = parse_int(data);
    let (data, name) = parse_string(data);
    let (data, _array_t) = parse_int(data);
    let (data, data_type) = parse_int(data);
    let (data, properties) = parse_properties(data);

    (
        data,
        TypeDef {
            id: TypeId(id),
            name,
            data_type: DataType::from_u32(data_type),
            properties,
        },
    )
}

fn parse_traces<'a>(file: &'a [u8], entry: &TocEntry) -> Vec<Trace<'a>> {
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

fn parse_trace_item(data: &[u8]) -> (&[u8], Trace<'_>) {
    let (data, block_t) = parse_int(data);
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
fn parse_group(data: &[u8]) -> (&[u8], TraceGroup<'_>) {
    let (data, id) = parse_int(data);
    let (data, name) = parse_string(data);
    let (mut data, count) = parse_int(data);

    let mut signals = Vec::new();
    for _ in 0..count {
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
            id: GroupId(id),
            signals,
        },
    )
}

// data type ref
fn parse_signal_ref(data: &[u8]) -> (&[u8], SignalRef<'_>) {
    let (data, id) = parse_int(data);
    let (data, name) = parse_string(data);
    let (data, type_id) = parse_int(data);
    let (data, properties) = parse_properties(data);

    (
        data,
        SignalRef {
            id: TraceId(id),
            name,
            type_id: TypeId(type_id),
            properties,
        },
    )
}

fn parse_header<'a>(file: &'a [u8], entry: &TocEntry) -> Header<'a> {
    let (_, eofs) = parse_int(&file[entry.start + 4..]);

    let mut data = &file[entry.start + 8..eofs as usize];
    let mut values = HashMap::new();

    while data.len() > 4 {
        let r = parse_named_value(data);
        data = r.0;
        values.insert(r.1.name, r.1.value);
    }

    Header { values }
}

fn parse_properties(data: &[u8]) -> (&[u8], Properties) {
    let mut data = data;

    let mut values = Vec::new();

    while {
        data.len() > 4 && {
            let (_, block_t) = parse_int(data);
            (33..=35).contains(&block_t)
        }
    } {
        let val;
        (data, val) = parse_named_value(data);
        values.push(val);
    }

    (data, Properties { values })
}

fn parse_named_value(data: &[u8]) -> (&[u8], NamedValue) {
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
pub enum SectionKind {
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

#[derive(Debug, Clone, Copy)]
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
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: HashMap::with_capacity(capacity),
        }
    }

    pub fn section(&self, section: SectionKind) -> TocEntry {
        self.data[&section]
    }
}