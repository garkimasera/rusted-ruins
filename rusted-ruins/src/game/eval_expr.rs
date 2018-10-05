
use common::gamedata::*;
use common::gobj;
use common::script::{Expr, Value};

pub trait EvalExpr {
    fn eval(&self, gd: &GameData) -> Value;
}

impl EvalExpr for Expr {
    fn eval(&self, gd: &GameData) -> Value {
        match self {
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

