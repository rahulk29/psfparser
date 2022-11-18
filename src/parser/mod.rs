use nom::IResult;
use nom::bytes::streaming::tag;
use nom::combinator::map_res;
use nom::multi::many0;
use nom::number::complete::float;
use nom::branch::alt;
use self::value::named_value;
use self::whitespace::ws;

use crate::data::Header;

pub mod number;
pub mod string;
pub mod value;
pub mod whitespace;


fn header(i: &str) -> IResult<&str, Header> {
    let (i, _) = ws(tag("HEADER"))(i)?;
    let (i, values) = many0(named_value)(i)?;

    Ok((i, Header { values }))
}

#[cfg(test)]
mod tests {
    use crate::data::{NamedValue, Value};

    use super::*;

    #[test]
    fn test_header() {
        let example = r#"
        HEADER
        "PSFversion" "1.00"
        "simulator" "spectre"
        "start" 0.000
        "stop" 8.000e-08
        "#;
        assert_eq!(header(example), Ok(("", Header {
            values: vec![
                NamedValue {
                    name: "PSFversion".to_string(),
                    value: Value::String("1.00".to_string()),
                },
                NamedValue {
                    name: "simulator".to_string(),
                    value: Value::String("spectre".to_string()),
                },
                NamedValue {
                    name: "start".to_string(),
                    value: Value::Real(0f64),
                },
                NamedValue {
                    name: "stop".to_string(),
                    value: Value::Real(8.0e-8f64),
                },
        ]
        })));
    }
}
