use common::item_selector::ItemSelector;
use serde_with::{serde_as, DisplayFromStr};
use std::collections::HashMap;

/// Used for town simulation
#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct Town {
    /// The minimum number of shop items
    pub min_shop_items: u32,
    /// The maximum number of shop items
    pub max_shop_items: u32,
    /// Shop kinds and its item selectors.
    #[serde_as(as = "HashMap<_, DisplayFromStr>")]
    pub shop_kinds: HashMap<String, ItemSelector>,
}
