
use gamedata::shop::*;
use fnv::FnvHashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Town {
    id: String,
    shops: FnvHashMap<u32, Shop>,
}

impl Town {
    pub fn new(id: &str) -> Town {
        Town {
            id: id.to_owned(),
            shops: FnvHashMap::default(),
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

    pub fn add_shop(&mut self, shop: Shop, n: u32) {
        self.shops.insert(n, shop);
    }
}

