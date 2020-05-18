use common::gamedata::*;
use common::gobj;

/// Used for creating filtered list and saving filtering state
#[derive(Clone, Copy, Debug)]
pub struct ItemFilter {
    pub all: bool,
    pub equip_slot_kind: Option<EquipSlotKind>,
    pub flags: ItemFlags,
    pub kind_rough: Option<ItemKindRough>,
    pub eatable: bool,
    pub drinkable: bool,
    pub usable: bool,
}

impl ItemFilter {
    pub fn new() -> ItemFilter {
        ItemFilter::default()
    }

    pub fn all() -> ItemFilter {
        let mut filter = ItemFilter::default();
        filter.all = true;
        filter
    }

    /// Given item will be filtered (false) or not (true)
    pub fn judge(&self, item: &Item) -> bool {
        if self.all {
            return true;
        }
        let o = gobj::get_obj(item.idx);

        if let Some(equip_slot_kind) = self.equip_slot_kind {
            if o.kind.equip_slot_kind() != Some(equip_slot_kind) {
                return false;
            }
        }

        if !item.flags.contains(self.flags) {
            return false;
        }

        if let Some(kind_rough) = self.kind_rough {
            if o.kind.rough() != kind_rough {
                return false;
            }
        }

        if self.eatable && o.kind != ItemKind::Food {
            return false;
        }

        if self.drinkable && o.kind != ItemKind::Potion {
            return false;
        }

        if self.usable && o.use_effect == UseEffect::None {
            return false;
        }

        true
    }

    pub fn equip_slot_kind(mut self, equip_slot_kind: EquipSlotKind) -> ItemFilter {
        self.equip_slot_kind = Some(equip_slot_kind);
        self
    }

    // pub fn flags(mut self, flags: ItemFlags) -> ItemFilter {
    //     self.flags = flags;
    //     self
    // }

    pub fn kind_rough(mut self, kind_rough: ItemKindRough) -> ItemFilter {
        self.kind_rough = Some(kind_rough);
        self
    }

    pub fn eatable(mut self, eatable: bool) -> ItemFilter {
        self.eatable = eatable;
        self
    }

    pub fn drinkable(mut self, drinkable: bool) -> ItemFilter {
        self.drinkable = drinkable;
        self
    }

    pub fn usable(mut self, usable: bool) -> ItemFilter {
        self.usable = usable;
        self
    }
}

impl Default for ItemFilter {
    fn default() -> ItemFilter {
        ItemFilter {
            all: false,
            equip_slot_kind: None,
            flags: ItemFlags::empty(),
            kind_rough: None,
            eatable: false,
            drinkable: false,
            usable: false,
        }
    }
}

#[derive(Clone)]
pub struct FilteredItemList<'a> {
    item_list: &'a ItemList,
    location: ItemListLocation,
    filter: ItemFilter,
    count: usize,
}

impl<'a> FilteredItemList<'a> {
    pub fn new(
        item_list: &'a ItemList,
        location: ItemListLocation,
        filter: ItemFilter,
    ) -> FilteredItemList<'a> {
        FilteredItemList {
            item_list,
            location,
            filter,
            count: 0,
        }
    }

    // pub fn all(item_list: &'a ItemList, location: ItemListLocation) -> FilteredItemList<'a> {
    //     FilteredItemList {
    //         item_list,
    //         location,
    //         filter: ItemFilter::all(),
    //         count: 0,
    //     }
    // }
}

impl<'a> Iterator for FilteredItemList<'a> {
    type Item = (ItemLocation, &'a Item, u32);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.item_list.items.len() <= self.count {
                return None;
            }
            let a = &self.item_list.items[self.count];

            let prev_count = self.count;
            self.count += 1;

            if self.filter.judge(&a.0) {
                return Some(((self.location, prev_count as u32), &a.0, a.1));
            }
        }
    }
}

pub trait FilteredListHolder {
    fn get_filtered_item_list(
        &self,
        list_location: ItemListLocation,
        filter: ItemFilter,
    ) -> FilteredItemList;
}

impl FilteredListHolder for GameData {
    fn get_filtered_item_list(
        &self,
        list_location: ItemListLocation,
        filter: ItemFilter,
    ) -> FilteredItemList {
        let item_list = self.get_item_list(list_location);
        FilteredItemList::new(item_list, list_location, filter)
    }
}
