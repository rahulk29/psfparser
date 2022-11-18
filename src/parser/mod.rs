use pest::iterators::Pair;
use pest::Parser;

use crate::parser::ast::{Header, Kind, NamedValue, Prop, PsfAst, SignalValues, Sweep, Trace, Value};
use crate::Result;

use self::ast::TypeDef;

pub mod ast;
#[cfg(test)]
mod tests;

#[derive(Parser)]
#[grammar = "psf_ascii.pest"]
pub struct PsfAsciiParser;

pub fn parse(input: &str) -> Result<PsfAst> {
    let input = PsfAsciiParser::parse(Rule::psf_ascii, input)?
        .next()
        .unwrap();
    parse_psf_inner(input)
}

fn parse_psf_inner(input: Pair<Rule>) -> Result<PsfAst> {
    assert_eq!(input.as_rule(), Rule::psf_ascii_inner);
    let mut pairs = input.into_inner();
    let header = parse_header(pairs.next().unwrap())?;

    let (types, sweeps, traces, values) = if let Some(input) = pairs.next() {
        let mut input = input.into_inner();
        let types = parse_types(input.next().unwrap())?;
        let sweeps = parse_sweeps(pairs.next().unwrap().into_inner().next().unwrap())?;
        let traces = parse_traces(pairs.next().unwrap().into_inner().next().unwrap())?;
        let values = parse_value_section(pairs.next().unwrap())?;
        (types, sweeps, traces, values)
    } else {
        (Vec::new(), Vec::new(), Vec::new(), Vec::new())
    };

    Ok(PsfAst {
        header,
        types,
        sweeps,
        traces,
        values,
    })
}

fn parse_header(input: Pair<Rule>) -> Result<Header> {
    assert_eq!(input.as_rule(), Rule::header_section);
    let mut pairs = input.into_inner();
    let named_values = pairs.next().unwrap();
    let values = parse_named_values(named_values)?;
    Ok(Header { values })
}

fn parse_named_values(input: Pair<Rule>) -> Result<Vec<NamedValue>> {
    assert_eq!(input.as_rule(), Rule::named_values);
    let pairs = input.into_inner();
    pairs.map(parse_named_value).collect::<Result<Vec<_>>>()
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
        Rule::string => Value::Str(parse_string(input)?),
        Rule::integer => Value::Int(parse_integer(input)?),
        Rule::real => Value::Real(parse_real(input)?),
        Rule::nan => Value::NaN,
        _ => unreachable!("unexpected value rule"),
    })
}

fn parse_string(input: Pair<Rule>) -> Result<&str> {
    assert_eq!(input.as_rule(), Rule::string);
    Ok(input.into_inner().next().unwrap().as_str())
}

fn parse_integer(input: Pair<Rule>) -> Result<i64> {
    assert_eq!(input.as_rule(), Rule::integer);
    Ok(input.as_str().parse()?)
}

fn parse_real(input: Pair<Rule>) -> Result<f64> {
    assert_eq!(input.as_rule(), Rule::real);
    Ok(input.as_str().parse()?)
}

fn parse_types(input: Pair<Rule>) -> Result<Vec<TypeDef>> {
    assert_eq!(input.as_rule(), Rule::types);
    let pairs = input.into_inner();
    pairs.map(parse_type).collect::<Result<Vec<_>>>()
}

fn parse_type(input: Pair<Rule>) -> Result<TypeDef> {
    assert_eq!(input.as_rule(), Rule::type_def);
    let mut input = input.into_inner();
    let name = parse_string(input.next().unwrap())?;
    let kinds = parse_kinds(input.next().unwrap())?;

    Ok(TypeDef { name, kinds })
}

fn parse_sweeps(input: Pair<Rule>) -> Result<Vec<Sweep>> {
    assert_eq!(input.as_rule(), Rule::sweeps);
    let pairs = input.into_inner();
    pairs.map(parse_sweep).collect::<Result<Vec<_>>>()
}

