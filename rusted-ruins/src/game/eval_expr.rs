
use common::gamedata::*;
use common::gobj;
use common::script::{Expr, Value, Operator, ExprErrorKind};

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
                    a = binary_operation(*o, a, &b, gd);
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
            Expr::HasItem(item_id) => {
                if let Some(idx) = gobj::id_to_idx_checked(item_id) {
                    let item_list = gd.get_item_list(ItemListLocation::Chara { cid: CharaId::Player });
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

fn binary_operation(o: Operator, a: Value, b: &Expr, gd: &GameData) -> Value {
    use self::Value::*;
    
    let b = b.eval(gd);

    match o {
        Operator::Add => {
            match (a, b) {
                (Int(a), Int(b)) => Int(a + b),
                (Int(a), RefUnknownVar) => Int(a),
                (RefUnknownVar, Int(b)) => Int(b),
                _ => Error(ExprErrorKind::InvalidType),
            }
        }
        Operator::None => Error(ExprErrorKind::Other),
    }
}

