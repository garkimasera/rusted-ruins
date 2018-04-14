
use std::collections::HashMap;
use array2d::Vec2d;
use common::gamedata::*;

/// Rules for map generation
pub type DungeonGen = HashMap<DungeonKind, DungeonGenParams>;

#[derive(Serialize, Deserialize)]
pub struct DungeonGenParams {
    /// Default map size
    pub map_size: Vec2d,
    /// The probability of npc generation for each race
    pub npc_race_probability: HashMap<Race, f32>,
    /// Tile and wall ids
    pub terrain: Vec<[String; 2]>,
    /// Items on a map is generated with a probability of 1/ item_gen_probability
    pub item_gen_probability: u32,
}

