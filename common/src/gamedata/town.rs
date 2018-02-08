
use std::collections::HashMap;
use gamedata::shop::Shop;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct TownId(pub(crate) u32);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Town {
    id: String,
    name: Option<String>,
    shops: HashMap<String, Shop>,
}

