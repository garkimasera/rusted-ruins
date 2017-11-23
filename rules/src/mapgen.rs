
use std::collections::HashMap;
use common::gamedata::site::DungeonKind;
use common::gamedata::chara::Race;

/// Rules for map generation
#[derive(Serialize, Deserialize)]
pub struct MapGen {
    pub npc_gen: HashMap<DungeonKind, HashMap<Race, f32>>,
    /// Items on a map is generated with a probability of 1/ item_gen_probability
    pub item_gen_probability: u32,
}

impl MapGen {
    pub fn new() -> MapGen {
        MapGen {
            npc_gen: HashMap::new(),
            item_gen_probability: 0,
        }
    }
}

