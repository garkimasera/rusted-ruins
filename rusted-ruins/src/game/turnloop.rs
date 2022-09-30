use super::chara::preturn::preturn;
use super::chara::CharaExt;
use super::npc::process_npc_turn;
use super::{ClosureTrigger, DialogOpenRequest};
use super::{Game, GameState, InfoGetter};
use common::gamedata::*;

/// Main game turn loop
pub fn turn_loop(game: &mut Game) {
    loop {
        if check_dying(game) {
            return;
        }
        if remove_dying_charas(game) {
            // If player died
            game.state = GameState::PlayerTurn;
            return;
        }

        let (cid, advanced_clock) = decrease_wait_time(game);

        super::time::advance_game_time_by_clock(game, advanced_clock);
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

/// Check dying charas and push destroy animation.
fn check_dying(game: &mut Game) -> bool {
    if game.destroy_anim_queued {
        game.destroy_anim_queued = false;
        return false;
    }

    let mut dying_chara_found = false;

    for &cid in &game.gd.get_charas_on_map() {
        let chara = game.gd.chara.get(cid);
        if chara.hp <= 0 {
            dying_chara_found = true;
            game.destroy_anim_queued = true;
            if let Some(pos) = game.gd.chara_pos(cid) {
                game.anim_queue.push_destroy(pos);
            }
        }
    }
    dying_chara_found
}

/// Dying chara is removed before new turn processing.
/// If returns true, player died.
fn remove_dying_charas(game: &mut Game) -> bool {
    for cid in game.gd.get_charas_on_map() {
        let chara = game.gd.chara.get_mut(cid);
        if chara.hp <= 0 {
            if cid == CharaId::Player {
                game.request_dialog_open(DialogOpenRequest::GameOver);
                return true;
            }
            // Process closures if registered
            for closure in game
                .pop_closure(ClosureTrigger::CharaRemove(cid))
                .into_iter()
            {
                closure(game);
            }

            // Remove dying chara
            game.gd.remove_chara_from_map(cid);
            // If the current target is cid, remove it
            if game.target_chara == Some(cid) {
                game.target_chara = None;
            }
            // Move dead party member
            if game.gd.player.party.contains(&cid) {
                game.gd.player.party.remove(&cid);
                game.gd.player.party_dead.insert(cid);
            }
        } else {
            match chara.ai.state {
                AiState::Combat { target } if target == cid => {
                    chara.ai.state = AiState::default_search();
                }
                _ => (),
            }
        }
    }
    false
}