fn parse_sweep(input: Pair<Rule>) -> Result<Sweep> {
    assert_eq!(input.as_rule(), Rule::sweep);
    let mut input = input.into_inner();
    let name = parse_string(input.next().unwrap())?;
    let sweep_type = parse_string(input.next().unwrap())?;
    let kinds = parse_kinds(input.next().unwrap())?;
    Ok(Sweep {
        name,
        sweep_type,
        kinds,
    })
}

fn parse_kinds(input: Pair<Rule>) -> Result<Vec<Kind>> {
    assert_eq!(input.as_rule(), Rule::kinds);
    let pairs = input.into_inner();
    pairs.map(parse_kind).collect::<Result<Vec<_>>>()
}

fn parse_kind(input: Pair<Rule>) -> Result<Kind> {
    assert_eq!(input.as_rule(), Rule::kind);
    let input = input.into_inner().next().unwrap();

    Ok(match input.as_rule() {
        Rule::t_float => Kind::Float,
        Rule::t_double => Kind::Double,
        Rule::t_complex => Kind::Complex,
        Rule::t_int => Kind::Int,
        Rule::t_byte => Kind::Byte,
        Rule::t_long => Kind::Long,
        Rule::t_string => Kind::String,
        Rule::prop => Kind::Prop(parse_prop(input)?),
        _ => panic!("Unexpected kind"),
    })
}

fn parse_prop(input: Pair<Rule>) -> Result<Prop> {
    assert_eq!(input.as_rule(), Rule::prop);
    let named_values = input.into_inner().next().unwrap();
    let values = parse_named_values(named_values)?;
    Ok(Prop { values })
}

fn parse_traces(input: Pair<Rule>) -> Result<Vec<Trace>> {
    assert_eq!(input.as_rule(), Rule::traces);
    let pairs = input.into_inner();
    pairs.map(parse_trace).collect::<Result<Vec<_>>>()
}

fn parse_trace(input: Pair<Rule>) -> Result<Trace> {
    assert_eq!(input.as_rule(), Rule::trace);
    let input = input.into_inner().next().unwrap();
    Ok(match input.as_rule() {
        Rule::trace_group => parse_trace_group(input)?,
        Rule::trace_with_props | Rule::simple_trace => parse_simple_trace(input)?,
        _ => panic!("Unexpected trace format"),
    })
}

fn parse_trace_group(input: Pair<Rule>) -> Result<Trace> {
    assert_eq!(input.as_rule(), Rule::trace_group);
    let mut pairs = input.into_inner();
    let name = parse_string(pairs.next().unwrap())?;
    let count = parse_integer(pairs.next().unwrap())?;
    Ok(Trace::Group { name, count })
}

fn parse_simple_trace(input: Pair<Rule>) -> Result<Trace> {
    assert!(input.as_rule() == Rule::simple_trace || input.as_rule() == Rule::trace_with_props);
    let mut pairs = input.into_inner();
    let name = parse_string(pairs.next().unwrap())?;
    let units = parse_string(pairs.next().unwrap())?;
    Ok(Trace::Signal { name, units })
}

fn parse_value_section(input: Pair<Rule>) -> Result<Vec<SignalValues>> {
    assert_eq!(input.as_rule(), Rule::value_section);
    let pairs = input.into_inner();
    pairs.map(parse_signal_value).collect::<Result<Vec<_>>>()
}

fn parse_signal_value(input: Pair<Rule>) -> Result<SignalValues> {
    assert_eq!(input.as_rule(), Rule::signal_value);
    let input = input.into_inner().next().unwrap();
    Ok(match input.as_rule() {
        Rule::signal_value_simple => parse_signal_value_simple(input)?,
        _ => panic!("Unexpected signal value"),
    })
}

fn parse_signal_value_simple(input: Pair<Rule>) -> Result<SignalValues> {
    assert_eq!(input.as_rule(), Rule::signal_value_simple);
    let mut input = input.into_inner();
    let signal = parse_string(input.next().unwrap())?;
    let values = parse_numbers(input.next().unwrap())?;
    Ok(SignalValues { signal, values })
}

fn parse_numbers(input: Pair<Rule>) -> Result<Vec<f64>> {
    assert_eq!(input.as_rule(), Rule::simple_numbers);
    let pairs = input.into_inner();
    pairs.map(parse_real).collect::<Result<Vec<_>>>()
}
