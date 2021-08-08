use crate::config::changeable::game_log_cfg;
use crate::game::{Game, InfoGetter};
use common::gamedata::*;

#[derive(Clone, Copy)]
pub enum CharaDamageKind {
    MeleeAttack,
    RangedAttack,
    Explosion,
    Poison,
    Starve,
}

/// Give damage to a character.
pub fn do_damage(
    game: &mut Game<'_>,
    cid: CharaId,
    damage: i32,
    damage_kind: CharaDamageKind,
) -> i32 {
    let pos = game.gd.chara_pos(cid);
    let chara = game.gd.chara.get_mut(cid);

    chara.hp -= damage;

    // Damage log
    if game_log_cfg().combat_log.damage() {
        game_log!("damaged-chara"; chara=chara, damage=damage);
    }

    if let Some(pos) = pos {
        crate::chara_log::get_log_mut().push_damage(cid, pos, damage);
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
            CharaDamageKind::Explosion => {
                game_log!("killed-by-explosion"; chara=chara);
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
