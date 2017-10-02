
use common::gamedata::chara::{Chara, CharaParams};
use common::basic::{TURN_RESOLUTION, WAIT_TIME_DEFAULT};
use super::{Game, GameState};
use super::chara::{CharaIdIter, CharaId};
use super::npc::process_npc_turn;

/// Contains interruption data of charaid iterator
#[derive(Clone, Copy)]
pub struct TurnLoopData(Option<CharaIdIter>);

impl TurnLoopData {
    pub fn new() -> TurnLoopData {
        TurnLoopData(None)
    }
}
    
/// Advance game time until player's waittime becomes 0
pub fn turn_loop(game: &mut Game, interrupt_data: TurnLoopData) -> TurnLoopData {
    let ciditer = match interrupt_data.0 {
        Some(ciditer) => ciditer,
        None => game.chara_holder.id_iter_on_map(),
    };
    
    'turn_loop:
    loop {
        for chara_id in ciditer {
            if decrease_wait_time(game.chara_holder.get_mut(chara_id)) {
                process_npc_turn(game, chara_id);
                
                // If an animation is started, turn_loop is interrupted
                if !game.anim_queue.is_empty() {
                    return TurnLoopData(Some(ciditer));
                }
            }
        }

        // If player's wait time becomes 0, player turn now.
        if decrease_wait_time(&mut game.chara_holder.player) {
            game.state = GameState::PlayerTurn;
            break;
        }
    }
    TurnLoopData(None)
}

// Returns true if chara's wait_time becomes 0
fn decrease_wait_time(chara: &mut Chara) -> bool {
    let spd = chara.params.spd;
    let mut wt = chara.wait_time;

    wt -= spd as f32 / TURN_RESOLUTION as f32;

    if wt < 0.0 {
        wt += WAIT_TIME_DEFAULT;
        if wt < 0.0 { wt = 0.0; }
        chara.wait_time = wt;
        trace!("Turn Processing: {} (wt={})", chara.name, chara.wait_time);
        true
    }else{
        chara.wait_time = wt;
        false
    }
}

