pub mod gen;
pub mod preturn;
pub mod status;
mod update;

use super::combat::DamageKind;
use super::extrait::*;
use super::Game;
use crate::text::ToText;
use common::basic::WAIT_TIME_NUMERATOR;
use common::gamedata::*;
use rules::RULES;

/// Additional Chara method
pub trait CharaEx {
    /// Add exp to specified skill. This method should be used in this module only.
    fn add_skill_exp(&mut self, kind: SkillKind, add_exp: u32, base_level: u32);
    /// Add exp when this character attacks.
    fn add_attack_exp(&mut self, kind: SkillKind, target_level: u32);
    /// Add exp when damaged.
    fn add_damage_exp(&mut self, damage: i32, attacker_level: u32);
    /// Add exp when attacked.
    fn add_evasion_exp(&mut self, attacker_level: u32);
    /// sp increase/decrease.
    fn add_sp(&mut self, v: f32, cid: CharaId);
    /// Give damage to this character
    fn damage(&mut self, damage: i32, damage_kind: DamageKind) -> i32;
    /// Update character parameters by its status
    fn update(&mut self);
    /// Reset wait time
    fn reset_wait_time(&mut self);
}

impl CharaEx for Chara {
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

    fn add_attack_exp(&mut self, kind: SkillKind, target_level: u32) {
        self.add_skill_exp(kind, RULES.exp.attack, target_level);
    }

    fn add_damage_exp(&mut self, damage: i32, attacker_level: u32) {
        let rel_damage = damage as f32 / self.attr.max_hp as f32;
        let exp = rel_damage * RULES.exp.endurance as f32;
        self.add_skill_exp(SkillKind::Endurance, exp as u32, attacker_level);
    }

    fn add_evasion_exp(&mut self, attacker_level: u32) {
        self.add_skill_exp(SkillKind::Evasion, RULES.exp.evasion, attacker_level);
    }

    fn add_sp(&mut self, v: f32, cid: CharaId) {
        let r = &RULES.chara;
        let old_sp = self.sp;
        let new_sp = if self.sp + v < r.sp_starving {
            r.sp_starving
        } else {
            self.sp + v
        };
        self.sp = new_sp;

        // Update status about sp
        match cid {
            CharaId::Player => {
                if v < 0.0 {
                    if new_sp <= r.sp_hungry && old_sp > r.sp_hungry {
                        self.add_status(CharaStatus::Hungry);
                    }
                    if new_sp <= r.sp_weak && old_sp > r.sp_weak {
                        self.add_status(CharaStatus::Weak);
                    }
                    if new_sp <= r.sp_starving && old_sp > r.sp_starving {
                        self.add_status(CharaStatus::Starving);
                    }
                } else if v > 0.0 {
                    if new_sp > r.sp_hungry && old_sp <= r.sp_hungry {
                        self.remove_sp_status();
                    }
                    if new_sp > r.sp_weak && old_sp <= r.sp_weak {
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
    }

    fn damage(&mut self, damage: i32, damage_kind: DamageKind) -> i32 {
        self.hp -= damage;

        if self.hp < 0 {
            // Logging
            match damage_kind {
                DamageKind::MeleeAttack => {
                    game_log!("killed-by-melee-attack"; chara=self);
                }
                DamageKind::RangedAttack => {
                    game_log!("killed-by-ranged-attack"; chara=self);
                }
                DamageKind::Poison => {
                    game_log!("killed-by-poison-damage"; chara=self);
                }
            }
        }
        self.hp
    }

    fn update(&mut self) {
        update::update_attributes(self);
    }

    fn reset_wait_time(&mut self) {
        self.wait_time = WAIT_TIME_NUMERATOR / self.attr.spd as u32;
    }
}
