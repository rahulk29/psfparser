use crate::parser::ac::AcData;
use crate::parser::ast::*;
use crate::parser::frontend::parse;

use super::transient::TransientData;

#[test]
fn basic() {
    let input = r#"
    HEADER
    "PSFversion" "1.00"
    "integer value" 4
    "start" 0.0000
    "stop" 8.000e-08
    TYPE
    SWEEP
    TRACE
    " 1" GROUP 1
    "v(dout[0])" "V"
    VALUE
    END
    "#;

    let psf = parse(input).unwrap();
    assert_eq!(
        psf,
        PsfAst {
            header: Header {
                values: vec![
                    NamedValue {
                        name: "PSFversion",
                        value: Value::Str("1.00"),
                    },
                    NamedValue {
                        name: "integer value",
                        value: Value::Int(4),
                    },
                    NamedValue {
                        name: "start",
                        value: Value::Real(0f64),
                    },
                    NamedValue {
                        name: "stop",
                        value: Value::Real(8.0e-8),
                    },
                ]
            },
            types: Vec::new(),
            sweeps: Vec::new(),
            traces: vec![
                Trace::Group {
                    name: " 1",
                    count: 1
                },
                Trace::Signal {
                    name: "v(dout[0])",
                    units: "V"
                }
            ],
            values: Vec::new(),
        }
    )
}

static TRAN_EXAMPLE1_PSF: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/examples/timeSweep1.tran.tran"
));

static TRAN_EXAMPLE2_PSF: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/examples/timeSweep2.tran.tran"
));

static AC_EXAMPLE_PSF: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/examples/frequencySweep.ac"
));

#[test]
fn parses_transient_1() {
    let ast = parse(TRAN_EXAMPLE1_PSF).expect("Failed to parse transient PSF file");
    let data = TransientData::from_ast(&ast);
    assert_eq!(data.signals.len(), 17);
}

#[test]
fn parses_transient_2() {
    let ast = parse(TRAN_EXAMPLE2_PSF).expect("Failed to parse transient PSF file");
    let data = TransientData::from_ast(&ast);
    assert_eq!(data.signals.len(), 41);
}

#[test]
fn parses_ac() {
    let ast = parse(AC_EXAMPLE_PSF).expect("Failed to parse ac PSF file");
    let data = AcData::from_ast(&ast);
    assert_eq!(data.signals.len(), 3);
    assert_eq!(data.freq.len(), 13);
}
