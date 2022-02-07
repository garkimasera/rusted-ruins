use super::Rule;
use ordered_float::NotNan;

/// Rules for calculation related to power/hit calclation.
#[derive(Serialize, Deserialize)]
pub struct Power {
    pub skill_base: f32,
    pub base_evasion_power: f32,
    pub base_defence: f32,
    pub bare_hand_hit: NotNan<f32>,
    pub bare_hand_power_base: f32,
    pub bare_hand_power_factor: f32,
    pub bare_hand_power_var: f32,
    pub hit_calc_factor0: f32,
    pub hit_calc_factor1: f32,
    pub hit_calc_factor2: f32,
    pub throw_weight_factor: f32,
    pub throw_hit_str_factor: f32,
    pub medical_power_base: f32,
}

impl Rule for Power {
    const NAME: &'static str = "power";

    fn append(&mut self, other: Self) {
        *self = other;
    }
}
