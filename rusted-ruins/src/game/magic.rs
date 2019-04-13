use crate::game::Game;
use common::gamedata::*;

pub fn do_magic(game: &mut Game, cid: CharaId, me: MagicalEffect, power: f32) {
    match me {
        MagicalEffect::None => {
            return;
        }
        MagicalEffect::Arrow => {
            arrow(game, cid, power);
        }
    }
}

pub fn arrow(game: &mut Game, cid: CharaId, power: f32) {
    if game.target_chara.is_none() {
        let player = game.gd.chara.get(CharaId::Player);
        game_log_i!("no-target"; chara=player);
        return;
    }
    let target_id = game.target_chara.unwrap();
    let start = game.gd.get_current_map().chara_pos(cid).unwrap();
    let target = game.gd.get_current_map().chara_pos(target_id).unwrap();

    game.anim_queue.push_magic_arrow(start, target);
}
