
use crate::gamedata::item::ItemList;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
pub enum ShopKind {
    /// Sells weapons and armors
    Equipment,
    /// Sells potions
    Potion,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Shop {
    pub kind: ShopKind,
    pub items: ItemList,
    /// Shop level is used to choose shop items
    pub level: u32,
}

