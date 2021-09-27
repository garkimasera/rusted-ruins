use super::Rule;

/// Rules for calculation related to combat.
#[derive(Serialize, Deserialize)]
pub struct Combat {
    pub skill_base: f32,
    pub throw_range_factor: u32,
    pub throw_range_max: u32,
    pub bare_hand_power_base: f32,
    pub bare_hand_power_factor: f32,
    pub bare_hand_power_var: f32,
    /// Normal state AI will become combat state when detect an enemy in this range.
    pub detection_range: i32,
    /// Factor for detection probability.
    pub detection_factor: f32,
}

impl Rule for Combat {
    const NAME: &'static str = "combat";

    fn append(&mut self, other: Self) {
        *self = other;
    }
}
