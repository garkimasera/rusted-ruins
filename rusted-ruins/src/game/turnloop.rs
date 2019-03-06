use super::chara::preturn::preturn;
use super::chara::CharaEx;
use super::npc::process_npc_turn;
use super::DialogOpenRequest;
use super::{Game, GameState};
use common::basic::WAIT_TIME_NUMERATOR;
use common::gamedata::*;
use rules::RULES;

pub fn turn_loop(game: &mut Game) {
    loop {
        if remove_dying_charas(game) {
            game.state = GameState::PlayerTurn;
            return;
        }

        let (cid, advanced_clock) = decrease_wait_time(game);

        advance_game_time(game, advanced_clock);
        if !game.anim_queue.is_empty() {
            return;
        }

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
    let min_wt = cids_on_map
        .iter()
        .map(|&cid| game.gd.chara.get(cid).wait_time)
        .min()
        .unwrap();

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

/// Dying chara is removed before new turn processing.
/// If returns true, player died.
fn remove_dying_charas(game: &mut Game) -> bool {
    for &cid in &game.gd.get_charas_on_map() {
        let chara = game.gd.chara.get(cid);
        if chara.hp <= 0 {
            if cid == CharaId::Player {
                game.request_dialog_open(DialogOpenRequest::GameOver);
                return true;
            }
            // Remove dying chara
            game.gd.remove_chara(cid);
            // If the current target is cid, remove it
            if game.target_chara == Some(cid) {
                game.target_chara = None;
            }
        }
    }
    false
}

fn advance_game_time(game: &mut Game, advanced_clock: u32) {
    let mid = game.gd.get_current_mapid();
    let minutes_per_turn = if mid.is_region_map() {
        RULES.params.minutes_per_turn_region
    } else {
        RULES.params.minutes_per_turn_normal
    };
    const AVERAGE_CLOCK_PER_TURN: u32 = WAIT_TIME_NUMERATOR / 100;
    let advanced_secs =
        minutes_per_turn * 60.0 * advanced_clock as f32 / AVERAGE_CLOCK_PER_TURN as f32;
    game.gd.time.advance(advanced_secs as u64)
}
