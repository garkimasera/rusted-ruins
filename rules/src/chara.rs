use super::Rule;

/// Rules for character parameter calculation
#[derive(Debug, Serialize, Deserialize)]
pub struct Chara {
    /// Default value of CharaParams::view_range.
    /// The actual value will be adjusted by character traits, and map attributes, etc.
    pub default_view_range: i32,
    /// Minimul speed
    pub min_spd: u16,
    /// Factor for calculating max hp
    pub max_hp_skill_factor: i32,
    /// Factor for calculating max mp
    pub max_mp_skill_factor: i32,
    /// Denominator for calculating max mp
    pub max_mp_denominator: i32,
    /// The probability of HP regeneration per turn, multiplied by VIT
    pub hp_regeneration_probability: f32,
    /// Multiplying factor of HP regeneration
    pub hp_regeneration_factor: f32,
    /// The probability of MP regeneration per turn, multiplied by WIL
    pub mp_regeneration_probability: f32,
    /// Multiplying factor of HP regeneration
    pub mp_regeneration_factor: f32,
    /// Default sp when a new character is created
    pub sp_default: f32,
    /// Maximum sp
    pub sp_max: f32,
    /// Character's sp is decreased by this value per turn
    pub sp_consumption: f32,
    /// Character's sp is decreased by this value per turn when hp is under the maximum
    pub sp_consumption_regen: f32,
    /// sp consumption factor in region maps
    pub sp_consumption_factor_in_region_map: f32,
    /// sp border of hungry
    pub sp_hungry: f32,
    /// sp border of weak
    pub sp_weak: f32,
    /// sp border of starving
    pub sp_starving: f32,
    /// (additional sp) = (nutriton) * (sp_nutrition_factor)
    pub sp_nutrition_factor: f32,
    /// (carrying capacity) = ((STR) / 2 + (VIT)) * (SKILL + 10) * (carrying_capacity_factor)
    pub carrying_capacity_factor: f32,
    pub carrying_capacity_threshold_burdened: f32,
    pub carrying_capacity_threshold_stressed: f32,
    pub carrying_capacity_threshold_strained: f32,
    pub carrying_capacity_threshold_overloaded: f32,
    /// speed coefficent for encumbrance status
    pub speed_coeff_burdened: f32,
    pub speed_coeff_stressed: f32,
    pub speed_coeff_strained: f32,
    pub speed_coeff_overloaded: f32,
    /// Damage factor for calculating encumbrance damage
    pub damage_factor_burdened: f32,
    pub damage_factor_stressed: f32,
    pub damage_factor_strained: f32,
    pub damage_factor_overloaded: f32,
    /// Dafame factor for calculating poison damage
    pub damage_factor_poisoned: f32,
}

impl Rule for Chara {
    const NAME: &'static str = "chara";
}
