use nom::IResult;
use nom::character::complete::*;
use common::script::{Expr, Operator, Value};

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
            _ => Expr::Term(vec![(Operator::None, self), (op, e)]),
        }
    }
}

/// Id as String in script.
/// The first character must be alphabetic or numeric, and can include '_', '-', and '.'.
named!(pub id<&str, String>,
    do_parse!(
        s: re_find_static!("[a-zA-Z0-9][a-zA-Z0-9_.-]*") >>
        (s.to_string())
    )
);

named!(pub symbol<&str, &str>,
    do_parse!(
        s: re_find_static!("[a-zA-Z][a-zA-Z0-9_]*") >>
        (&*s)
    )
);

#[test]
fn id_test() {
    assert_eq!(
        id("ab.c"),
        Ok(("", "ab.c".to_string()))
    );
    assert_eq!(
        id("abc-def "),
        Ok((" ", "abc-def".to_string()))
    );
}

named!(true_literal<&str, Expr>,
    do_parse!(
        tag!("true") >>
        (Expr::Value(Value::Bool(true)))
    )
);

named!(false_literal<&str, Expr>,
    do_parse!(
        tag!("false") >>
        (Expr::Value(Value::Bool(false)))
    )
);

named!(integer<&str, Expr>,
    do_parse!(
        i: digit1 >>
        (Expr::Value(Value::Int(i32::from_str_radix(&i, 10).unwrap())))
    )
);

named!(gvar<&str, Expr>,
    do_parse!(
        char!('$') >>
        char!('(') >>
        var_name: id >>
        char!(')') >>
        (Expr::GVar(var_name))
    )
);

named!(gvar_special<&str, Expr>,
    do_parse!(
        tag!("$?") >>
        (Expr::GVar("?".to_owned()))
    )
);

named!(is_gvar_empty<&str, Expr>,
    do_parse!(
        tag!("is_gvar_empty") >>
        multispace0 >>
        char!('(') >>
        var_name: id >>
        char!(')') >>
        (Expr::IsGVarEmpty(var_name))
    )
);

named!(current_time<&str, Expr>,
    do_parse!(
        tag!("current_time") >>
        multispace0 >>
        char!('(') >>
        multispace0 >>
        char!(')') >>
        (Expr::CurrentTime)
    )
);

named!(duration_hours<&str, Expr>,
    do_parse!(
        tag!("duration_hours") >>
        multispace0 >>
        char!('(') >>
        a: expr >>
        char!(',') >>
        b: expr >>
        char!(')') >>
        (Expr::DurationHour(Box::new(a), Box::new(b)))
    )
);

named!(has_item<&str, Expr>,
    do_parse!(
        tag!("has_item") >>
        multispace0 >>
        s: delimited!(char!('('), ws!(id), char!(')')) >>
        (Expr::HasItem(s))
    )
);

named!(factor<&str, Expr>,
    ws!(alt!(
        complete!(true_literal) |
        complete!(false_literal) |
        complete!(integer) |
        complete!(gvar) |
        complete!(gvar_special) |
        complete!(is_gvar_empty) |
        complete!(current_time) |
        complete!(duration_hours) |
        complete!(has_item) |
        complete!(parens)
    ))
);

named!(parens<&str, Expr>,
    delimited!(
        char!('('),
        ws!(expr),
        char!(')')
    )
);

// Operator precedence is the same as C.
// term_mul > term_plus > term_ord > term_eq > term_and > expr

