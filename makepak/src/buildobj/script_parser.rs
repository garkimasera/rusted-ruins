use common::hashmap::HashMap;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, space0};
use nom::multi::many0;
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

macro_rules! array(
    ($i:expr, $submac:ident!( $($args:tt)* )) => ({
        ws!($i, delimited!(
            char!('['),
            separated_list!(
                char!(','),
                ws!( $submac!($($args)*) )),
            char!(']')))
    });
    ($i:expr, $f:expr) => (
        array!($i, call!($f));
    );
);

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

named!(jump_instruction<&str, Instruction>,
    do_parse!(
        ws!(tag!("jump")) >>
        s: delimited!(tag!("("), ws!(id), tag!(")")) >>
        end_line >>
        (Instruction::Jump(s))
    )
);

named!(jump_if_instruction<&str, Instruction>,
    do_parse!(
        ws!(tag!("jump_if")) >>
        char!('(') >>
        s: ws!(id) >>
        char!(',') >>
        e: ws!(expr) >>
        char!(')') >>
        end_line >>
        (Instruction::JumpIf(s, e))
    )
);

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

named!(special_instruction<&str, Instruction>,
    do_parse!(
        ws!(tag!("special")) >>
        s: map_res!(delimited!(tag!("("), ws!(symbol), tag!(")")), FromStr::from_str) >>
        end_line >>
        (Instruction::Special(s))
    )
);

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

named!(talk_instruction<&str, Instruction>,
    do_parse!(
        ws!(tag!("talk")) >>
        text_id: delimited!(char!('('), ws!(id), char!(')')) >>
        end_line >>
        (Instruction::Talk(text_id, Vec::new()))
    )
);

named!(talk_instruction_with_choices<&str, Instruction>,
    do_parse!(
        ws!(tag!("talk")) >>
        char!('(') >>
        text_id: ws!(id) >>
        char!(',') >>
        choices: array!(delimited!(
            char!('('), separated_pair!(complete!(ws!(id)), complete!(char!(',')), complete!(ws!(id))), complete!(char!(')')) )) >>
        char!(')') >>
        end_line >>
        (Instruction::Talk(text_id, choices))
    )
);

named!(gset_instruction<&str, Instruction>,
    do_parse!(
        ws!(tag!("gset")) >>
        char!('(') >>
        var_name: ws!(id) >>
        char!(',') >>
        value: ws!(expr) >>
        char!(')') >>
        end_line >>
        (Instruction::GSet(var_name, value))
    )
);

named!(receive_money_instruction<&str, Instruction>,
    do_parse!(
        ws!(tag!("receive_money")) >>
        e: delimited!(char!('('), ws!(expr), char!(')')) >>
        end_line >>
        (Instruction::ReceiveMoney(e))
    )
);

named!(remove_item_instruction<&str, Instruction>,
    do_parse!(
        ws!(tag!("remove_item")) >>
        item_id: delimited!(char!('('), ws!(id), char!(')')) >>
        end_line >>
        (Instruction::RemoveItem(item_id))
    )
);

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

named!(instruction<&str, Instruction>,
    alt!(
        complete!(jump_instruction) |
        complete!(jump_if_instruction) |
        complete!(talk_instruction_with_choices) |
        complete!(talk_instruction) |
        complete!(gset_instruction) |
        complete!(receive_money_instruction) |
        complete!(remove_item_instruction) |
        complete!(special_instruction)
    )
);

fn section(input: &str) -> IResult<&str, (String, Vec<Instruction>)> {
    let (input, section) = section_start(input)?;
    let (input, instructions) = many0(instruction)(input)?;
    Ok((input, (section.to_owned(), instructions)))
}

named!(sections<&str, HashMap<String, Vec<Instruction>>>,
    exact!(fold_many0!(
        complete!(section),
        HashMap::default(),
        | mut s: HashMap<String, Vec<Instruction>>, section: (String, Vec<Instruction>) | {
            s.insert(section.0, section.1);
            s
        }))
);

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
