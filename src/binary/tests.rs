use crate::analysis::transient::TransientData;

use super::*;

static TRAN_EXAMPLE_PSFBIN: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/examples/tranbin.tran.tran"
));

#[test]
fn test_header() {
    let toc = parse_toc(TRAN_EXAMPLE_PSFBIN);
    let header = parse_header(TRAN_EXAMPLE_PSFBIN, &toc.data[&SectionKind::Header]);
    println!("Header: {:?}", header);
}

#[test]
fn test_types() {
    let toc = parse_toc(TRAN_EXAMPLE_PSFBIN);
    let types = parse_types(TRAN_EXAMPLE_PSFBIN, &toc.data[&SectionKind::Type]);
    println!("Types: {:?}", types);
}

#[test]
fn test_traces() {
    let toc = parse_toc(TRAN_EXAMPLE_PSFBIN);
    println!("ToC: {:?}", toc);
    let traces = parse_traces(TRAN_EXAMPLE_PSFBIN, &toc.data[&SectionKind::Trace]);
    let expected = vec![Trace::Group(TraceGroup {
        name: "group",
        count: 3,
        id: GroupId(24),
        signals: vec![
            SignalRef {
                id: TraceId(25),
                name: "out",
                type_id: TypeId(11),
                properties: Properties { values: vec![] },
            },
            SignalRef {
                id: TraceId(26),
                name: "vdd",
                type_id: TypeId(11),
                properties: Properties { values: vec![] },
            },
            SignalRef {
                id: TraceId(27),
                name: "Vvdd:p",
                type_id: TypeId(12),
                properties: Properties { values: vec![] },
            },
        ],
    })];
    println!("Traces: {:?}", traces);
    assert_eq!(traces, expected);
}

#[test]
fn test_values() {
    let toc = parse_toc(TRAN_EXAMPLE_PSFBIN);
    println!("ToC: {:?}", toc);
    let mut parser = PsfParser::new(TRAN_EXAMPLE_PSFBIN);
    parser.parse();
}

#[test]
fn test_sweeps() {
    let toc = parse_toc(TRAN_EXAMPLE_PSFBIN);
    println!("ToC: {:?}", toc);
    let sweeps = parse_sweeps(TRAN_EXAMPLE_PSFBIN, &toc.data[&SectionKind::Sweep]);
    println!("Sweeps: {:?}", sweeps);
}

#[test]
fn test_to_transient() {
    use crate::binary::parse;
    let ast = parse(TRAN_EXAMPLE_PSFBIN).unwrap();
    println!("ast = {ast:#?}");
    let data = TransientData::from_binary(ast);
    assert_eq!(data.signals.len(), 4);
    assert_eq!(
        data.signal("time")
            .expect("should contain a time signal")
            .len(),
        11
    );
}
