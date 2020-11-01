use common::hashmap::HashMap;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, line_ending, multispace0, space0};
use nom::combinator::map_res;
use nom::error::ParseError;
use nom::multi::{fold_many0, many0, separated_list0};
use nom::sequence::{delimited, separated_pair};
use nom::IResult;
use std::str::FromStr;

use super::expr_parser::*;
use crate::error::PakCompileError;
use common::script::*;

fn end_line(input: &str) -> IResult<&str, ()> {
    let (input, _) = space0(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, ()))
}

#[test]
fn end_line_test() {
    assert_eq!(end_line("   \naabb"), Ok(("aabb", ())));
}

fn ws<I, O, E: ParseError<I>, F>(mut f: F) -> impl FnMut(I) -> IResult<I, O, E>
where
    I: nom::InputTakeAtPosition,
    <I as nom::InputTakeAtPosition>::Item: nom::AsChar + Clone,
    F: FnMut(I) -> IResult<I, O, E>,
{
    move |input: I| {
        let (input, _) = multispace0(input)?;
        let (input, o) = f(input)?;
        let (input, _) = multispace0(input)?;
        Ok((input, o))
    }
}

fn array<I, O, E: ParseError<I>, F>(f: F) -> impl FnMut(I) -> IResult<I, Vec<O>, E>
where
    I: nom::InputTakeAtPosition
        + nom::Slice<std::ops::RangeFrom<usize>>
        + nom::InputIter
        + Clone
        + PartialEq,
    <I as nom::InputTakeAtPosition>::Item: nom::AsChar + Clone,
    <I as nom::InputIter>::Item: nom::AsChar,
    F: FnMut(I) -> IResult<I, O, E>,
{
    ws(delimited(
        char('['),
        separated_list0(char(','), ws(f)),
        char(']'),
    ))
}

fn section_start(input: &str) -> IResult<&str, String> {
    let (input, _) = tag("---")(input)?;
    let (input, _) = space0(input)?;
    let (input, s) = id(input)?;
    let (input, _) = end_line(input)?;
    Ok((input, s))
}

#[test]
fn section_start_test() {
    assert_eq!(
        section_start("---  section_name \n"),
        Ok(("", "section_name".to_string()))
    );
}

fn jump_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = ws(tag("jump"))(input)?;
    let (input, id) = delimited(char('('), ws(id), char(')'))(input)?;
    let (input, _) = end_line(input)?;
    Ok((input, Instruction::Jump(id)))
}

fn jump_if_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = ws(tag("jump_if"))(input)?;
    let (input, _) = char('(')(input)?;
    let (input, id) = ws(id)(input)?;
    let (input, _) = char(',')(input)?;
    let (input, expr) = ws(expr)(input)?;
    let (input, _) = char(')')(input)?;
    let (input, _) = end_line(input)?;
    Ok((input, Instruction::JumpIf(id, expr)))
}

#[test]
fn jump_instruction_test() {
    assert_eq!(
        jump_instruction(" jump ( other_section ) \n"),
        Ok(("", Instruction::Jump("other_section".to_owned())))
    );
    assert_eq!(
        jump_if_instruction("jump_if(has-key, has_item(key))\n"),
        Ok((
            "",
            Instruction::JumpIf("has-key".to_owned(), Expr::HasItem("key".to_owned()))
        ))
    );
}

fn special_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = ws(tag("special"))(input)?;
    let (input, s) = map_res(
        delimited(char('('), ws(symbol), char(')')),
        FromStr::from_str,
    )(input)?;
    let (input, _) = end_line(input)?;
    Ok((input, Instruction::Special(s)))
}

#[test]
fn special_instruction_test() {
    assert_eq!(
        special_instruction("special(shop_buy)\n"),
        Ok(("", Instruction::Special(SpecialInstruction::ShopBuy)))
    );
    assert_eq!(
        special_instruction("special(shop_sell)\n"),
        Ok(("", Instruction::Special(SpecialInstruction::ShopSell)))
    );
}

fn talk_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = ws(tag("talk"))(input)?;
    let (input, text_id) = delimited(char('('), ws(id), char(')'))(input)?;
    let (input, _) = end_line(input)?;
    Ok((input, Instruction::Talk(text_id, Vec::new())))
}

