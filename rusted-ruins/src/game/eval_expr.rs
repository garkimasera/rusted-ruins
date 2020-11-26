use common::gamedata::*;
use common::gobj;
use common::script::{Expr, ExprErrorKind, Operator, Value};

pub trait EvalExpr {
    fn eval(&self, gd: &GameData) -> Value;
}

impl EvalExpr for Expr {
    fn eval(&self, gd: &GameData) -> Value {
        match self {
            Expr::Value(value) => value.clone(),
            Expr::Term(v) => {
                let mut a = v[0].1.eval(gd);
                assert_eq!(v[0].0, Operator::None);

                for (o, b) in v.iter().skip(1) {
                    let b = b.eval(gd);
                    a = binary_operation(*o, a, b);
                }
                a
            }
            Expr::GVar(var_name) => {
                if let Some(v) = gd.vars.global_var(var_name) {
                    v.clone()
                } else {
                    Value::RefUnknownVar
                }
            }
            Expr::IsGVarEmpty(var_name) => Value::Bool(gd.vars.global_var(var_name).is_none()),
            Expr::CurrentTime => Value::Time(gd.time.current_time()),
            Expr::DurationHour(a, b) => match (a.eval(gd), b.eval(gd)) {
                (Value::Time(a), Value::Time(b)) => Value::Int(a.duration_from(b).as_hours()),
                _ => Value::Error(ExprErrorKind::InvalidType),
            },
            Expr::HasItem(item_id) => {
                if let Some(idx) = gobj::id_to_idx_checked(item_id) {
                    let item_list = gd.get_item_list(ItemListLocation::Chara {
                        cid: CharaId::Player,
                    });
                    for (item, _) in item_list.iter() {
                        if item.idx == idx {
                            return Value::Bool(true);
                        }
                    }
                    Value::Bool(false)
                } else {
                    warn!("script error: unknown id {}", item_id);
                    Value::Error(ExprErrorKind::UnknownIdRef)
                }
            }
        }
    }
}

fn binary_operation(o: Operator, a: Value, b: Value) -> Value {
    use self::Value::*;

    // Unwrap RefUnknownVar, cast type if enable, and early return if a or b are Error.
    let (a, b) = match (a, b) {
        (Bool(a), Bool(b)) => (Bool(a), Bool(b)),
        (Int(a), Int(b)) => (Int(a), Int(b)),
        (Bool(a), RefUnknownVar) => (Bool(a), Bool(false)),
        (RefUnknownVar, Bool(b)) => (Bool(false), Bool(b)),
        (Int(a), RefUnknownVar) => (Int(a), Int(0)),
        (RefUnknownVar, Int(b)) => (Int(0), Int(b)),
        (Error(e), _) => {
            return Error(e);
        }
        (_, Error(e)) => {
            return Error(e);
        }
        _ => {
            return Error(ExprErrorKind::InvalidType);
        }
    };

    match o {
        Operator::Or => match (a, b) {
            (Bool(true), _) => Bool(true),
            (Bool(a), Bool(b)) => Bool(a || b),
            _ => Error(ExprErrorKind::InvalidType),
        },
        Operator::And => match (a, b) {
            (Bool(false), _) => Bool(false),
            (Bool(a), Bool(b)) => Bool(a && b),
            _ => Error(ExprErrorKind::InvalidType),
        },
        Operator::Eq => Bool(a == b),
        Operator::NotEq => Bool(a != b),
        Operator::Less => Bool(a < b),
        Operator::LessEq => Bool(a <= b),
        Operator::Greater => Bool(a > b),
        Operator::GreaterEq => Bool(a >= b),
        Operator::Add => match (a, b) {
            (Int(a), Int(b)) => Int(a + b),
            _ => Error(ExprErrorKind::InvalidType),
        },
        Operator::Sub => match (a, b) {
            (Int(a), Int(b)) => Int(a - b),
            _ => Error(ExprErrorKind::InvalidType),
        },
        Operator::Mul => match (a, b) {
            (Int(a), Int(b)) => Int(a * b),
            _ => Error(ExprErrorKind::InvalidType),
        },
        Operator::Div => match (a, b) {
            (Int(a), Int(b)) => Int(a / b),
            _ => Error(ExprErrorKind::InvalidType),
        },
        Operator::None => Error(ExprErrorKind::Other),
    }
}

#[test]
fn binary_operation_test() {
    use self::Value::*;
    assert_eq!(
        binary_operation(Operator::Or, Bool(true), Bool(false)),
        Bool(true)
    );
    assert_eq!(
        binary_operation(Operator::And, Bool(false), Bool(true)),
        Bool(false)
    );
    assert_eq!(binary_operation(Operator::Eq, Int(10), Int(10)), Bool(true));
    assert_eq!(binary_operation(Operator::Eq, Int(2), Int(3)), Bool(false));
    assert_eq!(
        binary_operation(Operator::Less, Int(10), Int(20)),
        Bool(true)
    );
    assert_eq!(
        binary_operation(Operator::Greater, Int(10), Int(20)),
        Bool(false)
    );
    assert_eq!(binary_operation(Operator::Add, Int(20), Int(22)), Int(42));
    assert_eq!(binary_operation(Operator::Sub, Int(20), Int(22)), Int(-2));
    assert_eq!(binary_operation(Operator::Mul, Int(11), Int(12)), Int(132));
    assert_eq!(binary_operation(Operator::Div, Int(150), Int(25)), Int(6));
}
