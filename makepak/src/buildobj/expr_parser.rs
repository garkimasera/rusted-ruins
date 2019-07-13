use common::script::{Expr, Operator, Value};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, multispace0};
use nom::combinator::complete;
use nom::multi::fold_many0;
use nom::sequence::{delimited, pair};
use nom::IResult;

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
    assert_eq!(id("ab.c"), Ok(("", "ab.c".to_string())));
    assert_eq!(id("abc-def "), Ok((" ", "abc-def".to_string())));
}

fn true_literal(input: &str) -> IResult<&str, Expr> {
    let (input, _) = tag("true")(input)?;
    Ok((input, Expr::Value(Value::Bool(true))))
}

fn false_literal(input: &str) -> IResult<&str, Expr> {
    let (input, _) = tag("false")(input)?;
    Ok((input, Expr::Value(Value::Bool(false))))
}

fn integer(input: &str) -> IResult<&str, Expr> {
    let (input, digits) = digit1(input)?;
    Ok((
        input,
        Expr::Value(Value::Int(i32::from_str_radix(&digits, 10).unwrap())),
    ))
}

fn gvar(input: &str) -> IResult<&str, Expr> {
    let (input, _) = char('$')(input)?;
    let (input, _) = char('(')(input)?;
    let (input, var_name) = id(input)?;
    let (input, _) = char(')')(input)?;
    Ok((input, Expr::GVar(var_name)))
}

fn gvar_special(input: &str) -> IResult<&str, Expr> {
    let (input, _) = tag("$?")(input)?;
    Ok((input, Expr::GVar("?".to_owned())))
}

fn is_gvar_empty(input: &str) -> IResult<&str, Expr> {
    let (input, _) = tag("is_gvar_empty")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char('(')(input)?;
    let (input, var_name) = id(input)?;
    let (input, _) = char(')')(input)?;
    Ok((input, Expr::IsGVarEmpty(var_name)))
}

fn current_time(input: &str) -> IResult<&str, Expr> {
    let (input, _) = tag("current_time")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char('(')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char(')')(input)?;
    Ok((input, Expr::CurrentTime))
}

fn duration_hours(input: &str) -> IResult<&str, Expr> {
    let (input, _) = tag("duration_hours")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char('(')(input)?;
    let (input, expr_a) = expr(input)?;
    let (input, _) = char(',')(input)?;
    let (input, expr_b) = expr(input)?;
    let (input, _) = char(')')(input)?;
    Ok((
        input,
        Expr::DurationHour(Box::new(expr_a), Box::new(expr_b)),
    ))
}

fn has_item(input: &str) -> IResult<&str, Expr> {
    let (input, _) = tag("has_item")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, s) = delimited(char('('), id, char(')'))(input)?;
    Ok((input, Expr::HasItem(s)))
}

fn factor(input: &str) -> IResult<&str, Expr> {
    let (input, _) = multispace0(input)?;
    let (input, expr) = alt((
        complete(true_literal),
        complete(false_literal),
        complete(integer),
        complete(gvar),
        complete(gvar_special),
        complete(is_gvar_empty),
        complete(current_time),
        complete(duration_hours),
        complete(has_item),
        complete(parens),
    ))(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, expr))
}

fn parens(input: &str) -> IResult<&str, Expr> {
    let (input, _) = char('(')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, expr) = expr(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char(')')(input)?;
    Ok((input, expr))
}

// Operator precedence is the same as C.
// term_mul > term_plus > term_ord > term_eq > term_and > expr

fn term_mul(input: &str) -> IResult<&str, Expr> {
    let (input, _) = multispace0(input)?;
    let (input, init) = factor(input)?;
    let (input, res) = fold_many0(
        pair(alt((char('*'), char('/'))), factor),
        init,
        |a: Expr, (op, e): (char, Expr)| {
            let op = match op {
                '*' => Operator::Mul,
                '/' => Operator::Div,
                _ => unreachable!(),
            };
            a.join(op, e)
        },
    )(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, res))
}

fn term_plus(input: &str) -> IResult<&str, Expr> {
    let (input, _) = multispace0(input)?;
    let (input, init) = term_mul(input)?;
    let (input, res) = fold_many0(
        pair(alt((char('+'), char('-'))), term_mul),
        init,
        |a: Expr, (op, e): (char, Expr)| {
            let op = match op {
                '+' => Operator::Add,
                '-' => Operator::Sub,
                _ => unreachable!(),
            };
            a.join(op, e)
        },
    )(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, res))
}

fn term_ord(input: &str) -> IResult<&str, Expr> {
    let (input, _) = multispace0(input)?;
    let (input, init) = term_plus(input)?;
    let (input, res) = fold_many0(
        pair(
            alt((
                complete(tag("<=")),
                complete(tag(">=")),
                complete(tag("<")),
                complete(tag(">")),
            )),
            term_plus,
        ),
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
        },
    )(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, res))
}

fn term_eq(input: &str) -> IResult<&str, Expr> {
    let (input, _) = multispace0(input)?;
    let (input, init) = term_ord(input)?;
    let (input, res) = fold_many0(
        pair(alt((tag("=="), tag("!="))), term_ord),
        init,
        |a: Expr, (op, e): (&str, Expr)| {
            let op = match op {
                "==" => Operator::Eq,
                "!=" => Operator::NotEq,
                _ => unreachable!(),
            };
            a.join(op, e)
        },
    )(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, res))
}

fn term_and(input: &str) -> IResult<&str, Expr> {
    let (input, _) = multispace0(input)?;
    let (input, init) = term_eq(input)?;
    let (input, res) = fold_many0(
        pair(tag("&&"), term_eq),
        init,
        |a: Expr, (_, e): (&str, Expr)| a.join(Operator::And, e),
    )(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, res))
}

pub fn expr(input: &str) -> IResult<&str, Expr> {
    let (input, _) = multispace0(input)?;
    let (input, init) = term_and(input)?;
    let (input, res) = fold_many0(
        pair(tag("||"), term_and),
        init,
        |a: Expr, (_, e): (&str, Expr)| a.join(Operator::Or, e),
    )(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, res))
}

#[test]
fn expr_test() {
    assert_eq!(expr("true"), Ok(("", Expr::Value(Value::Bool(true)))));
    assert_eq!(expr("false"), Ok(("", Expr::Value(Value::Bool(false)))));
    assert_eq!(expr("1234"), Ok(("", Expr::Value(Value::Int(1234)))));
    assert_eq!(expr("$(aa)"), Ok(("", Expr::GVar("aa".to_owned()))));
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
    assert_eq!(expr("1 != 2"), Ok(("", term_ne_example)));
    let term_ord_example = Expr::Term(vec![
        (Operator::None, Expr::Value(Value::Int(1))),
        (Operator::LessEq, Expr::Value(Value::Int(2))),
    ]);
    assert_eq!(expr("1 <= 2"), Ok(("", term_ord_example.clone())));
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
