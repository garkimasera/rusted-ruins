use crate::config::changeable::game_log_cfg;
use crate::game::extrait::*;
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
    game: &mut Game,
    cid: CharaId,
    damage: i32,
    damage_kind: CharaDamageKind,
    origin: Option<CharaId>,
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

    let chara_hp = chara.hp;

    if chara_hp > 0 {
        // Faction process
        if origin == Some(CharaId::Player)
            && game.gd.chara_relation(CharaId::Player, cid) != Relationship::Hostile
        {
            let chara = game.gd.chara.get_mut(cid);
            chara.add_status(CharaStatus::Hostile {
                faction: FactionId::player(),
            });
        }
    } else {
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
    chara_hp
}
