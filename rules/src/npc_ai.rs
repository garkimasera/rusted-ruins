use common::gamedata::*;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct NpcAIs(HashMap<NpcAiKind, NpcAi>);

impl NpcAIs {
    pub fn get(&self, kind: NpcAiKind) -> &NpcAi {
        self.0
            .get(&kind)
            .unwrap_or_else(|| &self.0[&NpcAiKind::default()])
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NpcAi {
    pub move_kind: MoveKind,
    pub pathfinding_step: u32,
    /// Probability of random walk when normal state.
    #[serde(default)]
    pub walk_prob: f32,
    /// Probability of approaching to enemy when combat state.
    #[serde(default)]
    pub approach_enemy_prob: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MoveKind {
    NoMove,
    Wander,
    Return,
}
