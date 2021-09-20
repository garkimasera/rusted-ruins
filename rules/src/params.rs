use common::basic::BonusLevel;
use std::collections::HashMap;

/// Various parameters for game playing
#[derive(Serialize, Deserialize)]
pub struct Params {
    /// Minutes per one turn on maps in sites
    pub minutes_per_turn_normal: f32,
    /// Tile size in region map
    pub regionmap_tile_size: f32,
    /// Skill bonus (base * value.0 + value.1).
    pub skill_bonus: HashMap<BonusLevel, (f32, i32)>,
}
