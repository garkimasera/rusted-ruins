use crate::gamedata::item::ItemLocation;
use crate::gamedata::quest::*;
use crate::gamedata::shop::*;
use crate::gamedata::time::Time;
use crate::objholder::ItemIdx;
use fnv::FnvHashMap;
use std::collections::hash_map::{Keys, Values, ValuesMut};
use std::iter::Copied;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Town {
    id: String,
    shops: FnvHashMap<u32, Shop>,
    pub quests: Vec<TownQuest>,
    pub quests_last_update: Time,
    pub delivery_chest_content: Vec<(ItemIdx, u32)>,
}

impl Town {
    pub fn new(id: &str) -> Town {
        Town {
            id: id.to_owned(),
            shops: FnvHashMap::default(),
            quests: Vec::new(),
            quests_last_update: Time::from_secs(0),
            delivery_chest_content: Vec::new(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn get_shop(&self, n: u32) -> Option<&Shop> {
        self.shops.get(&n)
    }

    pub fn get_shop_mut(&mut self, n: u32) -> Option<&mut Shop> {
        self.shops.get_mut(&n)
    }

    pub fn iter_shop_n(&self) -> Copied<Keys<'_, u32, Shop>> {
        self.shops.keys().copied()
    }

    pub fn iter_shops(&self) -> Values<'_, u32, Shop> {
        self.shops.values()
    }

    pub fn iter_shops_mut(&mut self) -> ValuesMut<'_, u32, Shop> {
        self.shops.values_mut()
    }

    pub fn add_shop(&mut self, shop: Shop, n: u32) {
        self.shops.insert(n, shop);
    }
}
