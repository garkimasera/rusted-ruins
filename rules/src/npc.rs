/// Various parameters for game playing
#[derive(Serialize, Deserialize)]
pub struct Npc {
    /// Duration of npc recovering after map switching.
    pub map_switch_recover_minutes: u32,
}
