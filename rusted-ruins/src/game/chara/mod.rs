
pub mod creation;
pub mod preturn;
pub mod status;
mod update;

use common::gamedata::*;
use rules::RULES;
use text::ToText;
use super::Game;
use super::extrait::*;
use super::combat::DamageKind;

/// Additional Chara method
pub trait CharaEx {
    fn add_skill_exp(&mut self, kind: SkillKind, add_exp: u32, base_level: u16);
    /// sp increase/decrease.
    fn add_sp(&mut self, v: i32, cid: CharaId);
    /// Update character parameters by its status
    fn update(&mut self);
}

impl CharaEx for Chara {
    fn add_skill_exp(&mut self, kind: SkillKind, add_exp: u32, base_level: u16) {
        let result = self.skills.add_exp(kind, add_exp, base_level);
        trace!("{} gains {} exp for {:?}", self.to_text(), result.1, kind);
        if result.0 { // If level up
            trace!("{} level up ({:?})", self.to_text(), kind);
            game_log!("skill-level-up"; chara=self, skill=kind);
        }
    }

    fn add_sp(&mut self, v: i32, cid: CharaId) {
        let old_sp = self.sp;
        let new_sp = self.sp + v;
        self.sp = new_sp;
        let r = &RULES.chara;

        // Update status about sp
        match cid {
            CharaId::Player => {
                if v < 0 {
                    if new_sp <= r.sp_hungry && old_sp > r.sp_hungry {
                        self.add_status(CharaStatus::Hungry);
                    }
                    if new_sp <= r.sp_weak && old_sp > r.sp_weak {
                        self.add_status(CharaStatus::Weak);
                    }
                    if new_sp <= r.sp_starving && old_sp > r.sp_starving {
                        self.add_status(CharaStatus::Starving);
                    }
                } else if v > 0 {
                    if new_sp > r.sp_hungry && old_sp <= r.sp_hungry {
                        self.remove_sp_status();
                    }
                    if new_sp > r.sp_weak && old_sp <= r.sp_weak {
                        self.add_status(CharaStatus::Weak);
                    }
                }
            }
            _ => {
                if new_sp < 0 {
                    self.sp = 0;
                }
            }
        }
    }

    fn update(&mut self) {
        update::update_params(self);
    }
}

pub fn damage(game: &mut Game, cid: CharaId, damage: i32, damage_kind: DamageKind) {
    let chara = game.gd.chara.get_mut(cid);
    
    chara.hp -= damage;

    if chara.hp < 0 {
        game.dying_charas.push(cid);
        // Logging
        match damage_kind {
            DamageKind::MeleeAttack => {
                game_log!("killed-by-melee-attack"; chara=chara);
            }
            DamageKind::RangedAttack => {
                game_log!("killed-by-ranged-attack"; chara=chara);
            }
            DamageKind::Poison => {
                game_log!("killed-by-poison-damage"; chara=chara);
            }
        }
    }
}

