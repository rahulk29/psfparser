use pest::Parser;
use pest::iterators::Pair;

use crate::data::{Psf, Header, NamedValue, Value, Types, Sweep};
use crate::Result;

#[cfg(test)]
mod tests;

#[derive(Parser)]
#[grammar = "psf_ascii.pest"]
pub struct PsfAsciiParser;


pub fn parse(input: &str) -> Result<Psf> {
    let input = PsfAsciiParser::parse(Rule::psf_ascii, input)?.next().unwrap();
    parse_psf_inner(input)
}

fn parse_psf_inner(input: Pair<Rule>) -> Result<Psf> {
    assert_eq!(input.as_rule(), Rule::psf_ascii_inner);
    let mut pairs = input.into_inner();
    let header = parse_header(pairs.next().unwrap())?;

    let (types, sweeps) = if let Some(input) = pairs.next() {
        let mut input = input.into_inner();
        let types = parse_types(input.next().unwrap())?;
        let sweeps = parse_sweeps(input.next().unwrap())?;
        (types, sweeps)
    } else {
        (Types {}, Vec::new())
    };

    Ok(Psf {
        header,
        types,
        sweeps,
    })
}

fn parse_header(input: Pair<Rule>) -> Result<Header> {
    assert_eq!(input.as_rule(), Rule::header_section);
    let mut pairs = input.into_inner();
    let named_values = pairs.next().unwrap();
    let values = parse_named_values(named_values)?;
    Ok(Header {
        values,
    })
}

fn parse_named_values(input: Pair<Rule>) -> Result<Vec<NamedValue>> {
    assert_eq!(input.as_rule(), Rule::named_values);
    let pairs = input.into_inner();
    Ok(pairs.map(|p| parse_named_value(p)).collect::<Result<Vec<_>>>()?)
}

fn parse_named_value(input: Pair<Rule>) -> Result<NamedValue> {
    assert_eq!(input.as_rule(), Rule::named_value);
    let mut pairs = input.into_inner();

    let name = parse_string(pairs.next().unwrap())?;
    let value = parse_value(pairs.next().unwrap())?;

    Ok(NamedValue { name, value })
}

fn parse_value(input: Pair<Rule>) -> Result<Value> {
    Ok(match input.as_rule() {
        Rule::string => Value::String(parse_string(input)?),
        Rule::integer => Value::Int(parse_integer(input)?),
        Rule::real => Value::Real(parse_real(input)?),
        Rule::nan => Value::NaN,
        _ => unreachable!("unexpected value rule")
    })
}

fn parse_string(input: Pair<Rule>) -> Result<String> {
    assert_eq!(input.as_rule(), Rule::string);
    Ok(String::from(input.into_inner().next().unwrap().as_str()))
}

fn parse_integer(input: Pair<Rule>) -> Result<i64> {
    assert_eq!(input.as_rule(), Rule::integer);
    Ok(input.as_str().parse()?)
}

fn parse_real(input: Pair<Rule>) -> Result<f64> {
    assert_eq!(input.as_rule(), Rule::real);
    Ok(input.as_str().parse()?)
}

fn parse_types(input: Pair<Rule>) -> Result<Types> {
    assert_eq!(input.as_rule(), Rule::type_section);
    Ok(Types {})
}

fn parse_sweeps(input: Pair<Rule>) -> Result<Vec<Sweep>> {
    assert_eq!(input.as_rule(), Rule::sweeps);
    let pairs = input.into_inner();
    Ok(pairs.map(parse_sweep).collect::<Result<Vec<_>>>()?)
}

fn parse_sweep(input: Pair<Rule>) -> Result<Sweep> {
    assert_eq!(input.as_rule(), Rule::sweep);
    let mut input = input.into_inner();
    let name = parse_string(input.next().unwrap())?;
    let sweep_type = parse_string(input.next().unwrap())?;
    Ok(Sweep {
        name,
        sweep_type,
        kinds: vec![],
    })
}
