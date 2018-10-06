
use common::script::{Expr, Value};
use nom::{digit1, space, IResult};
use nom::types::CompleteStr;

const CUSTOM_ERR_SYMBOL: u32 = 100;
const CUSTOM_ERR_ID: u32 = 101;

/// Symbols in script.
/// The first character must be alphabetic, and can include '_' and '-'.
pub fn symbol(input: CompleteStr) -> IResult<CompleteStr, String> {
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

/// Id as String in script.
/// The first character must be alphabetic, and can include '_', '-', and '.'.
pub fn id(input: CompleteStr) -> IResult<CompleteStr, String> {
    use nom::{Err, Needed, ErrorKind};
    
    if input.len() < 1 {
        Err(Err::Incomplete(Needed::Size(1)))
    } else {
        let c = input.chars().next().unwrap();
        if !c.is_alphabetic() {
            return Err(Err::Error(error_position!(input, ErrorKind::Custom(CUSTOM_ERR_ID))));
        }
        for (i, c) in input.char_indices() {
            if !(c.is_alphabetic() || c.is_digit(10) || c == '_' || c == '-' || c == '.') {
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

#[test]
fn id_test() {
    assert_eq!(id(CompleteStr("ab.c")), Ok((CompleteStr(""), "ab.c".to_string())));
}

named!(integer<CompleteStr, Expr>,
    do_parse!(
        i: digit1 >>
        (Expr::Value(Value::Int(i32::from_str_radix(&i, 10).unwrap())))
    )
);

named!(has_item<CompleteStr, Expr>,
    do_parse!(
        tag!("has_item") >>
        opt!(space) >>
        s: delimited!(tag!("("), ws!(id), tag!(")")) >>
        (Expr::HasItem(s))
    )
);

named!(pub expr<CompleteStr, Expr>,
    alt!(
        integer |
        has_item
    )
);

#[test]
fn expr_test() {
    assert_eq!(expr(CompleteStr("1234")), Ok((CompleteStr(""), Expr::Value(Value::Int(1234)))));
    let a = Expr::HasItem("box".to_owned());
    assert_eq!(expr(CompleteStr("has_item(box)")), Ok((CompleteStr(""), a)));
}

