
use array2d::*;
use common::gamedata::chara::CharaId;
use super::Game;
use super::action;
use rng::*;

pub fn process_npc_turn(game: &mut Game, cid: CharaId) {
    // let pos = game.current_map.chara_pos(cid);
    let dir = Direction::new(
        *get_rng().choose(&[HDirection::Left, HDirection::None, HDirection::Right]).unwrap(),
        *get_rng().choose(&[VDirection::Up, VDirection::None, VDirection::Down]).unwrap());

    action::try_move(game, cid, dir);
}

