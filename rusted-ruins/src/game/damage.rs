use crate::config::changeable::game_log_cfg;
use crate::game::extrait::*;
use crate::game::{Game, InfoGetter};
use common::gamedata::*;
use rules::RULES;

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
    origin: Option<CharaId>,
) -> i32 {
    let origin_faction = origin.map(|origin| game.gd.chara.get(origin).faction);
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
        if let (Some(origin), Some(origin_faction)) = (origin, origin_faction) {
            if !chara.ai.state.is_combat() {
                chara.ai.state = AiState::Combat { target: origin };

                if origin_faction == FactionId::player() {
                    game_log_i!("npc-get-hostile"; chara=chara);
                    let target_faction = chara.faction;
                    game.gd
                        .faction
                        .change(target_faction, RULES.faction.relvar_attacked);
                }
            }
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

        if origin == Some(CharaId::Player) && Some(cid) != origin {
            let chara = game.gd.chara.get(cid);
            let idx = chara.idx;
            let target_faction = chara.faction;
            crate::game::quest::count_slayed_monster(&mut game.gd, idx);

            game.gd
                .faction
                .change(target_faction, RULES.faction.relvar_killed);
        }
    }
    chara_hp
}
