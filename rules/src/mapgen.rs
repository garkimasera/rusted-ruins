
use std::collections::HashMap;
use common::gamedata::site::DungeonKind;
use common::gamedata::chara::Race;

/// Rules for map generation
#[derive(Serialize, Deserialize)]
pub struct MapGen {
    pub npc_gen: HashMap<DungeonKind, HashMap<Race, f32>>,
}

impl MapGen {
    pub fn new() -> MapGen {
        MapGen {
            npc_gen: HashMap::new(),
        }
    }
}

