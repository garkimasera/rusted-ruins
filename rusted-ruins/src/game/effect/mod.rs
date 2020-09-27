use super::combat::{attack_target, AttackParams, DamageKind};
use crate::game::extrait::CharaStatusOperation;
use crate::game::Game;
use common::gamedata::*;
use geom::*;

pub fn process_effect(
    game: &mut Game,
    effect: &Effect,
    cid: Option<CharaId>,
    pos: Option<Vec2d>,
    power: f64,
) {
    // TODO: multiple cids will be needed for widely ranged effect.
    let cids = if cid.is_some() {
        vec![cid.unwrap()]
    } else {
        if let Some(pos) = pos {
            if let Some(cid) = game.gd.get_current_map().get_chara(pos) {
                vec![cid]
            } else {
                vec![]
            }
        } else {
            vec![]
        }
    };

    for effect_kind in &effect.kind {
        match effect_kind {
            EffectKind::Ranged { element } => {
                for cid in &cids {
                    ranged_attack(game, *cid, power, *element);
                }
            }
            EffectKind::Status { status } => {
                for cid in &cids {
                    cause_status(game, *cid, power, *status);
                }
            }
            _ => (),
        }
    }
}

// Ranged attack to a chara.
fn ranged_attack(game: &mut Game, cid: CharaId, attack_power: f64, element: Element) {
    if game.target_chara.is_none() {
        let player = game.gd.chara.get(CharaId::Player);
        game_log_i!("no-target"; chara=player);
        return;
    }
    let target_id = game.target_chara.unwrap();
    let start = game.gd.get_current_map().chara_pos(cid).unwrap();
    let target_pos = game.gd.get_current_map().chara_pos(target_id).unwrap();

    game.anim_queue.push_magic_arrow(start, target_pos);

    let target = game.gd.chara.get(target_id);
    game_log!("arrow-hit"; chara=target);

    let attack_params = AttackParams {
        attacker_id: Some(cid),
        kind: DamageKind::RangedAttack,
        element,
        attack_power,
    };

    attack_target(game, attack_params, target_id);
}

// Cause status effect to given chara.
fn cause_status(game: &mut Game, cid: CharaId, power: f64, status: StatusEffect) {
    let chara = game.gd.chara.get_mut(cid);

    match status {
        StatusEffect::Asleep => {
            chara.add_status(CharaStatus::Asleep {
                turn_left: power as u16,
            });
            game_log!("fall-asleep"; chara=chara);
        }
        StatusEffect::Poison => {
            chara.add_status(CharaStatus::Poisoned);
            game_log!("poisoned"; chara=chara);
        }
    }
}
