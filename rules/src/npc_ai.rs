use crate::Rule;
use common::gamedata::*;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct NpcAIs(HashMap<NpcAiKind, NpcAi>);

impl Rule for NpcAIs {
    const NAME: &'static str = "npc_ai";

    fn append(&mut self, other: Self) {
        for (k, v) in other.0.into_iter() {
            self.0.insert(k, v);
        }
    }
}

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
    /// Probabilities of npc actions in combat.
    #[serde(default)]
    pub combat_prob: HashMap<CombatActionKind, f32>,
    /// Probability of approaching to enemy when combat state.
    #[serde(default)]
    pub approach_enemy_prob: f32,
    /// Probability of using ranged weapon.
    #[serde(default)]
    pub ranged_weapon_prob: f32,
    /// Probability of trying to use active skill.
    #[serde(default)]
    pub ability_prob: f32,
    #[serde(default = "search_turn_default")]
    /// Required turn to change state search to normal
    pub search_turn: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MoveKind {
    NoMove,
    Wander,
    Return,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum CombatActionKind {
    Skip,
    ApproachEnemy,
    RangedWeapon,
    Ability,
}

impl CombatActionKind {
    pub const ALL: &'static [CombatActionKind] = &[
        CombatActionKind::ApproachEnemy,
        CombatActionKind::RangedWeapon,
        CombatActionKind::Ability,
    ];
}

fn search_turn_default() -> u32 {
    10
}
