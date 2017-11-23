
use objholder::ItemIdx;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
/// Game item
pub struct Item {
    pub idx: ItemIdx,
    pub content: ItemContent,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum ItemKind {
    Object,
    Potion,
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
}

impl ItemContent {
    pub fn kind(&self) -> ItemKind {
        match *self {
            ItemContent::Object => ItemKind::Object,
            ItemContent::Potion { .. } => ItemKind::Potion,
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum PotionKind {
    Heal,
}

pub enum ItemListLocation {
    OnMap { mid: super::map::MapId },
    Chara { cid: super::chara::CharaId },
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
    pub fn remove<T: Into<ItemMoveNum>>(&mut self, i: usize, n: T) {
        let n = n.into().to_u32(self.items[i].1);
        assert!(self.items[i].1 <= n && n != 0);
        if n == 0 { return; }

        self.items[i].1 -= n;
        if self.items[i].1 == 0 {
            self.items.remove(i);
        }
    }

    /// Remove an item from list and get its clone or moved value
    pub fn remove_and_get<T: Into<ItemMoveNum>>(&mut self, i: usize, n: T) -> Box<Item> {
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
#[derive(Clone, Debug)]
pub struct ItemFilter {
    pub all: bool,
    pub kind: Option<ItemKind>,
}

impl ItemFilter {
    /// Given item will be filtered (false) or not (true)
    pub fn judge(&self, item: &Item) -> bool {
        if self.all { return true; }
        
        if let Some(kind) = self.kind {
            if item.content.kind() != kind { return false; }
        }
        
        true
    }
}

pub struct FilteredItemList<'a> {
    item_list: &'a ItemList,
    filter: ItemFilter,
    count: usize,
}

impl<'a> FilteredItemList<'a> {
    pub fn new(item_list: &'a ItemList, filter: ItemFilter) -> FilteredItemList<'a> {
        FilteredItemList {
            item_list, filter, count: 0,
        }
    }

    pub fn all(item_list: &'a ItemList) -> FilteredItemList<'a> {
        let filter = ItemFilter {
            all: true, kind: None,
        };
        FilteredItemList::new(item_list, filter)
    }
}

impl<'a> Iterator for FilteredItemList<'a> {
    type Item = (usize, &'a Item, u32);
    
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.item_list.items.len() <= self.count {
                return None
            }
            let a = &self.item_list.items[self.count];

            let prev_count = self.count;
            self.count += 1;

            if self.filter.judge(&*a.0) {
                return Some((prev_count, &*a.0, a.1));
            }
        }
    }
}

