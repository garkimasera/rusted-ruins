
use common::script::{Expr, Value, Operator};
use nom::{digit1, space};
use nom::types::CompleteStr;

trait Join {
    fn join(self, op: Operator, e: Expr) -> Expr;
}

impl Join for Expr {
    fn join(self, op: Operator, e: Expr) -> Expr {
        match self {
            Expr::Term(mut v) => {
                v.push((op, e));
                Expr::Term(v)
            }
            _ => {
                Expr::Term(vec![(Operator::None, self), (op, e)])
            }
        }
    }
}

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
        s: delimited!(char!('('), ws!(id), char!(')')) >>
        (Expr::HasItem(s))
    )
);

named!(factor<CompleteStr, Expr>,
    ws!(alt_complete!(
        true_literal |
        false_literal |
        integer |
        gvar |
        has_item |
        parens
    ))
);

named!(parens<CompleteStr, Expr>,
    delimited!(
        char!('('),
        ws!(expr),
        char!(')')
    )
);

named!(pub expr<CompleteStr, Expr>,
    do_parse!(
        init: factor >>
        res: fold_many0!(
            pair!(char!('+'), factor),
            init,
            |a: Expr, (_op, e): (char, Expr)| {
                a.join(Operator::Add, e)
            }
        ) >>
        (res)
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
    assert_eq!(
        expr(CompleteStr("1 + 2 + 3")),
        Ok((CompleteStr(""),
            Expr::Term(vec![
                (Operator::None, Expr::Value(Value::Int(1))),
                (Operator::Add, Expr::Value(Value::Int(2))),
                (Operator::Add, Expr::Value(Value::Int(3)))])
        )));
}

