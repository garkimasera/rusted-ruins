
use common::gamedata::*;
use common::gobj;
use common::script::{Expr, Value};

pub trait EvalExpr {
    fn eval(&self, gd: &GameData) -> Value;
}

impl EvalExpr for Expr {
    fn eval(&self, gd: &GameData) -> Value {
        match self {
            Expr::Value(value) => value.clone(),
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
                    Value::Error
                }
            }
        }
    }
}

