
use common::gamedata::*;

pub fn sell_item(gd: &mut GameData, il: ItemLocation) {
    gd.remove_item(il, 1);
}

