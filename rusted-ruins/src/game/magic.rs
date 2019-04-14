use super::combat::{attack_target, AttackParams, DamageKind};
use crate::game::Game;
use common::gamedata::*;

pub fn do_magic(game: &mut Game, cid: CharaId, me: MagicalEffect, power: f64) {
    match me {
        MagicalEffect::None => {
            return;
        }
        MagicalEffect::Arrow => {
            arrow(game, cid, power);
        }
    }
}

pub fn arrow(game: &mut Game, cid: CharaId, power: f64) {
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

    let attack_power = power;

    let attack_params = AttackParams {
        attacker_id: Some(cid),
        kind: DamageKind::RangedAttack,
        element: Element::Physical,
        attack_power,
    };

    attack_target(game, attack_params, target_id);
}
