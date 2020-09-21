use super::combat::{attack_target, AttackParams, DamageKind};
use crate::game::Game;
use common::gamedata::*;

pub fn process_effect(game: &mut Game, cid: CharaId, effect: &Effect, power: f64) {
    for effect_kind in &effect.kind {
        match effect_kind {
            EffectKind::Ranged { element } => {
                ranged_attack(game, cid, power, *element);
            }
            _ => (),
        }
    }
}

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
