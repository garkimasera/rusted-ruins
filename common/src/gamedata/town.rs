
use std::collections::HashMap;
use gamedata::shop::Shop;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Town {
    id: String,
    pub shops: HashMap<String, Shop>,
}

impl Town {
    pub fn new(id: &str) -> Town {
        Town {
            id: id.to_owned(),
            shops: HashMap::new(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

