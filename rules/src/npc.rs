use crate::Rule;

/// Various parameters for game playing
#[derive(Serialize, Deserialize)]
pub struct Npc {
    /// Duration of npc recovering after map switching
    pub map_switch_recover_minutes: u32,
    /// The maximum size of player's party
    pub party_size_max: u32,
    pub party_pathfinding_step: u32,
}

impl Rule for Npc {
    const NAME: &'static str = "npc";
}
