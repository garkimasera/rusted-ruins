
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

// Operator precedence is the same as C.
// term_mul > term_plus > term_ord > term_eq > term_and > expr

named!(term_mul<CompleteStr, Expr>,
    ws!(do_parse!(
        init: factor >>
        res: fold_many0!(
            pair!(alt!(char!('*') | char!('/')), factor),
            init,
            |a: Expr, (op, e): (char, Expr)| {
                let op = match op {
                    '*' => Operator::Mul,
                    '/' => Operator::Div,
                    _ => unreachable!(),
                };
                a.join(op, e)
            }
        ) >>
        (res)
    ))
);

named!(term_plus<CompleteStr, Expr>,
    ws!(do_parse!(
        init: term_mul >>
        res: fold_many0!(
            pair!(alt!(char!('+') | char!('-')), term_mul),
            init,
            |a: Expr, (op, e): (char, Expr)| {
                let op = match op {
                    '+' => Operator::Add,
                    '-' => Operator::Sub,
                    _ => unreachable!(),
                };
                a.join(op, e)
            }
        ) >>
        (res)
    ))
);

named!(term_ord<CompleteStr, Expr>,
    ws!(do_parse!(
        init: term_plus >>
        res: fold_many0!(
            pair!(alt!(tag!("<=") | tag!(">=") | tag!("<") | tag!(">")), term_plus),
            init,
            |a: Expr, (op, e): (CompleteStr, Expr)| {
                let op = match op {
                    CompleteStr("<") => Operator::Less,
                    CompleteStr("<=") => Operator::LessEq,
                    CompleteStr(">") => Operator::Greater,
                    CompleteStr(">=") => Operator::GreaterEq,
                    _ => unreachable!(),
                };
                a.join(op, e)
            }
        ) >>
        (res)
    ))
);

named!(term_eq<CompleteStr, Expr>,
    ws!(do_parse!(
        init: term_ord >>
        res: fold_many0!(
            pair!(alt!(tag!("==") | tag!("!=")), term_ord),
            init,
            |a: Expr, (op, e): (CompleteStr, Expr)| {
                let op = match op {
                    CompleteStr("==") => Operator::Eq,
                    CompleteStr("!=") => Operator::NotEq,
                    _ => unreachable!(),
                };
                a.join(op, e)
            }
        ) >>
        (res)
    ))
);

named!(term_and<CompleteStr, Expr>,
    ws!(do_parse!(
        init: term_eq >>
        res: fold_many0!(
            pair!(tag!("&&"), term_eq),
            init,
            |a: Expr, (_, e): (CompleteStr, Expr)| {
                a.join(Operator::And, e)
            }
        ) >>
        (res)
    ))
);

named!(pub expr<CompleteStr, Expr>,
    ws!(do_parse!(
        init: term_and >>
        res: fold_many0!(
            pair!(tag!("||"), term_and),
            init,
            |a: Expr, (_, e): (CompleteStr, Expr)| {
                a.join(Operator::Or, e)
            }
        ) >>
        (res)
    ))
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
        expr(CompleteStr("1 * 2 + 3")),
        Ok((CompleteStr(""),
            Expr::Term(vec![
                (Operator::None, Expr::Value(Value::Int(1))),
                (Operator::Mul, Expr::Value(Value::Int(2))),
                (Operator::Add, Expr::Value(Value::Int(3)))])
        )));
    assert_eq!(
        expr(CompleteStr("1 + 2 * 3")),
        Ok((CompleteStr(""),
            Expr::Term(vec![
                (Operator::None, Expr::Value(Value::Int(1))),
                (Operator::Add, Expr::Term(vec![
                    (Operator::None, Expr::Value(Value::Int(2))),
                    (Operator::Mul, Expr::Value(Value::Int(3))),
                ]))
            ])
        )));
   assert_eq!(
        expr(CompleteStr("3 == 1 + 2")),
        Ok((CompleteStr(""),
            Expr::Term(vec![
                (Operator::None, Expr::Value(Value::Int(3))),
                (Operator::Eq, Expr::Term(vec![
                    (Operator::None, Expr::Value(Value::Int(1))),
                    (Operator::Add, Expr::Value(Value::Int(2))),
                ])),
            ])
        )));
    let term_ne_example = Expr::Term(vec![
        (Operator::None, Expr::Value(Value::Int(1))),
        (Operator::NotEq, Expr::Value(Value::Int(2)))]);
    assert_eq!(
        expr(CompleteStr("1 != 2")),
        Ok((CompleteStr(""), term_ne_example)));
    let term_ord_example = Expr::Term(vec![
        (Operator::None, Expr::Value(Value::Int(1))),
        (Operator::LessEq, Expr::Value(Value::Int(2)))]);
    assert_eq!(
        expr(CompleteStr("1 <= 2")),
        Ok((CompleteStr(""), term_ord_example.clone())));
    assert_eq!(
        expr(CompleteStr("1 != 2 || 1 <= 2")),
        Ok((CompleteStr(""),
            Expr::Term(vec![
                (Operator::None, Expr::Value(Value::Int(1))),
                (Operator::NotEq, Expr::Value(Value::Int(2))),
                (Operator::Or, term_ord_example),
            ])
        )));
}

