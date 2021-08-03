pub mod filter;
pub mod gen;
pub mod info;
pub mod merged;
pub mod throw;
pub mod time;

use crate::context::IconIdx;
use crate::game::extrait::MapExt;
use common::gamedata::*;
use common::gobj;
use common::obj::ImgVariationRule;
use common::objholder::ItemIdx;
use geom::Vec2d;
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
            ImgVariationRule::RandomOnGen => find_attr!(self, ItemAttr::ImageVariation(n) => n)
                .copied()
                .unwrap_or(0),
            ImgVariationRule::Growing => {
                let remaining = if let Some(time) = self.time.as_ref() {
                    time.remaining
                } else {
                    return 0;
                };
                let growing_time = if let Some(growing_time_hours) = find_attr!(obj, ItemObjAttr::Plant { growing_time_hours, .. } => growing_time_hours)
                {
                    *growing_time_hours as u64 * common::gamedata::time::SECS_PER_HOUR
                } else {
                    return 0;
                };

                if remaining.is_zero() || growing_time == 0 {
                    obj.img.n_pattern - 1
                } else if remaining.as_secs() >= growing_time {
                    0
                } else {
                    let passed_time = growing_time - remaining.as_secs();
                    ((obj.img.n_pattern as u64 - 1) * passed_time / growing_time) as u32
                }
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

    /// Calculate factor for the item power
    fn power_factor(&self) -> f32 {
        let mut factor = 1.0;
        if let Some((_, material)) = self.material() {
            factor *= material.eff;
        }
        factor
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

    fn defence(&self, element: Element) -> u32 {
        if let Some(defence) = find_attr!(self.obj(), ItemObjAttr::Defence(defence)) {
            defence[element].into()
        } else {
            0
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

    /// Update Time attribute. Returns true if remaining reaches zero.
    fn update_time(&mut self) -> bool {
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

            new_remaining == 0
        } else {
            false
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

    /// Append item with some processes
    fn append(&mut self, item: Item, n: u32) {
        self.append_simple(item, n);
    }

    /// Move an item to the other item list
    fn move_to<T: Into<ItemMoveNum>>(&mut self, dest: &mut ItemList, i: usize, n: T) {
        let n = n.into().to_u32(self.items[i].1);
        assert!(self.items[i].1 >= n && n != 0);

        self.items[i].1 -= n;

        let item = if self.items[i].1 == 0 {
            self.items.remove(i).0
        } else {
            self.items[i].0.clone()
        };

        dest.append(item, n);
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

#[extend::ext(pub, name = GameDataItemExt)]
impl GameData {
    /// Add item on specified tile of the current map
    fn add_item_on_tile(&mut self, pos: Vec2d, item: Item, n: u32) {
        let map = self.get_current_map_mut();
        map.locate_item(item, pos, n);
    }

    /// Move item to dest
    fn move_item<T: Into<ItemMoveNum>>(
        &mut self,
        item_location: ItemLocation,
        dest: ItemListLocation,
        n: T,
    ) {
        let (item, n, container_location_and_id) = {
            let container_location_and_id = match item_location.0 {
                ItemListLocation::Container { ill, i } => Some((
                    ill,
                    self.get_item((ill.into(), i))
                        .0
                        .attrs
                        .iter()
                        .filter_map(|attr| match attr {
                            ItemAttr::Container(container) => Some(container.id()),
                            _ => None,
                        })
                        .next()
                        .unwrap(),
                )),
                _ => None,
            };

            let src_list = self.get_item_list_mut(item_location.0);
            let n = match n.into() {
                ItemMoveNum::Partial(n) => n,
                ItemMoveNum::All => src_list.get_number(item_location.1),
            };

            let i = item_location.1 as usize;
            let item = src_list.items[i].0.clone();
            src_list.items[i].1 -= n;

            (item, n, container_location_and_id)
        };

        let dest_list = self.get_item_list_mut(dest);
        dest_list.append(item, n);

        let src_list_location = container_location_and_id
            .map(|(ill, id)| self.find_container_item(ill.into(), id).unwrap())
            .map(ItemListLocation::in_container)
            .unwrap_or(item_location.0);
        let src_list = self.get_item_list_mut(src_list_location);
        src_list.items.retain(|(_, n)| *n > 0);
    }

    /// Find container item that have specified id
    fn find_container_item(&self, ill: ItemListLocation, id: UniqueId) -> Option<ItemLocation> {
        let item_list = self.get_item_list(ill);

        item_list
            .items
            .iter()
            .enumerate()
            .filter_map(|(i, (item, _))| {
                let item_id = item
                    .attrs
                    .iter()
                    .filter_map(|attr| match attr {
                        ItemAttr::Container(container) => Some(container.id()),
                        _ => None,
                    })
                    .next();
                if item_id == Some(id) {
                    Some((ill, i as u32))
                } else {
                    None
                }
            })
            .next()
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
