use crate::Rule;

/// Rules for effect processing
#[derive(Debug, Serialize, Deserialize)]
pub struct Effect {
    pub item_drink_power_factor: f32,
    pub item_eat_power_factor: f32,
    pub mining_power_factor: f32,
    pub mining_power_base: f32,
    pub restore_hp_factor: f32,
    pub throw_weight_to_power_factor: f32,
}

impl Rule for Effect {
    const NAME: &'static str = "effect";
}
