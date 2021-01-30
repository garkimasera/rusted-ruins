use common::gamedata::*;
use std::collections::HashMap;

/// Rules for map generation
pub type DungeonGen = HashMap<DungeonKind, DungeonGenParams>;

#[derive(Serialize, Deserialize)]
pub struct DungeonGenParams {
    /// Specify map generation type that is described in map_gen and its weight.
    pub map_gen: Vec<(String, f32)>,
    /// The probability of npc generation for each race
    pub npc_race_probability: HashMap<String, f32>,
    /// Default npc faction
    pub default_faction_id: FactionId,
    /// Tile and wall ids
    pub terrain: Vec<[String; 2]>,
    /// Additional walls to replace default wall.
    pub sub_walls: Vec<(String, f32)>,
    /// Item generatation probability on each tile
    pub item_gen_probability: f64,
    /// Item generation weight for each ItemKind
    pub item_gen_weight_for_kind: HashMap<ItemKindRough, f32>,
    /// The range of number of floor of auto generated dungeons
    pub floor_range: [u32; 2],
    /// Default map music
    pub music: String,
}
