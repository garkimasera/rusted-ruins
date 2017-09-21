
use common::array2d::*;
use super::Game;
use super::chara::*;
use super::action;
use rand::{thread_rng, Rng};

pub fn process_npc_turn(game: &mut Game, cid: CharaId) {
    // let pos = game.current_map.chara_pos(cid);
    let dir = Direction::new(
        *thread_rng().choose(&[HDirection::Left, HDirection::None, HDirection::Right]).unwrap(),
        *thread_rng().choose(&[VDirection::Up, VDirection::None, VDirection::Down]).unwrap());

    action::try_move(game, cid, dir);
}

