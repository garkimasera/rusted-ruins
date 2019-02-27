use common::gamedata::*;
use std::collections::HashMap;

/// Rules for map generation
pub type DungeonGen = HashMap<DungeonKind, DungeonGenParams>;

#[derive(Serialize, Deserialize)]
pub struct DungeonGenParams {
    /// Specify floor generation type that is described in floor_gen and its weight.
    pub floor_gen: Vec<(String, f32)>,
    /// The probability of npc generation for each race
    pub npc_race_probability: HashMap<Race, f32>,
    /// Tile and wall ids
    pub terrain: Vec<[String; 2]>,
    /// Items generatation probability on each tile
    pub item_gen_probability: f64,
    /// The range of number of floor of auto generated dungeons
    pub floor_range: [u32; 2],
}
