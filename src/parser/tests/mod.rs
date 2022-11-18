use super::*;

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
        Psf {
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
            types: Types {},
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
        }
    )
}

static TRAN_EXAMPLE_PSF: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/examples/timeSweep.tran.tran"
));

#[test]
fn parses_transient() {
    let _psf = parse(TRAN_EXAMPLE_PSF).expect("Failed to parse transient PSF file");
}
