use nom::sequence::{preceded, terminated};
use nom::{
  IResult,
  error::ParseError,
  sequence::delimited,
  character::complete::multispace0,
};

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and 
/// trailing whitespace, returning the output of `inner`.
pub fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
  where
  F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
  delimited(
    multispace0,
    inner,
    multispace0
  )
}

/// A combinator that takes a parser `inner` and produces a parser that consumes leading
/// whitespace, returning the output of `inner`.
pub fn ws_before<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
  where
  F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
  preceded(
    multispace0,
    inner,
  )
}

/// A combinator that takes a parser `inner` and produces a parser that consumes trailing
/// whitespace, returning the output of `inner`.
pub fn ws_after<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
  where
  F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
  terminated(
    inner,
    multispace0,
  )
}
