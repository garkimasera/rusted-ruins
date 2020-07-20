use crate::gamedata::item::ItemList;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShopKind {
    /// Items are specified by id or group.
    Specified,
    /// Sells weapons and armors
    Equipment,
    /// Sells foods
    Food,
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
