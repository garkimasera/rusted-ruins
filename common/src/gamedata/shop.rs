use crate::gamedata::item::ItemList;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Shop {
    pub items: ItemList,
    /// Shop level is used to choose shop items
    pub level: u32,
}
