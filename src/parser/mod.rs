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
