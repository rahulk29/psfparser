use nom::IResult;
use nom::bytes::streaming::tag;
use nom::character::complete::space1;
use nom::combinator::map_res;
use nom::number::complete::float;
use nom::branch::alt;
use nom::sequence::tuple;

use crate::data::{Value, NamedValue};
use crate::parser::whitespace::ws;

use crate::parser::number::decimal;
use crate::parser::string::parse_string;

pub(crate) fn value(i: &str) -> IResult<&str, Value> {
    alt((int_value, real_value, string_value, nan_value))(i)
}


pub(crate) fn named_value(i: &str) -> IResult<&str, NamedValue> {
    let (i, (name, _, value)) = tuple((ws(parse_string), space1, ws(value)))(i)?;

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

