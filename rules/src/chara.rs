use common::gamedata::*;
use std::collections::HashMap;

/// Rules for character parameter calculation
#[derive(Debug, Serialize, Deserialize)]
pub struct Chara {
    /// Attribute revisions by class
    pub class_revision: HashMap<CharaClass, CharaAttrRevision>,
    /// Default value of CharaParams::view_range.
    /// The actual value will be adjusted by character traits, and map attributes, etc.
    pub default_view_range: i32,
    /// The probability of HP regeneration per turn.
    pub hp_regeneration_probability: f32,
    /// Multiplying factor of HP regeneration
    pub hp_regeneration_factor: f32,
    /// Default sp when a new character is created.
    pub sp_default: f32,
    /// Maximum sp
    pub sp_max: f32,
    /// Character's sp is decreased by this value per turn.
    pub sp_consumption: f32,
    /// Character's sp is decreased by this value per turn when hp is under the maximum.
    pub sp_consumption_regen: f32,
    /// sp border of hungry
    pub sp_hungry: f32,
    /// sp border of weak
    pub sp_weak: f32,
    /// sp border of starving
    pub sp_starving: f32,
}
