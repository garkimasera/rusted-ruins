use crate::game::extrait::*;
use common::gamedata::*;

pub fn item_to_throw_effect(gd: &GameData, il: ItemLocation, cid: CharaId) -> Effect {
    let item = gd.get_item(il).0;
    let item_obj = item.obj();
    let range = item.throw_range(gd.chara.get(cid).attr.str);
    let mut effect = if let Some(effect) = item_obj.throw_effect.clone() {
        effect
    } else {
        todo!()
    };
    effect.range = range;
    effect
}
