pub mod filter;
pub mod gen;
pub mod info;
pub mod merged;
pub mod throw;
pub mod time;

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
        let obj = self.obj();

        match obj.img.variation_rule {
            ImgVariationRule::RandomOnGen => {
                if let Some(&ItemAttr::ImageVariation(n)) = self
                    .attrs
                    .iter()
                    .find(|attr| matches!(attr, ItemAttr::ImageVariation(..)))
                {
                    n
                } else {
                    0
                }
            }
            ImgVariationRule::Growing => {
                let remaining = if let Some(time) = self.time.as_ref() {
                    time.remaining
                } else {
                    return 0;
                };
                let growing_time = if let Some(&ItemObjAttr::Plant {
                    growing_time_hours, ..
                }) = obj
                    .attrs
                    .iter()
                    .find(|attr| matches!(attr, ItemObjAttr::Plant { .. }))
                {
                    growing_time_hours as u64 * common::gamedata::time::SECS_PER_HOUR
                } else {
                    return 0;
                };

                let i_pattern = if remaining.is_zero() || growing_time == 0 {
                    obj.img.n_pattern - 1
                } else if remaining.as_secs() >= growing_time {
                    0
                } else {
                    let passed_time = growing_time - remaining.as_secs();
                    ((obj.img.n_pattern as u64 - 1) * passed_time / growing_time) as u32
                };
                i_pattern
            }
            _ => 0,
        }
    }

    fn material(&self) -> Option<(MaterialName, &Material)> {
        for attr in &self.attrs {
            if let ItemAttr::Material(material_name) = attr {
                return Some((*material_name, RULES.material.get(material_name)));
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
            if let ItemAttr::Charge { n } = attr {
                return Some(*n);
            }
        }
        None
    }

    fn charge_mut(&mut self) -> Option<&mut u32> {
        for attr in &mut self.attrs {
            if let ItemAttr::Charge { n } = attr {
                return Some(n);
            }
        }
        None
    }

    fn title(&self) -> Option<&str> {
        for attr in &self.attrs {
            if let ItemAttr::Title(title) = attr {
                return Some(title);
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

    /// Compare Time attribute and current time. Returns Some(true) if current time >= Time attr.
    fn remaining(&self) -> Option<Duration> {
        self.time.map(|time| time.remaining)
    }

    /// Update Time attribute
    fn update_time(&mut self) {
        crate::game::time::current_time();
        if let Some(&mut ItemTime {
            ref mut last_updated,
            ref mut remaining,
        }) = self.time.as_mut()
        {
            let current_time = crate::game::time::current_time();
            let since_last_updated = current_time.duration_from(*last_updated);
            *last_updated = current_time;
            let new_remaining = remaining
                .as_secs()
                .saturating_sub(since_last_updated.as_secs());
            *remaining = Duration::from_seconds(new_remaining);
        }
    }

    /// Randomize remaining time
    fn randomize_time(&mut self) {
        if let Some(&mut ItemTime {
            ref mut remaining, ..
        }) = self.time.as_mut()
        {
            let new_remaining = rng::gen_range(0..=remaining.as_secs());
            *remaining = Duration::from_seconds(new_remaining);
        }
    }

    /// Reset Time attribute.
    fn reset_time(&mut self, new_remaining: Duration) {
        let current_time = crate::game::time::current_time();
        if let Some(&mut ItemTime {
            ref mut last_updated,
            ref mut remaining,
        }) = self.time.as_mut()
        {
            *last_updated = current_time;
            *remaining = new_remaining;
        } else {
            self.time = Some(ItemTime {
                last_updated: current_time,
                remaining: new_remaining,
            });
        }
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
