use crate::Rule;
use common::gamedata::Duration;

/// Rules for game world
#[derive(Serialize, Deserialize)]
pub struct World {
    /// Restart map path
    pub restart_path: String,
    /// Script id to execute on restart
    pub restart_script: String,
    /// Interval of map regeneration
    pub map_regen_interval: Duration,
}

impl Rule for World {
    const NAME: &'static str = "world";
}
