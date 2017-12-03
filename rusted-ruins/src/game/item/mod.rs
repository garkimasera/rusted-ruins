
pub mod gen;

use common::gamedata::{self, GameData};
use common::gamedata::chara::CharaId;
use common::gamedata::item::*;

/// Change specified character's equipment by given item
pub fn change_equipment(gd: &mut GameData, cid: CharaId, slot: (ItemKind, u8), il: ItemLocation) -> bool {
    if !gd.get_item_list(il.0).has_empty() {
        return false;
    }
    let item = gd.remove_item_and_get(il, 1);
    if let Some(removed_equipment) = gd.get_equip_list_mut(cid).equip(slot.0, slot.1 as usize, item) {
        gd.get_item_list_mut(il.0).append(removed_equipment, 1);
    }
    true
}

