
use std::collections::HashMap;
use common::gamedata::site::DungeonKind;
use common::gamedata::chara::Race;

/// Rules for map generation
#[derive(Serialize, Deserialize)]
pub struct MapGen {
    pub dungeons: HashMap<DungeonKind, DungeonGenParams>,
    /// Items on a map is generated with a probability of 1/ item_gen_probability
    pub item_gen_probability: u32,
}

#[derive(Serialize, Deserialize)]
pub struct DungeonGenParams {
    pub npc_race_probability: HashMap<Race, f32>,
}

impl MapGen {
    pub fn new() -> MapGen {
        MapGen {
            dungeons: HashMap::new(),
            item_gen_probability: 0,
        }
    }
}

