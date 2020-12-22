pub mod filter;
pub mod gen;
pub mod info;
pub mod merged;
pub mod throw;

use common::gamedata::*;
use common::gobj;
use common::objholder::ItemIdx;
use rules::material::Material;
use rules::RULES;

/// Additional Item methods
pub trait ItemEx {
    fn material(&self) -> Option<(MaterialName, &Material)>;
    fn dice(&self) -> (u16, u16);
    /// Calculate item price
    fn price(&self) -> i64;
    /// Calculate item selling price
    fn selling_price(&self) -> i64;
    fn w(&self) -> u32;
    fn charge(&self) -> Option<u32>;
    fn charge_mut(&mut self) -> Option<&mut u32>;
    fn title(&self) -> Option<&str>;
    /// Calculate throw range by item weight and character STR.
    fn throw_range(&self, str: u16) -> u32;
}

impl ItemEx for Item {
    fn material(&self) -> Option<(MaterialName, &Material)> {
        for attr in &self.attributes {
            match attr {
                ItemAttribute::Material(material_name) => {
                    return Some((*material_name, RULES.material.get(material_name)))
                }
                _ => (),
            }
        }
        None
    }

    fn dice(&self) -> (u16, u16) {
        let item_obj = gobj::get_obj(self.idx);
        if let Some((_, material)) = self.material() {
            let x = item_obj.dice_x as f32 * material.dice;
            (item_obj.dice_n, x as u16)
        } else {
            (item_obj.dice_n, item_obj.dice_x)
        }
    }

    fn price(&self) -> i64 {
        let item_obj = gobj::get_obj(self.idx);
        let mut factor = 1.0;

        if let Some((_, material)) = self.material() {
            factor = material.price;
        }

        (item_obj.basic_price as f32 * factor) as i64
    }

    fn selling_price(&self) -> i64 {
        self.price() / 2
    }

    fn w(&self) -> u32 {
        let item_obj = gobj::get_obj(self.idx);

        if let Some((_, material)) = self.material() {
            (item_obj.w as f32 * material.w) as u32
        } else {
            item_obj.w
        }
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

    fn title(&self) -> Option<&str> {
        for attr in &self.attributes {
            match attr {
                ItemAttribute::Title(title) => {
                    return Some(title);
                }
                _ => (),
            }
        }
        None
    }

    fn throw_range(&self, str: u16) -> u32 {
        let w = std::cmp::max(self.w(), 1);
        std::cmp::min(
            RULES.combat.throw_range_max,
            RULES.combat.throw_range_factor * str as u32 / w,
        )
    }
}

pub trait ItemListEx {
    /// Return the first item found
    fn find(&self, idx: ItemIdx) -> Option<u32>;
    /// Sum of item weight
    fn sum_weight(&self) -> u32;
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

    fn sum_weight(&self) -> u32 {
        self.iter()
            .map(|(item, n)| {
                let obj = item.obj();
                obj.w * n
            })
            .sum()
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