fn talk_instruction_with_choices(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = ws(tag("talk"))(input)?;
    let (input, _) = char('(')(input)?;
    let (input, text_id) = ws(id)(input)?;
    let (input, _) = char(',')(input)?;
    let (input, choices) = array(delimited(
        char('('),
        separated_pair(ws(id), char(','), ws(id)),
        char(')'),
    ))(input)?;
    let (input, _) = char(')')(input)?;
    let (input, _) = end_line(input)?;
    Ok((input, Instruction::Talk(text_id, choices)))
}

fn gset_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = ws(tag("gset"))(input)?;
    let (input, _) = char('(')(input)?;
    let (input, var_name) = ws(id)(input)?;
    let (input, _) = char(',')(input)?;
    let (input, value) = ws(expr)(input)?;
    let (input, _) = char(')')(input)?;
    let (input, _) = end_line(input)?;
    Ok((input, Instruction::GSet(var_name, value)))
}

fn receive_item_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = ws(tag("receive_item"))(input)?;
    let (input, _) = char('(')(input)?;
    let (input, id) = ws(id)(input)?;
    let (input, _) = char(',')(input)?;
    let (input, n) = ws(expr)(input)?;
    let (input, _) = char(')')(input)?;
    let (input, _) = end_line(input)?;
    Ok((input, Instruction::ReceiveItem(id, n)))
}

fn receive_money_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = ws(tag("receive_money"))(input)?;
    let (input, expr) = delimited(char('('), ws(expr), char(')'))(input)?;
    let (input, _) = end_line(input)?;
    Ok((input, Instruction::ReceiveMoney(expr)))
}

fn remove_item_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = ws(tag("remove_item"))(input)?;
    let (input, item_id) = delimited(char('('), ws(id), char(')'))(input)?;
    let (input, _) = end_line(input)?;
    Ok((input, Instruction::RemoveItem(item_id)))
}

#[test]
fn talk_instruction_test() {
    let result = Instruction::Talk(
        "text-id".to_owned(),
        vec![
            ("a".to_owned(), "b".to_owned()),
            ("c".to_owned(), "d".to_owned()),
        ],
    );
    assert_eq!(
        talk_instruction_with_choices("talk(text-id, [(a, b), (c, d)])\n"),
        Ok(("", result))
    );
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        jump_instruction,
        jump_if_instruction,
        talk_instruction_with_choices,
        talk_instruction,
        gset_instruction,
        receive_item_instruction,
        receive_money_instruction,
        remove_item_instruction,
        special_instruction,
    ))(input)
}

fn section(input: &str) -> IResult<&str, (String, Vec<Instruction>)> {
    let (input, section) = section_start(input)?;
    let (input, instructions) = many0(instruction)(input)?;
    Ok((input, (section.to_owned(), instructions)))
}

fn sections(input: &str) -> IResult<&str, HashMap<String, Vec<Instruction>>> {
    fold_many0(
        section,
        HashMap::default(),
        |mut s: HashMap<String, Vec<Instruction>>, section: (String, Vec<Instruction>)| {
            s.insert(section.0, section.1);
            s
        },
    )(input)
}

pub fn parse(input: &str) -> Result<Script, PakCompileError> {
    match sections(input) {
        Ok(o) => Ok(Script::from_map(o.1)),
        Err(e) => Err(PakCompileError::ScriptParseError {
            description: format!("{:?}", e),
        }),
    }
}

#[test]
fn parse_test() {
    let script = r#"--- test_section0
talk(textid0)
special(shop_buy)
jump(test_section1)
--- test_section1
talk(textid1,
     [(aaa, bbb), (ccc, ddd)])
"#;
    let mut result = HashMap::default();

    result.insert(
        "test_section0".to_owned(),
        vec![
            Instruction::Talk("textid0".to_owned(), vec![]),
            Instruction::Special(SpecialInstruction::ShopBuy),
            Instruction::Jump("test_section1".to_owned()),
        ],
    );
    result.insert(
        "test_section1".to_owned(),
        vec![Instruction::Talk(
            "textid1".to_owned(),
            vec![
                ("aaa".to_owned(), "bbb".to_owned()),
                ("ccc".to_owned(), "ddd".to_owned()),
            ],
        )],
    );

    assert_eq!(sections(script), Ok(("", result)))
}
