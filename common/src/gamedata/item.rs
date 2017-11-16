
use objholder::ItemIdx;

#[derive(Clone, Debug, Serialize, Deserialize)]
/// Game item
pub struct Item {
    pub id: String,
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

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum PotionKind {
    Heal,
}

/// Item list that records all items owned by one character or one tile
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemList {
    pub limit: usize,
    pub items: Vec<Box<Item>>,
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
}

