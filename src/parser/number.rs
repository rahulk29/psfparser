use nom::{
  IResult,
  branch::alt,
  multi::{many0, many1},
  combinator::{opt, recognize},
  sequence::{preceded, terminated, tuple},
  character::complete::{char, one_of},
};

pub(crate) fn decimal(input: &str) -> IResult<&str, &str> {
  recognize(
    many1(
      one_of("0123456789")
    )
  )(input)
}
