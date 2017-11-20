
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

    /// Append item
    /// If the list doesn't have empty, returns given item.
    pub fn append(&mut self, item: Box<Item>, n: u32) -> Option<Box<Item>> {
        if self.limit <= self.items.len() + 1 {
            return Some(item)
        }

        for i in self.items.iter_mut() {
            if *i.0 == *item {
                i.1 += n;
            }
        }
        
        self.items.push((item, n));
        None
    }

    /// Return item iterator
    pub fn iter(&self) -> ::std::slice::Iter<(Box<Item>, u32)> {
        self.items.iter()
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

