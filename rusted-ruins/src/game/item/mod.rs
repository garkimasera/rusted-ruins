pub mod filter;
pub mod gen;
pub mod info;
pub mod merged;
pub mod throw;

use crate::context::IconIdx;
use common::gamedata::*;
use common::gobj;
use common::obj::ImgVariationRule;
use common::objholder::ItemIdx;
use rules::material::Material;
use rules::RULES;

/// Additional Item methods
#[extend::ext(pub)]
impl Item {
    fn icon(&self) -> IconIdx {
        IconIdx::Item {
            idx: self.idx,
            i_pattern: self.img_variation(),
        }
    }

    fn img_variation(&self) -> u32 {
        let time = crate::game::time::current_time();
        let obj = self.obj();

        match obj.img.variation_rule {
            ImgVariationRule::RandomOnGen => {
                let i_pattern = if_first! { &ItemAttr::ImageVariation(n) = &self.attrs; {
                    n
                } else {
                    return 0;
                }};
                return i_pattern;
            }
            ImgVariationRule::Growing => {
                let ready_time = if_first! { &ItemAttr::Time(time) = &self.attrs; {
                    time.as_secs()
                } else {
                    return 0;
                }};
                let growing_time = if_first! { &ItemObjAttr::Plant { growing_time_hours, .. }
                                                = &obj.attrs; {
                    growing_time_hours as u64 * common::gamedata::time::SECS_PER_HOUR
                } else {
                    return 0;
                }};

                let current = time.as_secs();

                let i_pattern = if ready_time <= current || growing_time == 0 {
                    obj.img.n_pattern - 1
                } else {
                    ((obj.img.n_pattern as u64) * (ready_time - current) / growing_time) as u32
                };
                return i_pattern;
            }
            _ => 0,
        }
    }

    fn material(&self) -> Option<(MaterialName, &Material)> {
        for attr in &self.attrs {
            match attr {
                ItemAttr::Material(material_name) => {
                    return Some((*material_name, RULES.material.get(material_name)))
                }
                _ => (),
            }
        }
        None
    }

    /// Calculate factor for the item effectiveness
    fn eff_factor(&self) -> f32 {
        let mut factor = 1.0;
        if let Some((_, material)) = self.material() {
            factor *= material.eff;
        }
        factor
    }

    /// Calculate effectiveness for this item
    fn calc_eff(&self) -> i32 {
        let item_obj = gobj::get_obj(self.idx);
        let base_eff = self.calc_eff_without_var();
        let eff_var = (item_obj.eff_var as f32 * self.eff_factor()) as i32;
        let eff_min = std::cmp::max(base_eff - item_obj.eff_var as i32, 0);
        let eff_max = base_eff + eff_var;
        if eff_max > eff_min {
            rng::gen_range(eff_min..eff_max)
        } else {
            eff_min
        }
    }

    /// Calculate effectiveness for this item without variation
    fn calc_eff_without_var(&self) -> i32 {
        let item_obj = gobj::get_obj(self.idx);
        (item_obj.eff as f32 * self.eff_factor()) as i32
    }

    /// Calculate item price
    fn price(&self) -> i64 {
        let item_obj = gobj::get_obj(self.idx);
        let mut factor = 1.0;

        if let Some((_, material)) = self.material() {
            factor = material.price;
        }

        (item_obj.basic_price as f32 * factor) as i64
    }

    /// Calculate item selling price
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
        for attr in &self.attrs {
            match attr {
                ItemAttr::Charge { n } => {
                    return Some(*n);
                }
                _ => (),
            }
        }
        None
    }

    fn charge_mut(&mut self) -> Option<&mut u32> {
        for attr in &mut self.attrs {
            match attr {
                ItemAttr::Charge { n } => {
                    return Some(n);
                }
                _ => (),
            }
        }
        None
    }

    fn title(&self) -> Option<&str> {
        for attr in &self.attrs {
            match attr {
                ItemAttr::Title(title) => {
                    return Some(title);
                }
                _ => (),
            }
        }
        None
    }

    /// Calculate throw range by item weight and character STR.
    fn throw_range(&self, str: u16) -> u32 {
        let w = std::cmp::max(self.w(), 1);
        std::cmp::min(
            RULES.combat.throw_range_max,
            RULES.combat.throw_range_factor * str as u32 / w,
        )
    }
}

#[extend::ext(pub)]
impl ItemList {
    /// Return the first item found
    fn find(&self, idx: ItemIdx) -> Option<u32> {
        for (i, (item, _)) in self.iter().enumerate() {
            if item.idx == idx {
                return Some(i as u32);
            }
        }
        None
    }

    /// Sum of item weight
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
