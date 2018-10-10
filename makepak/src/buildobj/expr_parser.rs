
use common::script::{Expr, Value};
use nom::{digit1, space};
use nom::types::CompleteStr;

/// Id as String in script.
/// The first character must be alphabetic or numeric, and can include '_', '-', and '.'.
named!(pub id<CompleteStr, String>,
    do_parse!(
        s: re_find_static!("[a-zA-Z0-9][a-zA-Z0-9_.-]*") >>
        (s.to_string())
    )
);

#[test]
fn id_test() {
    assert_eq!(id(CompleteStr("ab.c")), Ok((CompleteStr(""), "ab.c".to_string())));
    assert_eq!(id(CompleteStr("abc-def ")), Ok((CompleteStr(" "), "abc-def".to_string())));
}

named!(true_literal<CompleteStr, Expr>,
    do_parse!(
        tag!("true") >>
        (Expr::Value(Value::Bool(true)))
    )
);

named!(false_literal<CompleteStr, Expr>,
    do_parse!(
        tag!("false") >>
        (Expr::Value(Value::Bool(false)))
    )
);

named!(integer<CompleteStr, Expr>,
    do_parse!(
        i: digit1 >>
        (Expr::Value(Value::Int(i32::from_str_radix(&i, 10).unwrap())))
    )
);

named!(gvar<CompleteStr, Expr>,
    do_parse!(
        char!('$') >>
        char!('(') >>
        var_name: id >>
        char!(')') >>
        (Expr::GVar(var_name))
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
        true_literal |
        false_literal |
        integer |
        gvar |
        has_item
    )
);

#[test]
fn expr_test() {
    assert_eq!(expr(CompleteStr("true")), Ok((CompleteStr(""), Expr::Value(Value::Bool(true)))));
    assert_eq!(expr(CompleteStr("false")), Ok((CompleteStr(""), Expr::Value(Value::Bool(false)))));
    assert_eq!(expr(CompleteStr("1234")), Ok((CompleteStr(""), Expr::Value(Value::Int(1234)))));
    assert_eq!(expr(CompleteStr("$(aa)")), Ok((CompleteStr(""), Expr::GVar("aa".to_owned()))));
    let a = Expr::HasItem("box".to_owned());
    assert_eq!(expr(CompleteStr("has_item(box)")), Ok((CompleteStr(""), a)));
}

