pub mod convert_container;
pub mod filter;
pub mod gen;
pub mod info;
pub mod merged;
pub mod throw;
pub mod time;

use crate::context::IconIdx;
use crate::game::extrait::*;
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
                let growing_duration_s = if let Some(&growing_duration) =
                    find_attr!(obj, ItemObjAttr::Plant { growing_duration, .. } => growing_duration)
                {
                    growing_duration.as_secs()
                } else {
                    return 0;
                };

                if remaining.is_zero() || growing_duration_s == 0 {
                    obj.img.n_pattern - 1
                } else if remaining.as_secs() >= growing_duration_s {
                    0
                } else {
                    let passed_time = growing_duration_s - remaining.as_secs();
                    ((obj.img.n_pattern as u64 - 1) * passed_time / growing_duration_s) as u32
                }
            }
            _ => 0,
        }
    }

    fn material(&self) -> Option<(MaterialName, &Material)> {
        for attr in &self.attrs {
            if let ItemAttr::Material(material_name) = attr {
                return Some((*material_name, RULES.materials.get(material_name)));
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

        let mut w = item_obj.w as f32;

        if let Some((_, material)) = self.material() {
            w *= material.w;
        }

        if let Some(container) = find_attr!(self, ItemAttr::Container(container)) {
            let item_weight_in_container: u32 = container
                .item_list()
                .items
                .iter()
                .map(|(item, n)| item.w() * n)
                .sum();
            w += item_weight_in_container as f32;
        }

        w as u32
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
    fn update_time(&mut self, update_remaining: bool) -> bool {
        crate::game::time::current_time();
        if let Some(&mut ItemTime {
            ref mut last_updated,
            ref mut remaining,
        }) = self.time.as_mut()
        {
            let current_time = crate::game::time::current_time();
            let since_last_updated = current_time.duration_from(*last_updated);
            *last_updated = current_time;

            if !update_remaining {
                return false;
            }

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
        self.iter().map(|(item, n)| item.w() * n).sum()
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
        let n = match n.into() {
            ItemMoveNum::Partial(n) => n,
            ItemMoveNum::All => self.get_item(item_location).1,
        };
        let item = &self.get_item(item_location).0;
        if !self.item_appendable(dest, item, n) {
            game_log!("item-container-capacity-limit");
            return;
        }

        let (item, n, container_location_and_id) = {
            let container_location_and_id = match item_location.0 {
                ItemListLocation::Container { ill, i } => Some((
                    ill,
                    find_attr!(self.get_item((ill.into(), i)).0,
                               ItemAttr::Container(container) => container)
                    .map(|container| container.id())
                    .unwrap(),
                )),
                _ => None,
            };

            let src_list = self.get_item_list_mut(item_location.0);

            let i = item_location.1 as usize;
            let item = src_list.items[i].0.clone();
            src_list.items[i].1 -= n;

            (item, n, container_location_and_id)
        };

        self.append_item_to(dest, item, n);

        let src_list_location = container_location_and_id
            .map(|(ill, id)| self.find_container_item(ill.into(), id).unwrap())
            .map(ItemListLocation::in_container)
            .unwrap_or(item_location.0);
        let src_list = self.get_item_list_mut(src_list_location);
        src_list.items.retain(|(_, n)| *n > 0);

        after_src_item_list(self, src_list_location);
    }

    fn append_item_to(&mut self, ill: ItemListLocation, item: Item, n: u32) {
        let mut update_delivery_chest_needed = false;

        if let Some(container_il) = ill.container_item_location() {
            let container_item = &mut self.get_item_mut(container_il).0;

            if let Some(ItemObjAttr::Container { function, .. }) =
                find_attr!(container_item.obj(), ItemObjAttr::Container)
            {
                match function {
                    ContainerFunction::DeliveryChest { .. } => {
                        update_delivery_chest_needed = true;
                    }
                    ContainerFunction::Converter { .. } => {
                        self::convert_container::append_to_converter(container_item, item, n);
                        return;
                    }
                    ContainerFunction::ConvertMixed { .. } => {
                        self::convert_container::append_to_mixed_converter(container_item, item, n);
                        return;
                    }
                    _ => (),
                }
            }
        }

        let item_list = self.get_item_list_mut(ill);
        item_list.append(item, n);

        if update_delivery_chest_needed {
            crate::game::quest::update_delivery_chest(self, ill);
        }
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

    /// Weight capacity of an item list
    fn item_list_capacity(&self, ill: ItemListLocation) -> Option<u32> {
        match ill {
            ItemListLocation::Container { ill, i } => {
                let container_item = &self.get_item((ill.into(), i)).0;

                find_attr!(container_item.obj(), ItemObjAttr::Container { capacity, .. } => capacity).copied()
            }
            _ => None,
        }
    }

    /// Can append item to this item list or not
    fn item_appendable(&self, ill: ItemListLocation, item: &Item, n: u32) -> bool {
        let w = item.w() * n;
        if let Some(capacity) = self.item_list_capacity(ill) {
            self.get_item_list(ill).sum_weight() + w * n <= capacity
        } else {
            true
        }
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

    game_log!("item-equip"; chara=gd.chara.get(cid), item=item);
    if let Some(removed_equipment) = gd
        .get_equip_list_mut(cid)
        .equip(slot.0, slot.1 as usize, item)
    {
        gd.get_item_list_mut(il.0).append(removed_equipment, 1);
    }

    gd.chara.get_mut(cid).update();
}

pub fn remove_equipment(gd: &mut GameData, cid: CharaId, slot: (EquipSlotKind, u8)) {
    if let Some(removed_equipment) = gd.get_equip_list_mut(cid).remove(slot.0, slot.1 as usize) {
        gd.get_item_list_mut(ItemListLocation::Chara { cid })
            .append(removed_equipment, 1);
        gd.chara.get_mut(cid).update();
    }
}

fn after_src_item_list(gd: &mut GameData, ill: ItemListLocation) {
    if let ItemListLocation::Container { ill: ill0, i } = ill {
        let container_item = &mut gd.get_item((ill0.into(), i)).0;

        if let Some(ItemObjAttr::Container {
            function: ContainerFunction::DeliveryChest,
            ..
        }) = find_attr!(container_item.obj(), ItemObjAttr::Container)
        {
            crate::game::quest::update_delivery_chest(gd, ill);
        }
    }
}
