
/// Rules for exp calculation
#[derive(Serialize, Deserialize)]
pub struct Exp {
    /// Level difference (skill level - base level) for the first element of adjust_coeff
    pub begin_adjust_coeff: isize,
    /// Coefficient to adjust exp by difference skill level between base level
    pub adjust_coeff: Vec<f32>,
    /// All exp is multiplied by this value
    pub base_factor: f32,
    /// Base exp to weapon skills after attacking
    pub attack: u32,
    /// Base exp to Endurance skill when attacked
    pub endurance: u32,
    /// Base exp to Evasion skill when attacked
    pub evasion: u32,
}

