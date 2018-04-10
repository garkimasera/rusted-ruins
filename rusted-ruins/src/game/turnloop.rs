
use std::collections::VecDeque;
use common::gamedata::chara::{Chara, CharaId};
use common::basic::WAIT_TIME_START;
use super::{Game, GameState};
use super::chara::CharaEx;
use super::chara::preturn::preturn;
use super::npc::process_npc_turn;
use super::DialogOpenRequest;

/// Contains interruption data of charaid iterator
#[derive(Clone)]
pub struct TurnLoopData(VecDeque<CharaId>);

impl TurnLoopData {
    pub fn new() -> TurnLoopData {
        TurnLoopData(VecDeque::new())
    }
}
    
/// Advance game time until player's waittime becomes 0
pub fn turn_loop(game: &mut Game) {
    remove_dying_charas(game);
    
    'turn_loop:
    loop {
        // Add chara ids that are on the current map
        if game.turn_loop_data.0.is_empty() {
            let map = game.gd.get_current_map();
            for cid in map.iter_charaid() {
                match *cid {
                    CharaId::OnMap { .. } => {
                        game.turn_loop_data.0.push_back(*cid);
                    },
                    CharaId::Player => (),
                }
            }
        }
        
        while let Some(cid) = game.turn_loop_data.0.pop_front() {
            
            let is_process_npc_turn = {
                let chara = game.gd.chara.get_mut(cid);
                decrease_wait_time(chara)
            };
            
            if is_process_npc_turn {
                if preturn(game, cid) {
                    process_npc_turn(game, cid);
                }
                
                // If an animation is started, turn_loop is interrupted
                if !game.anim_queue.is_empty() {
                    return;
                }
            }
        }

        // If player's wait time becomes 0, player turn now.
        if decrease_wait_time(&mut game.gd.chara.get_mut(CharaId::Player)) {
            if preturn(game, CharaId::Player) {
                game.state = GameState::PlayerTurn;
                game.update_before_player_turn();
                break;
            }
        }
    }
}

/// Returns true if chara's wait_time becomes 0
fn decrease_wait_time(chara: &mut Chara) -> bool {
    let spd = chara.params.spd;

    if chara.wait_time < spd {
        if spd < WAIT_TIME_START {
            chara.wait_time += WAIT_TIME_START - spd;
        } else {
            warn!("Character's speed is over {}", WAIT_TIME_START);
            chara.wait_time = 0;
        }
        trace!("Turn Processing: {} (wt={})", chara.get_name(), chara.wait_time);
        true
    }else{
        chara.wait_time -= spd;
        false
    }
}

/// Dying chara is removed before new turn processing
fn remove_dying_charas(game: &mut Game) {
    while let Some(cid) = game.dying_charas.pop() {
        if cid == CharaId::Player {
            game.request_dialog_open(DialogOpenRequest::GameOver);
            return;
        }
        // Remove from gamedata
        game.gd.remove_chara(cid);
        // Remove from action queue
        let action_queue = &mut game.turn_loop_data.0;
        if let Some((i, _)) = action_queue.iter().enumerate().find(|&(_, a)| *a == cid) {
            action_queue.remove(i);
        }
        // If the current target is cid, remove it
        if game.target_chara == Some(cid) {
            game.target_chara = None;
        }
    }
}

