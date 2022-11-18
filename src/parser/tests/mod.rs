use super::*;

#[test]
fn basic() {
    let input = r#"
    HEADER
    "PSFversion" "1.00"
    "integer value" 4
    "start" 0.0000
    "stop" 8.000e-08
    END
    "#;

    let psf = parse(input).unwrap();
    assert_eq!(psf, Psf {
        header: Header { values: vec![
            NamedValue {
                name: "PSFversion".to_string(),
                value: Value::String("1.00".to_string()),
            },
            NamedValue {
                name: "integer value".to_string(),
                value: Value::Int(4),
            },
            NamedValue {
                name: "start".to_string(),
                value: Value::Real(0f64),
            },
            NamedValue {
                name: "stop".to_string(),
                value: Value::Real(8.0e-8),
            },
        ] },
        types: Types {  },
        sweeps: Vec::new(),
    })
}
