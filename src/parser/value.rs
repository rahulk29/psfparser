use nom::IResult;
use nom::bytes::streaming::tag;
use nom::character::complete::space1;
use nom::combinator::map_res;
use nom::number::complete::float;
use nom::branch::alt;
use nom::sequence::tuple;

use crate::data::{Value, NamedValue};

use crate::parser::number::decimal;
use crate::parser::string::parse_string;

use super::whitespace::{ws_before, ws_after};

pub(crate) fn value(i: &str) -> IResult<&str, Value> {
    alt((nan_value, real_value, int_value, string_value))(i)
}


pub(crate) fn named_value(i: &str) -> IResult<&str, NamedValue> {
    let (i, (name, _, value)) = tuple((ws_before(parse_string), space1, ws_after(value)))(i)?;

    Ok((i, NamedValue { name, value }))
}

fn int_value(i: &str) -> IResult<&str, Value> {
    let (i, v) = map_res(decimal, |s: &str| s.parse::<i64>())(i)?;

    Ok((i, Value::Int(v)))
}

fn real_value(i: &str) -> IResult<&str, Value> {
    let (i, v) = float(i)?;

    Ok((i, Value::Real(v as f64)))
}

fn string_value(i: &str) -> IResult<&str, Value> {
    let (i, v) = parse_string(i)?;

    Ok((i, Value::String(v)))
}

fn nan_value(i: &str) -> IResult<&str, Value> {
    let (i, _) = alt((tag("nan"), tag("NaN")))(i)?;

    Ok((i, Value::NaN))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_named_value() {
        assert_eq!(named_value(r#""PSFversion" "1.00""#), Ok(("", NamedValue {
            name: "PSFversion".to_string(),
            value: Value::String("1.00".to_string()),
        })));

        assert_eq!(named_value(r#""start" 0.00"#), Ok(("", NamedValue {
            name: "start".to_string(),
            value: Value::Real(0f64),
        })));

        assert_eq!(named_value(r#""test" 2"#), Ok(("", NamedValue {
            name: "test".to_string(),
            value: Value::Int(2),
        })));

        assert_eq!(named_value(r#""nan value" nan"#), Ok(("", NamedValue {
            name: "nan value".to_string(),
            value: Value::NaN,
        })));
    }
}
