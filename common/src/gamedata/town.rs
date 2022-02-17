use crate::gamedata::item::ItemLocation;
use crate::gamedata::quest::*;
use crate::gamedata::shop::*;
use crate::gamedata::time::Time;
use crate::objholder::ItemIdx;
use fnv::FnvHashMap;
use std::collections::hash_map::{Keys, Values, ValuesMut};
use std::iter::Copied;

use super::ItemListLocation;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Town {
    id: String,
    pub quests: Vec<TownQuest>,
    pub quests_last_update: Time,
    pub delivery_chest: Option<ItemListLocation>,
    pub delivery_chest_content: Vec<(ItemIdx, u32)>,
}

impl Town {
    pub fn new(id: &str) -> Town {
        Town {
            id: id.to_owned(),
            quests: Vec::new(),
            quests_last_update: Time::from_secs(0),
            delivery_chest: None,
            delivery_chest_content: Vec::new(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}
