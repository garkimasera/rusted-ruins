/// Rules for effect processing
#[derive(Debug, Serialize, Deserialize)]
pub struct Effect {
    pub mining_power_factor: f32,
    pub mining_power_base: f32,
}
