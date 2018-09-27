
use nom::{IResult, space, line_ending};
use nom::types::CompleteStr;
use hashmap::HashMap;

use super::{Instruction, Script};

const CUSTOM_ERR_SYMBOL: u32 = 100;

/// Symbols in script.
/// The first character must be alphabetic, and can include '_' and '-'.
fn symbol(input: CompleteStr) -> IResult<CompleteStr, String> {
    use nom::{Err, Needed, ErrorKind};
    
    if input.len() < 1 {
        Err(Err::Incomplete(Needed::Size(1)))
    } else {
        let c = input.chars().next().unwrap();
        if !c.is_alphabetic() {
            return Err(Err::Error(error_position!(input, ErrorKind::Custom(CUSTOM_ERR_SYMBOL))));
        }
        for (i, c) in input.char_indices() {
            if !(c.is_alphabetic() || c.is_digit(10) || c == '_' || c == '-') {
                let slices = input.split_at(i);
                return Ok((CompleteStr(slices.1), slices.0.to_string()));
            }
        }
        Ok((CompleteStr(""), input[..].to_string()))
    }
}

#[test]
fn symbol_test() {
    assert_eq!(symbol(CompleteStr("abc")), Ok((CompleteStr(""), "abc".to_string())));
    assert_eq!(symbol(CompleteStr("a0d0  ")), Ok((CompleteStr("  "), "a0d0".to_string())));
    assert!(symbol(CompleteStr("01ab")).is_err());
}

named!(end_line<CompleteStr, ()>,
    do_parse!(
        opt!(space) >>
        line_ending >>
        (())
    )
);

#[test]
fn end_line_test() {
    assert_eq!(end_line(CompleteStr("   \naabb")), Ok((CompleteStr("aabb"), ())));
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

named!(section_start<CompleteStr, String>,
    do_parse!(
        tag!("---") >>
        space >>
        s: symbol >>
        end_line >>
        (s)
    )
);

#[test]
fn section_start_test() {
    assert_eq!(section_start(
        CompleteStr("---  section_name \n")), Ok((CompleteStr(""), "section_name".to_string())));
}

named!(jump_instruction<CompleteStr, Instruction>,
    do_parse!(
        ws!(tag!("jump")) >>
        s: delimited!(tag!("("), ws!(symbol), tag!(")")) >>
        end_line >>
        (Instruction::Jump(s))
    )
);

#[test]
fn jump_instruction_test() {
    assert_eq!(
        jump_instruction(CompleteStr(" jump ( other_section ) \n")),
        Ok((CompleteStr(""), Instruction::Jump("other_section".to_owned()))));
}

macro_rules! define_parser_for_noarg_instructions {
    ( $($parser_name:ident, $result:ident, $func_name:expr),* ) => {
        $(
            named!($parser_name<CompleteStr, Instruction>,
                do_parse!(
                    ws!(tag!($func_name)) >>
                    tag!("()") >>
                    end_line >>
                    (Instruction::$result)
                )
            );
        )*
    }
}

define_parser_for_noarg_instructions! {
    shop_buy_instruction, ShopBuy, "shop_buy",
    shop_sell_instruction, ShopSell, "shop_sell",
    get_dungeon_location_instruction, GetDungeonLocation, "get_dungeon_location"
}

#[test]
fn shop_instruction_test() {
    assert_eq!(shop_buy_instruction(
        CompleteStr("shop_buy()\n")), Ok((CompleteStr(""), Instruction::ShopBuy)));
    assert_eq!(shop_sell_instruction(
        CompleteStr("shop_sell()\n")), Ok((CompleteStr(""), Instruction::ShopSell)));
}

named!(talk_instruction<CompleteStr, Instruction>,
    do_parse!(
        ws!(tag!("talk")) >>
        s: delimited!(tag!("("), ws!(symbol), tag!(")")) >>
        end_line >>
        (Instruction::Talk(s, Vec::new()))
    )
);

named!(talk_instruction_with_choices<CompleteStr, Instruction>,
    do_parse!(
        ws!(tag!("talk")) >>
        tag!("(") >>
        section: ws!(symbol) >>
        tag!(",") >>
        choices: array!(delimited!(
            char!('('), separated_pair!(ws!(symbol), char!(','), ws!(symbol)), char!(')') )) >>
/*        choices: ws!(delimited!(char!('['), separated_list!(
            char!(','),
            ws!(delimited!(char!('('), separated_pair!(ws!(symbol), char!(','), ws!(symbol)), char!(')')))), char!(']'))) >>*/
        tag!(")") >>
        end_line >>
        (Instruction::Talk(section, choices))
    )
);

#[test]
fn talk_instruction_test() {
    let result = Instruction::Talk(
        "text_id".to_owned(),
        vec![("a".to_owned(), "b".to_owned()), ("c".to_owned(), "d".to_owned())]);
    assert_eq!(
        talk_instruction_with_choices(CompleteStr("talk(text_id, [(a, b), (c, d)])\n")),
        Ok((CompleteStr(""), result)));
}

named!(instruction<CompleteStr, Instruction>,
    alt!(
        jump_instruction |
        talk_instruction_with_choices |
        talk_instruction |
        shop_buy_instruction |
        shop_sell_instruction |
        get_dungeon_location_instruction
    )
);

named!(section<CompleteStr, (String, Vec<Instruction>)>,
    do_parse!(
        section: section_start >>
        instructions: many0!(instruction) >>
        (section.to_string(), instructions)
    )
);

named!(pub parse<CompleteStr, Script>,
    fold_many0!(
        complete!(section),
        HashMap::default(),
        | mut map: HashMap<String, Vec<Instruction>>, section: (String, Vec<Instruction>) | {
            map.insert(section.0, section.1);
            map
        })
);

#[test]
fn parse_test() {
    let script = r#"--- test_section0
talk(textid0)
shop_buy()
jump(test_section1)
--- test_section1
talk(textid1,
     [(aaa, bbb), (ccc, ddd)])
"#;
    let mut result: Script = HashMap::default();

    result.insert(
        "test_section0".to_owned(),
        vec![
            Instruction::Talk("textid0".to_owned(), vec![]),
            Instruction::ShopBuy,
            Instruction::Jump("test_section1".to_owned())
        ]);
    result.insert(
        "test_section1".to_owned(),
        vec![
            Instruction::Talk(
                "textid1".to_owned(),
                vec![("aaa".to_owned(), "bbb".to_owned()), ("ccc".to_owned(), "ddd".to_owned())]),
        ]);

    assert_eq!(parse(CompleteStr(script)), Ok((CompleteStr(""), result)))
}

