pub mod filter;
pub mod gen;
pub mod info;

use common::gamedata::*;
use common::gobj;
use common::objholder::ItemIdx;

/// Additional Item methods
pub trait ItemEx {
    /// Calculate item price
    fn price(&self) -> i64;
    /// Calculate item selling price
    fn selling_price(&self) -> i64;
    fn w(&self) -> u32;
    fn charge(&self) -> Option<u32>;
    fn charge_mut(&mut self) -> Option<&mut u32>;
}

impl ItemEx for Item {
    fn price(&self) -> i64 {
        let item_obj = gobj::get_obj(self.idx);

        item_obj.basic_price as i64
    }

    fn selling_price(&self) -> i64 {
        self.price() / 2
    }

    fn w(&self) -> u32 {
        let item_obj = gobj::get_obj(self.idx);

        item_obj.w
    }

    fn charge(&self) -> Option<u32> {
        for attr in &self.attributes {
            match attr {
                ItemAttribute::Charge { n } => {
                    return Some(*n);
                }
                _ => (),
            }
        }
        None
    }

    fn charge_mut(&mut self) -> Option<&mut u32> {
        for attr in &mut self.attributes {
            match attr {
                ItemAttribute::Charge { n } => {
                    return Some(n);
                }
                _ => (),
            }
        }
        None
    }
}

pub trait ItemListEx {
    /// Return the first item found
    fn find(&self, idx: ItemIdx) -> Option<u32>;
}

impl ItemListEx for ItemList {
    fn find(&self, idx: ItemIdx) -> Option<u32> {
        for (i, (item, _)) in self.iter().enumerate() {
            if item.idx == idx {
                return Some(i as u32);
            }
        }
        None
    }
}

/// Change specified character's equipment by given item
pub fn change_equipment(
    gd: &mut GameData,
    cid: CharaId,
    slot: (EquipSlotKind, u8),
    il: ItemLocation,
) {
    let item = gd.remove_item_and_get(il, 1);

    game_log_i!("item-equip"; chara=gd.chara.get(cid), item=item);
    if let Some(removed_equipment) = gd
        .get_equip_list_mut(cid)
        .equip(slot.0, slot.1 as usize, item)
    {
        gd.get_item_list_mut(il.0).append(removed_equipment, 1);
    }
}
