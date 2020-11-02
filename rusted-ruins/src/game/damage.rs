use crate::config::changeable::game_log_cfg;
use crate::config::UI_CFG;
use crate::game::{Game, InfoGetter};
use common::gamedata::*;
use geom::Vec2d;

#[derive(Clone, Copy)]
pub enum CharaDamageKind {
    MeleeAttack,
    RangedAttack,
    Poison,
    Starve,
}

pub struct DamageView {
    damaged_charas: Vec<DamagedChara>,
}

#[derive(Clone, Copy, Debug)]
pub struct DamagedChara {
    pub cid: CharaId,
    pub pos: Vec2d,
    pub damage: i32,
    pub passed_frame: u32,
}

impl DamageView {
    pub fn new() -> DamageView {
        DamageView {
            damaged_charas: Vec::new(),
        }
    }

    pub fn push(&mut self, cid: CharaId, pos: Vec2d, damage: i32) {
        if let Some(damaged_chara) = self
            .damaged_charas
            .iter_mut()
            .find(|damaged_chara| damaged_chara.cid == cid)
        {
            damaged_chara.damage += damage;
            damaged_chara.pos = pos;
            damaged_chara.passed_frame = 0;
        } else {
            self.damaged_charas.push(DamagedChara {
                cid,
                pos,
                damage,
                passed_frame: 0,
            });
        }
    }

    pub fn advance(&mut self) {
        for damaged_chara in &mut self.damaged_charas {
            damaged_chara.passed_frame += 1;
        }

        self.damaged_charas
            .retain(|damaged_chara| damaged_chara.passed_frame < UI_CFG.damage.n_frame);
    }

    pub fn iter(&self) -> std::slice::Iter<DamagedChara> {
        self.damaged_charas.iter()
    }
}

/// Give damage to a character.
pub fn do_damage(game: &mut Game, cid: CharaId, damage: i32, damage_kind: CharaDamageKind) -> i32 {
    let pos = game.gd.chara_pos(cid);
    let chara = game.gd.chara.get_mut(cid);

    chara.hp -= damage;

    // Damage log
    if game_log_cfg().combat_log.damage() {
        game_log!("damaged-chara"; chara=chara, damage=damage);
    }

    if let Some(pos) = pos {
        game.damage_view.push(cid, pos, damage);
    } else {
        error!("damage to character that is not on map");
    }

    if chara.hp < 0 {
        // Logging
        match damage_kind {
            CharaDamageKind::MeleeAttack => {
                game_log!("killed-by-melee-attack"; chara=chara);
            }
            CharaDamageKind::RangedAttack => {
                game_log!("killed-by-ranged-attack"; chara=chara);
            }
            CharaDamageKind::Poison => {
                game_log!("killed-by-poison-damage"; chara=chara);
            }
            CharaDamageKind::Starve => {
                game_log!("killed-by-starve-damage"; chara=chara);
            }
        }
    }
    chara.hp
}
