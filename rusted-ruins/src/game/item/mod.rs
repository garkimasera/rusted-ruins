
pub mod filter;
pub mod gen;

use common::gamedata::GameData;
use common::gamedata::chara::CharaId;
use common::gamedata::item::*;
use common::gobj;
use game::extrait::*;

/// Additional Item methods
pub trait ItemEx {
    fn get_name(&self) -> String;
}

impl ItemEx for Item {
    fn get_name(&self) -> String {
        ::text::obj_txt(gobj::idx_to_id(self.idx)).to_owned()
    }
}

/// Change specified character's equipment by given item
pub fn change_equipment(
    gd: &mut GameData, cid: CharaId, slot: (EquipSlotKind, u8), il: ItemLocation) -> bool {
    
    if !gd.get_item_list(il.0).has_empty() {
        return false;
    }
    let item = gd.remove_item_and_get(il, 1);
    let item_name = item.get_name();
    if let Some(removed_equipment) = gd.get_equip_list_mut(cid).equip(slot.0, slot.1 as usize, item) {
        gd.get_item_list_mut(il.0).append(removed_equipment, 1);
    }
    game_log_i!("item-equip"; chara=gd.chara.get(cid).get_name(), item=item_name);
    true
}

