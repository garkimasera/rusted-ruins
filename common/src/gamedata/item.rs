
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
    Thing,
    Potion,
}

/// Kind dependent data for a item
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum ItemContent {
    Thing,
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

/// Inventory for one character
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Inventory {
    pub limit: usize,
    pub items: Vec<Item>,
}

impl Inventory {
    pub fn new(limit: usize) -> Inventory {
        Inventory {
            limit, items: Vec::new(),
        }
    }    

    pub fn for_chara() -> Inventory {
        Self::new(::basic::INVENTORY_MAX_ITEM_CHARA)
    }

    /// Inventory for player has larger size
    pub fn for_player() -> Inventory {
        Self::new(::basic::INVENTORY_MAX_ITEM_PLAYER)
    }
}