named!(term_mul<&str, Expr>,
    ws!(do_parse!(
        init: factor >>
        res: fold_many0!(
            pair!(alt!(complete!(char!('*')) | complete!(char!('/'))), factor),
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

named!(term_plus<&str, Expr>,
    ws!(do_parse!(
        init: term_mul >>
        res: fold_many0!(
            pair!(alt!(complete!(char!('+')) | complete!(char!('-'))), term_mul),
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

named!(term_ord<&str, Expr>,
    ws!(do_parse!(
        init: term_plus >>
        res: fold_many0!(
            pair!(alt!(
                complete!(tag!("<=")) |
                complete!(tag!(">=")) |
                complete!(tag!("<")) |
                complete!(tag!(">"))), term_plus),
            init,
            |a: Expr, (op, e): (&str, Expr)| {
                let op = match op {
                    "<" => Operator::Less,
                    "<=" => Operator::LessEq,
                    ">" => Operator::Greater,
                    ">=" => Operator::GreaterEq,
                    _ => unreachable!(),
                };
                a.join(op, e)
            }
        ) >>
        (res)
    ))
);

named!(term_eq<&str, Expr>,
    ws!(do_parse!(
        init: term_ord >>
        res: fold_many0!(
            pair!(alt!(complete!(tag!("==")) | complete!(tag!("!="))), term_ord),
            init,
            |a: Expr, (op, e): (&str, Expr)| {
                let op = match op {
                    "==" => Operator::Eq,
                    "!=" => Operator::NotEq,
                    _ => unreachable!(),
                };
                a.join(op, e)
            }
        ) >>
        (res)
    ))
);

named!(term_and<&str, Expr>,
    ws!(do_parse!(
        init: term_eq >>
        res: fold_many0!(
            pair!(complete!(tag!("&&")), term_eq),
            init,
            |a: Expr, (_, e): (&str, Expr)| {
                a.join(Operator::And, e)
            }
        ) >>
        (res)
    ))
);

named!(pub expr<&str, Expr>,
    ws!(do_parse!(
        init: term_and >>
        res: fold_many0!(
            pair!(complete!(tag!("||")), term_and),
            init,
            |a: Expr, (_, e): (&str, Expr)| {
                a.join(Operator::Or, e)
            }
        ) >>
        (res)
    ))
);

#[test]
fn expr_test() {
    assert_eq!(
        expr("true"),
        Ok(("", Expr::Value(Value::Bool(true))))
    );
    assert_eq!(
        expr("false"),
        Ok(("", Expr::Value(Value::Bool(false))))
    );
    assert_eq!(
        expr("1234"),
        Ok(("", Expr::Value(Value::Int(1234))))
    );
    assert_eq!(
        expr("$(aa)"),
        Ok(("", Expr::GVar("aa".to_owned())))
    );
    assert_eq!(
        expr("is_gvar_empty(bb)"),
        Ok(("", Expr::IsGVarEmpty("bb".to_owned())))
    );
    let a = Expr::HasItem("box".to_owned());
    assert_eq!(expr("has_item(box)"), Ok(("", a)));
    assert_eq!(
        expr("1 * 2 + 3"),
        Ok((
            "",
            Expr::Term(vec![
                (Operator::None, Expr::Value(Value::Int(1))),
                (Operator::Mul, Expr::Value(Value::Int(2))),
                (Operator::Add, Expr::Value(Value::Int(3)))
            ])
        ))
    );
    assert_eq!(
        expr("1 + 2 * 3"),
        Ok((
            "",
            Expr::Term(vec![
                (Operator::None, Expr::Value(Value::Int(1))),
                (
                    Operator::Add,
                    Expr::Term(vec![
                        (Operator::None, Expr::Value(Value::Int(2))),
                        (Operator::Mul, Expr::Value(Value::Int(3))),
                    ])
                )
            ])
        ))
    );
    assert_eq!(
        expr("3 == 1 + 2"),
        Ok((
            "",
            Expr::Term(vec![
                (Operator::None, Expr::Value(Value::Int(3))),
                (
                    Operator::Eq,
                    Expr::Term(vec![
                        (Operator::None, Expr::Value(Value::Int(1))),
                        (Operator::Add, Expr::Value(Value::Int(2))),
                    ])
                ),
            ])
        ))
    );
    let term_ne_example = Expr::Term(vec![
        (Operator::None, Expr::Value(Value::Int(1))),
        (Operator::NotEq, Expr::Value(Value::Int(2))),
    ]);
    assert_eq!(
        expr("1 != 2"),
        Ok(("", term_ne_example))
    );
    let term_ord_example = Expr::Term(vec![
        (Operator::None, Expr::Value(Value::Int(1))),
        (Operator::LessEq, Expr::Value(Value::Int(2))),
    ]);
    assert_eq!(
        expr("1 <= 2"),
        Ok(("", term_ord_example.clone()))
    );
    assert_eq!(
        expr("1 != 2 || 1 <= 2"),
        Ok((
            "",
            Expr::Term(vec![
                (Operator::None, Expr::Value(Value::Int(1))),
                (Operator::NotEq, Expr::Value(Value::Int(2))),
                (Operator::Or, term_ord_example),
            ])
        ))
    );
}
