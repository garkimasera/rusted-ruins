
use gamedata::shop::*;
use fnv::FnvHashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Town {
    id: String,
    pub shops: FnvHashMap<u32, Shop>,
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
}

