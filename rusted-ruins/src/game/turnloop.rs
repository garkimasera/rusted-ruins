
use common::gamedata::*;
use rules::RULES;
use crate::text::ToText;
use super::{Game, GameState};
use super::chara::CharaEx;
use super::chara::preturn::preturn;
use super::npc::process_npc_turn;
use super::DialogOpenRequest;

pub fn turn_loop(game: &mut Game) {
    loop {
        remove_dying_charas(game);
        let (cid, _advanced_clock) = decrease_wait_time(game);

        if preturn(game, cid) {
            if cid == CharaId::Player {
                game.state = GameState::PlayerTurn;
                game.update_before_player_turn();
                return;
            } else {
                process_npc_turn(game, cid);
                if !game.anim_queue.is_empty() {
                    return;
                }
            }
        }
    }
}

fn decrease_wait_time(game: &mut Game) -> (CharaId, u32) {
    let cids_on_map = game.gd.get_charas_on_map();
    let min_wt = cids_on_map.iter().map(|&cid| game.gd.chara.get(cid).wait_time).min().unwrap();

    if min_wt > 0 {
        for &cid in &cids_on_map {
            game.gd.chara.get_mut(cid).wait_time -= min_wt;
        }
    }

    for &cid in &cids_on_map {
        let chara = game.gd.chara.get_mut(cid);
        if chara.wait_time == 0 {
            chara.reset_wait_time();
            return (cid, min_wt);
        }
    }
    unreachable!()
}

/// Dying chara is removed before new turn processing
fn remove_dying_charas(game: &mut Game) {
    for &cid in &game.gd.get_charas_on_map() {
        let chara = game.gd.chara.get(cid);
        if chara.hp <= 0 {
            if cid == CharaId::Player {
                game.request_dialog_open(DialogOpenRequest::GameOver);
                return;
            }
            // Remove dying chara
            game.gd.remove_chara(cid);
            // If the current target is cid, remove it
            if game.target_chara == Some(cid) {
                game.target_chara = None;
            }
        }
    }
}

