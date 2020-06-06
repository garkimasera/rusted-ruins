use common::gamedata::*;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct NpcAIs(HashMap<NpcAIKind, NpcAI>);

impl NpcAIs {
    pub fn get(&self, kind: NpcAIKind) -> &NpcAI {
        self.0
            .get(&kind)
            .unwrap_or_else(|| &self.0[&NpcAIKind::default()])
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NpcAI {
    pub move_kind: MoveKind,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MoveKind {
    NoMove,
    Melee,
    Wander,
}
