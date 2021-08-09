use super::merged::*;
use super::ItemExt;
use common::gamedata::*;
use common::gobj;
use common::item_selector::ItemSelector;

/// Used for creating filtered list and saving filtering state
#[derive(Debug, Clone)]
pub struct ItemFilter {
    pub deny_container: bool,
    pub all: bool,
    pub equip_slot_kind: Option<EquipSlotKind>,
    pub selector: Option<ItemSelector>,
    pub flags: ItemFlags,
    pub kind_rough: Option<ItemKindRough>,
    pub eatable: bool,
    pub drinkable: bool,
    pub usable: bool,
    pub readable: bool,
    pub container: bool,
    pub throw_str: Option<u16>,
    pub convertable_by_container: Option<String>,
}

impl ItemFilter {
    pub fn new() -> ItemFilter {
        ItemFilter::default()
    }

    pub fn all() -> ItemFilter {
        ItemFilter {
            all: true,
            ..ItemFilter::default()
        }
    }

    /// Given item will be filtered (false) or not (true)
    pub fn judge(&self, item: &Item) -> bool {
        let o = gobj::get_obj(item.idx);

        if self.deny_container && has_attr!(o, ItemObjAttr::Container) {
            return false;
        }

        if self.all {
            return true;
        }

        if let Some(equip_slot_kind) = self.equip_slot_kind {
            if o.kind.equip_slot_kind() != Some(equip_slot_kind) {
                return false;
            }
        }

        if let Some(selector) = self.selector.as_ref() {
            if !selector.is(o) {
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

        if self.usable && !has_attr!(o, ItemObjAttr::Use) {
            return false;
        }

        if self.readable && item.title().is_none() {
            return false;
        }

        if self.container && !has_attr!(o, ItemObjAttr::Container) {
            return false;
        }

        if let Some(throw_str) = self.throw_str {
            if item.throw_range(throw_str) == 0 {
                return false;
            }
        }

        if let Some(k) = self.convertable_by_container.as_ref() {
            if let Some(kind) =
                find_attr!(o, ItemObjAttr::ConvertableByContainer { kind, .. } => kind)
            {
                if kind != k {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    pub fn deny_container(mut self) -> ItemFilter {
        self.deny_container = true;
        self
    }

    pub fn equip_slot_kind(mut self, equip_slot_kind: EquipSlotKind) -> ItemFilter {
        self.equip_slot_kind = Some(equip_slot_kind);
        self
    }

    pub fn selector(mut self, selector: ItemSelector) -> ItemFilter {
        self.selector = Some(selector);
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

    pub fn readable(mut self, readable: bool) -> ItemFilter {
        self.readable = readable;
        self
    }

    pub fn container(mut self, container: bool) -> ItemFilter {
        self.container = container;
        self
    }

    pub fn throwable(mut self, throw_str: Option<u16>) -> ItemFilter {
        self.throw_str = throw_str;
        self
    }

    pub fn convertable_by_container(mut self, kind: &str) -> ItemFilter {
        self.convertable_by_container = Some(kind.into());
        self
    }
}

impl Default for ItemFilter {
    fn default() -> ItemFilter {
        ItemFilter {
            deny_container: false,
            all: false,
            equip_slot_kind: None,
            selector: None,
            flags: ItemFlags::empty(),
            kind_rough: None,
            eatable: false,
            drinkable: false,
            usable: false,
            readable: false,
            container: false,
            throw_str: None,
            convertable_by_container: None,
        }
    }
}

#[derive(Clone)]
pub struct FilteredItemList<'a> {
    item_list: MergedItemList<'a>,
    filter: ItemFilter,
    count: usize,
}

impl<'a> FilteredItemList<'a> {
    pub fn new(item_list: MergedItemList<'a>, filter: ItemFilter) -> FilteredItemList<'a> {
        FilteredItemList {
            item_list,
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

    fn next(&mut self) -> Option<(ItemLocation, &'a Item, u32)> {
        loop {
            if self.item_list.len() <= self.count {
                return None;
            }
            let a = &self.item_list.get(self.count);

            let il = self.item_list.item_location(self.count);
            self.count += 1;

            if self.filter.judge(&a.0) {
                return Some((il, &a.0, a.1));
            }
        }
    }
}

pub trait FilteredListHolder {
    fn get_filtered_item_list(
        &self,
        list_location: ItemListLocation,
        filter: ItemFilter,
    ) -> FilteredItemList<'_>;

    fn get_merged_filtered_item_list(
        &self,
        list_location0: ItemListLocation,
        list_location1: ItemListLocation,
        filter: ItemFilter,
    ) -> FilteredItemList<'_>;
}

impl FilteredListHolder for GameData {
    fn get_filtered_item_list(
        &self,
        list_location: ItemListLocation,
        filter: ItemFilter,
    ) -> FilteredItemList<'_> {
        let item_list = self.get_merged_item_list(list_location, None);
        FilteredItemList::new(item_list, filter)
    }

    fn get_merged_filtered_item_list(
        &self,
        list_location0: ItemListLocation,
        list_location1: ItemListLocation,
        filter: ItemFilter,
    ) -> FilteredItemList<'_> {
        let item_list = self.get_merged_item_list(list_location0, Some(list_location1));
        FilteredItemList::new(item_list, filter)
    }
}
