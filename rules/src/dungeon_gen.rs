
use std::collections::HashMap;
use common::gamedata::site::DungeonKind;
use common::gamedata::chara::Race;

/// Rules for map generation
pub type DungeonGen = HashMap<DungeonKind, DungeonGenParams>;

#[derive(Serialize, Deserialize)]
pub struct DungeonGenParams {
    /// The probability of npc generation for each race
    pub npc_race_probability: HashMap<Race, f32>,
    /// Tile and wall ids
    pub terrain: Vec<[String; 2]>,
    /// Items on a map is generated with a probability of 1/ item_gen_probability
    pub item_gen_probability: u32,
}

