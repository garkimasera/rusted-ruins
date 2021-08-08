pub mod gen;
pub mod power;
pub mod preturn;
pub mod status;
mod update;

use super::extrait::*;
use super::Game;
use crate::text::ToText;
use common::basic::WAIT_TIME_NUMERATOR;
use common::gamedata::*;
use rng::{get_rng, Rng};
use rules::RULES;

/// Additional Chara method
#[extend::ext(pub)]
impl Chara {
    /// Get this chara is main character or not.
    fn is_main_character(&self) -> bool {
        self.traits.iter().any(|t| t.1 == CharaTrait::MainCharacter)
    }

    /// Add exp to specified skill. This method should be used in this module only.
    fn add_skill_exp(&mut self, kind: SkillKind, add_exp: u32, base_level: u32) {
        let result = self.skills.add_exp(kind, add_exp, base_level);
        trace!("{} gains {} exp for {:?}", self.to_text(), result.1, kind);
        if result.0 {
            // If level up
            trace!("{} level up ({:?})", self.to_text(), kind);
            game_log!("skill-level-up"; chara=self, skill=kind);
            self.update();
        }
    }

    /// Add exp when this character attacks.
    fn add_attack_exp(&mut self, kind: SkillKind, target_level: u32) {
        self.add_skill_exp(kind, RULES.exp.attack, target_level);
    }

    /// Add exp when damaged.
    fn add_damage_exp(&mut self, damage: i32, attacker_level: u32) {
        let rel_damage = damage as f32 / self.attr.max_hp as f32;
        let exp = rel_damage * RULES.exp.endurance as f32;
        self.add_skill_exp(SkillKind::Endurance, exp as u32, attacker_level);
    }

    /// Add exp when attacked.
    fn add_evasion_exp(&mut self, attacker_level: u32) {
        self.add_skill_exp(SkillKind::Evasion, RULES.exp.evasion, attacker_level);
    }

    /// Add exp when regeneration
    fn add_healing_exp(&mut self) {
        let lv = self.skill_level(SkillKind::Healing);
        if get_rng().gen_bool(RULES.exp.healing_probability.into()) {
            self.add_skill_exp(SkillKind::Healing, RULES.exp.healing, lv);
        }
    }

    /// sp increase/decrease. return damage if sp is lower than 0.
    fn add_sp(&mut self, v: f32, cid: CharaId) -> Option<i32> {
        let r = &RULES.chara;
        let mut damage = None;
        let new_sp = if self.sp + v < r.sp_starving {
            let d = r.sp_starving - (self.sp + v);
            damage = Some(std::cmp::max(
                4 * (self.attr.max_hp as f32 * d / r.sp_max) as i32,
                1,
            ));
            r.sp_starving
        } else {
            self.sp + v
        };
        self.sp = new_sp;

        // Update status about sp
        match cid {
            CharaId::Player => {
                if v < 0.0 {
                    if new_sp <= r.sp_starving {
                        self.add_status(CharaStatus::Starving);
                    } else if new_sp <= r.sp_weak {
                        self.add_status(CharaStatus::Weak);
                    } else if new_sp <= r.sp_hungry {
                        self.add_status(CharaStatus::Hungry);
                    }
                } else if v > 0.0 {
                    if new_sp > r.sp_hungry {
                        self.remove_sp_status();
                    } else if new_sp > r.sp_weak {
                        self.add_status(CharaStatus::Hungry);
                    } else if new_sp > r.sp_starving {
                        self.add_status(CharaStatus::Weak);
                    }
                }
            }
            _ => {
                // NPC's sp is not under sp_hungry
                if new_sp < r.sp_hungry {
                    self.sp = r.sp_hungry;
                }
            }
        }
        damage
    }

    fn sub_sp(&mut self, v: f32, cid: CharaId) -> Option<i32> {
        self.add_sp(-v, cid)
    }

    /// Heal HP of this character
    fn heal(&mut self, value: i32) {
        self.hp = std::cmp::min(self.hp + value, self.attr.max_hp);
    }

    /// Update character parameters by its status
    fn update(&mut self) {
        update::update_encumbrance_status(self);
        update::update_attributes(self);
    }

    /// Reset wait time
    fn reset_wait_time(&mut self) {
        self.wait_time = WAIT_TIME_NUMERATOR / self.attr.spd as u32;
    }

    /// Return (current item weight, capacity)
    fn item_weight(&self) -> (f32, f32) {
        (
            self.item_list.sum_weight() as f32,
            update::calc_carrying_capacity(self),
        )
    }

    /// Get character's skill level summed with its adjustment value.
    fn skill_level(&self, kind: SkillKind) -> u32 {
        let (lv, adj) = self.skill_level_with_adj(kind);

        if adj > 0 {
            lv + adj as u32
        } else {
            let adj = adj.abs() as u32;
            lv.saturating_sub(adj)
        }
    }

    /// Get character's base skill level and the adjustment value.
    fn skill_level_with_adj(&self, kind: SkillKind) -> (u32, i32) {
        let lv = self.skills.skills.get(&kind).copied().unwrap_or(0);

        let mut adj = 0;
        let mut adj_factor = 0.0;

        if let Some(bonus_level) = RULES.class.get(self.class).skill_bonus.get(&kind) {
            let bonus = RULES.params.skill_bonus[bonus_level];
            adj_factor += bonus.0;
            adj += bonus.1;
        }

        let adj = adj + (lv as f32 * adj_factor) as i32;

        (lv, adj)
    }

    /// active skill available or not.
    fn active_skill_available(&self, active_skill: &ActiveSkill) -> bool {
        self.sp > active_skill.cost_sp as f32
    }
}
