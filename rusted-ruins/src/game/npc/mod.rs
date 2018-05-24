//! Functions for NPC's AI and actions

pub mod map_search;

use array2d::*;
use common::gamedata::*;
use super::Game;
use super::action;
use rng::*;

pub fn process_npc_turn(game: &mut Game, cid: CharaId) {

    {
        let chara = game.gd.chara.get(cid);
        let ai = &chara.ai;

        match ai.kind {
            NpcAIKind::None => {
                return;
            }
            NpcAIKind::NoMove => {
                return;
            }
            _ => (),
        }
    }
    
    // let pos = game.current_map.chara_pos(cid);
    let dir = Direction::new(
        *get_rng().choose(&[HDirection::Left, HDirection::None, HDirection::Right]).unwrap(),
        *get_rng().choose(&[VDirection::Up, VDirection::None, VDirection::Down]).unwrap());

    action::try_move(game, cid, dir);
}

