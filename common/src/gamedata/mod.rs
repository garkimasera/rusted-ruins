
pub mod item;
pub mod chara;
pub mod map;
pub mod site;

use std::collections::HashMap;

use self::map::{Map, MapId};

/// Includes all data for one game
#[derive(Serialize, Deserialize)]
pub struct GameData {
    site: HashMap<site::SiteId, site::Site>,
    chara: HashMap<chara::CharaId, chara::Chara>,
}

impl GameData {
    pub fn get_map(&self, id: &MapId) -> &Map {
        &self.site.get(&id.site).expect("Internal Error: MapId not exist").get_map(id.floor)
    }
}

