use crate::gamedata::quest::Quest;
use crate::gamedata::shop::*;
use fnv::FnvHashMap;
use std::collections::hash_map::{Values, ValuesMut};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Town {
    id: String,
    shops: FnvHashMap<u32, Shop>,
    pub quests: Vec<Quest>,
}

impl Town {
    pub fn new(id: &str) -> Town {
        Town {
            id: id.to_owned(),
            shops: FnvHashMap::default(),
            quests: Vec::new(),
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

    pub fn iter_shops(&self) -> Values<u32, Shop> {
        self.shops.values()
    }

    pub fn iter_shops_mut(&mut self) -> ValuesMut<u32, Shop> {
        self.shops.values_mut()
    }

    pub fn add_shop(&mut self, shop: Shop, n: u32) {
        self.shops.insert(n, shop);
    }
}
