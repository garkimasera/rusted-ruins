
use common::gamedata::*;

pub fn buy_item(gd: &mut GameData, il: ItemLocation) {
    gd.move_item(il, ItemListLocation::Chara { cid: CharaId::Player }, 1);
}

pub fn sell_item(gd: &mut GameData, il: ItemLocation) {
    gd.remove_item(il, 1);
}

