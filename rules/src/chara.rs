
/// Rules for character parameter calculation
#[derive(Serialize, Deserialize)]
pub struct Chara {
    /// Default value of CharaParams::view_range
    /// The actual value will be adjusted by character traits, and map attributes, etc.
    pub default_view_range: i32,
    /// Default sp when a new character is created.
    pub default_sp: i32,
    /// Character's sp is decreased by this value per turn.
    pub sp_consumption: i32,
}

