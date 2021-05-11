use common::basic::BonusLevel;
use std::collections::HashMap;

/// Various parameters for game playing
#[derive(Serialize, Deserialize)]
pub struct Params {
    /// Initial game date (year)
    pub initial_date_year: u32,
    /// Initial game date (month)
    pub initial_date_month: u32,
    /// Initial game date (day)
    pub initial_date_day: u32,
    /// Initial game date (hour)
    pub initial_date_hour: u32,
    /// Minutes per one turn on maps in sites
    pub minutes_per_turn_normal: f32,
    /// Minutes per one turn on region maps
    pub minutes_per_turn_region: f32,
    /// Restart map path
    pub restart_path: String,
    /// Skill bonus (base * value.0 + value.1).
    pub skill_bonus: HashMap<BonusLevel, (f32, i32)>,
}
