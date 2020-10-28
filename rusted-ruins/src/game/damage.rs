use crate::game::Game;
use common::gamedata::*;

#[derive(Clone, Copy)]
pub enum CharaDamageKind {
    MeleeAttack,
    RangedAttack,
    Poison,
    Starve,
}

pub struct DamageView {}

impl DamageView {
    pub fn new() -> DamageView {
        DamageView {}
    }
}

/// Give damage to a character.
pub fn do_damage(game: &mut Game, cid: CharaId, damage: i32, damage_kind: CharaDamageKind) -> i32 {
    let chara = game.gd.chara.get_mut(cid);

    chara.hp -= damage;

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
