use anyhow::{anyhow, Result};
use common::gamedata::Value;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till1, take_until},
    character::{complete::space0, is_space},
    combinator::map,
    multi::separated_list0,
    IResult,
};
use once_cell::sync::Lazy;
use regex::Regex;

type ArgsMap = std::collections::HashMap<String, Value>;

static RE_SCRIPT_ID: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[!_\-a-zA-Z]+$").unwrap());
static RE_INPUT_WITH_ARGS: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^([!_\-a-zA-Z0-9]+)\((.+)\)$").unwrap());

pub fn parse_input(input: &str) -> Result<(String, ArgsMap)> {
    if RE_SCRIPT_ID.is_match(input) {
        return Ok((input.into(), ArgsMap::default()));
    }

    let invalid_input_err = || anyhow!("invalid script input \"{}\"", input);

    let caps = RE_INPUT_WITH_ARGS
        .captures(input)
        .ok_or_else(invalid_input_err)?;
    let id = caps.get(1).ok_or_else(invalid_input_err)?.as_str();
    let args = caps.get(2).ok_or_else(invalid_input_err)?.as_str();

    let args_map = parse_args(args)
        .map_err(|_| invalid_input_err())
        .and_then(|result| {
            if result.0.is_empty() {
                Ok(result)
            } else {
                Err(invalid_input_err())
            }
        })?
        .1;

    Ok((id.into(), args_map))
}

fn parse_args(input: &str) -> IResult<&str, ArgsMap> {
    map(separated_list0(tag(","), parse_arg), |args| {
        args.into_iter().collect()
    })(input)
}

fn parse_arg(input: &str) -> IResult<&str, (String, Value)> {
    let (input, _) = space0(input)?;
    let (input, arg_name) = take_till1(|c: char| c == '=' || is_space(c as u8))(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, _) = space0(input)?;
    let (input, value) = value(input)?;
    let (input, _) = space0(input)?;
    Ok((input, (arg_name.into(), value)))
}

fn value(input: &str) -> IResult<&str, Value> {
    alt((value_string, value_int))(input)
}

fn value_string(input: &str) -> IResult<&str, Value> {
    let (input, _) = tag("\'")(input)?;
    let (input, s) = take_until("\'")(input)?;
    let (input, _) = tag("\'")(input)?;
    Ok((input, Value::String(s.into())))
}

fn value_int(input: &str) -> IResult<&str, Value> {
    let (input, i) = nom::character::complete::i64(input)?;
    Ok((input, Value::Int(i)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input_test() {
        assert_eq!(
            parse_arg("arg0='abc'").unwrap(),
            ("", ("arg0".into(), Value::String("abc".into())))
        );
        assert_eq!(
            parse_arg("a=42").unwrap(),
            ("", ("a".into(), Value::Int(42)))
        );

        assert_eq!(
            parse_input("!example-id").unwrap(),
            ("!example-id".to_owned(), ArgsMap::default())
        );

        let result = parse_input("!example-id(arg0='abc')").unwrap();
        assert_eq!(&result.0, "!example-id");
        let mut args_map = ArgsMap::default();
        args_map.insert("arg0".into(), Value::String("abc".into()));
        assert_eq!(result.1, args_map);

        let result = parse_input("!example-id(arg0 ='(abc)', arg1= 42 )").unwrap();
        assert_eq!(&result.0, "!example-id");
        let mut args_map = ArgsMap::default();
        args_map.insert("arg0".into(), Value::String("(abc)".into()));
        args_map.insert("arg1".into(), Value::Int(42));
        assert_eq!(result.1, args_map);
    }
}
