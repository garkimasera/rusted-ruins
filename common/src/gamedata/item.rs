
use array2d::Vec2d;
use objholder::ItemIdx;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
/// Game item
pub struct Item {
    pub idx: ItemIdx,
    pub content: ItemContent,
}

impl Item {
    pub fn kind(&self) -> ItemKind {
        self.content.kind()
    }
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum ItemKind {
    Object, Potion, Weapon, BodyArmor,
}

/// Kind dependent data for a item
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum ItemContent {
    Object,
    Potion {
        kind: PotionKind,
        /// Effectiveness of this item
        eff: f32,
    },
    Weapon {
        kind: WeaponKind,
        eff: f32,
    },
}

impl ItemContent {
    pub fn kind(&self) -> ItemKind {
        match *self {
            ItemContent::Object => ItemKind::Object,
            ItemContent::Potion { .. } => ItemKind::Potion,
            ItemContent::Weapon { .. } => ItemKind::Weapon,
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum PotionKind {
    Heal,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum WeaponKind {
    Sword, Spear, Axe, Whip,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ItemListLocation {
    OnMap { mid: super::map::MapId, pos: Vec2d },
    Chara { cid: super::chara::CharaId },
    Equip { cid: super::chara::CharaId },
}

pub type ItemLocation = (ItemListLocation, u32);

/// Item list that records all items owned by one character or one tile
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemList {
    pub limit: usize,
    pub items: Vec<(Box<Item>, u32)>,
}

impl ItemList {
    pub fn new(limit: usize) -> ItemList {
        ItemList {
            limit, items: Vec::new(),
        }
    }    

    pub fn for_chara() -> ItemList {
        Self::new(::basic::MAX_ITEM_CHARA)
    }

    /// Inventory for player has larger size
    pub fn for_player() -> ItemList {
        Self::new(::basic::MAX_ITEM_PLAYER)
    }

    /// Get the number of item
    pub fn get_number(&self, i: u32) -> u32 {
        self.items[i as usize].1
    }

    /// This list is empty or not
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// This list has empty slot or not
    pub fn has_empty(&self) -> bool {
        self.limit > self.items.len() + 1
    }

    /// Append item
    /// If the list doesn't have empty, returns given item.
    pub fn append(&mut self, item: Box<Item>, n: u32) -> Option<Box<Item>> {
        if self.limit <= self.items.len() + 1 {
            return Some(item)
        }

        for i in self.items.iter_mut() {
            if *i.0 == *item {
                i.1 += n;
                return None;
            }
        }
        
        self.items.push((item, n));
        None
    }

    /// Remove an item from list
    pub fn remove<T: Into<ItemMoveNum>>(&mut self, i: u32, n: T) {
        let i = i as usize;
        let n = n.into().to_u32(self.items[i].1);
        assert!(self.items[i].1 <= n && n != 0);
        if n == 0 { return; }

        self.items[i].1 -= n;
        if self.items[i].1 == 0 {
            self.items.remove(i);
        }
    }

    /// Remove an item from list and get its clone or moved value
    pub fn remove_and_get<T: Into<ItemMoveNum>>(&mut self, i: u32, n: T) -> Box<Item> {
        let i = i as usize;
        let n = n.into().to_u32(self.items[i].1);
        assert!(self.items[i].1 <= n && n != 0);

        self.items[i].1 -= n;
        if self.items[i].1 == 0 {
            self.items.remove(i).0
        } else {
            self.items[i].0.clone()
        }
    }

    /// Move an item to the other item list
    pub fn move_to<T: Into<ItemMoveNum>>(&mut self, dest: &mut ItemList, i: usize, n: T) -> bool {
        let n = n.into().to_u32(self.items[i].1);
        assert!(self.items[i].1 <= n && n != 0);
        if !dest.has_empty() { return false; }
        
        self.items[i].1 -= n;

        let item = if self.items[i].1 == 0 {
            self.items.remove(i).0
        } else {
            self.items[i].0.clone()
        };

        dest.append(item, n);
        true
    }

    /// Return item iterator
    pub fn iter(&self) -> ::std::slice::Iter<(Box<Item>, u32)> {
        self.items.iter()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ItemMoveNum {
    All, Partial(u32),
}

impl ItemMoveNum {
    fn to_u32(self, all: u32) -> u32 {
        match self {
            ItemMoveNum::All => all,
            ItemMoveNum::Partial(n) => n,
        }
    }
}

impl From<u32> for ItemMoveNum {
    fn from(n: u32) -> ItemMoveNum {
        ItemMoveNum::Partial(n)
    }
}

/// Used for creating filtered list and saving filtering state
#[derive(Clone, Copy, Debug)]
pub struct ItemFilter {
    pub all: bool,
    pub kind: Option<ItemKind>,
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
        if self.all { return true; }
        
        if let Some(kind) = self.kind {
            if item.content.kind() != kind { return false; }
        }
        
        true
    }

    pub fn kind(mut self, kind: ItemKind) -> ItemFilter {
        self.kind = Some(kind);
        self
    }
}

impl Default for ItemFilter {
    fn default() -> ItemFilter {
        ItemFilter {
            all: false,
            kind: None,
        }
    }
}

pub struct FilteredItemList<'a> {
    item_list: &'a ItemList,
    location: ItemListLocation,
    filter: ItemFilter,
    count: usize,
}

impl<'a> FilteredItemList<'a> {
    pub fn new(item_list: &'a ItemList, location: ItemListLocation,
               filter: ItemFilter) -> FilteredItemList<'a> {
        
        FilteredItemList {
            item_list, location, filter, count: 0,
        }
    }

    pub fn all(item_list: &'a ItemList, location: ItemListLocation) -> FilteredItemList<'a> {
        
        FilteredItemList {
            item_list, location, filter: ItemFilter::all(), count: 0,
        }
    }
}

impl<'a> Iterator for FilteredItemList<'a> {
    type Item = (ItemLocation, &'a Item, u32);
    
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.item_list.items.len() <= self.count {
                return None
            }
            let a = &self.item_list.items[self.count];

            let prev_count = self.count;
            self.count += 1;

            if self.filter.judge(&*a.0) {
                return Some(((self.location, prev_count as u32), &*a.0, a.1));
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct EquipItemList {
    /// Slot infomation
    slots: Vec<SlotInfo>,
    item_list: ItemList,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct SlotInfo {
    /// The kind of equipment
    ik: ItemKind,
    /// The index in this ItemKind
    n: u8,
    /// The Index at list
    list_idx: Option<u8>,
}

impl SlotInfo {
    fn new(ik: ItemKind, n: u8) -> SlotInfo {
        SlotInfo { ik, n, list_idx: None }
    }
}

pub const MAX_SLOT_NUM_PER_KIND: usize = ::basic::MAX_EQUIP_SLOT as usize;

impl EquipItemList {
    pub fn new(slots: &[(ItemKind, u8)]) -> EquipItemList {
        let mut slots = slots.to_vec();
        slots.sort_by_key(|&(ik, _)| ik);
        let mut new_slots = Vec::new();
        for &(ik, n) in slots.iter() {
            for i in 0..n {
                new_slots.push(SlotInfo::new(ik, i));
            }
        }
        
        EquipItemList {
            slots: new_slots,
            item_list: ItemList::new(::basic::MAX_EQUIP_SLOT),
        }
    }

    /// Number of slots for specified ItemKind
    pub fn slot_num(&self, ik: ItemKind) -> usize {
        self.slots.iter().filter(|slot| slot.ik == ik).count()
    }
    
    /// Specified slot is empty or not
    /// If specified slot doesn't exist, return false.
    pub fn is_slot_empty(&self, ik: ItemKind, n: usize) -> bool {
        assert!(n < MAX_SLOT_NUM_PER_KIND);
        if let Some(a) = self.slots.iter().filter(|slot| slot.ik == ik).nth(n) {
            a.list_idx.is_none()
        } else {
            false
        }
    }

    /// Get specified equipped item
    pub fn item(&self, ik: ItemKind, n: usize) -> Option<&Item> {
        assert!(n < MAX_SLOT_NUM_PER_KIND);
        if let Some(a) = self.list_idx(ik, n) {
            Some(&self.item_list.items[a].0)
        } else {
            None
        }
    }
    
    /// Equip an item to specified slot (the nth slot of given ItemKind), and returns removed item
    pub fn equip(&mut self, ik: ItemKind, n: usize, item: Box<Item>) -> Option<Box<Item>> {
        assert!(self.slot_num(ik) > n);
        if let Some(i) = self.list_idx(ik, n) { // Replace existing item
            return Some(::std::mem::replace(&mut self.item_list.items[i].0, item));
        }
        
        if self.item_list.items.is_empty() { // If any item is not equipped.
            self.item_list.items.push((item, 1));
            self.set_list_idx(ik, n, 0);
            return None;
        }
        
        // Calculate new index for insert
        let mut new_idx = 0;
        let mut processed_slot = 0;
        for i_slot in 0..self.slots.len() {
            if self.slots[i_slot].ik == ik && self.slots[i_slot].n as usize == n {
                self.set_list_idx(ik, n, new_idx);
                self.item_list.items.insert(new_idx, (item, 1));
                processed_slot = i_slot;
                break;
            } else if self.slots[i_slot].list_idx.is_some() {
                new_idx += 1;
            }
        }

        for i_slot in (processed_slot + 1)..self.slots.len() {
            if let Some(list_idx) = self.slots[i_slot].list_idx {
                self.slots[i_slot].list_idx = Some(list_idx + 1);
            }
        }
        
        None
    }

    fn list_idx(&self, ik: ItemKind, n: usize) -> Option<usize> {
        if let Some(slot) = self.slots.iter().find(|slot| slot.ik == ik && slot.n as usize == n) {
            if let Some(list_idx) = slot.list_idx {
                Some(list_idx as usize)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn set_list_idx(&mut self, ik: ItemKind, n: usize, idx: usize) {
        if let Some(slot) = self.slots.iter_mut().find(|slot| slot.ik == ik && slot.n as usize == n) {
            slot.list_idx = Some(idx as u8);
        } else {
            panic!("set_list_idx for invalid slot");
        }
    }

    pub fn slot_iter(&self) -> EquipSlotIter {
        EquipSlotIter {
            equip_item_list: &self,
            n: 0,
        }
    }

    pub fn item_iter(&self) -> EquipItemIter {
        EquipItemIter {
            equip_item_list: &self,
            n: 0,
        }
    }

    pub fn list(&self) -> &ItemList {
        &self.item_list
    }
}

pub struct EquipSlotIter<'a> {
    equip_item_list: &'a EquipItemList,
    n: usize,
}

impl<'a> Iterator for EquipSlotIter<'a> {
    type Item = (ItemKind, u8, Option<&'a Item>);
    fn next(&mut self) -> Option<Self::Item> {
        if self.n >= self.equip_item_list.slots.len() {
            return None;
        }
        let slot = &self.equip_item_list.slots[self.n];
        let result = if let Some(i) = slot.list_idx {
            (slot.ik, slot.n, Some(&*self.equip_item_list.item_list.items[i as usize].0))
        } else {
            (slot.ik, slot.n, None)
        };
        self.n += 1;
        return Some(result);
    }
}
    
pub struct EquipItemIter<'a> {
    equip_item_list: &'a EquipItemList,
    n: usize,
}

impl<'a> Iterator for EquipItemIter<'a> {
    type Item = (ItemKind, u8, &'a Item);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.n >= self.equip_item_list.slots.len() {
                return None;
            }
            let slot = &self.equip_item_list.slots[self.n];
            if let Some(i) = slot.list_idx {
                let result = (slot.ik, slot.n, &*self.equip_item_list.item_list.items[i as usize].0);
                self.n += 1;
                return Some(result);
            }
            self.n += 1;
        }
    }
}

