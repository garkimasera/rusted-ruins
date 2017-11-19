
pub mod playeract;
pub mod item;
mod npc;
mod action;
mod command;
mod site;
mod map;
mod chara;
mod infogetter;
mod animation;
mod newgame;
mod combat;
mod turnloop;

use std::collections::VecDeque;
use common::gamedata::GameData;
use common::gamedata::chara::CharaId;
pub use self::command::Command;
pub use self::infogetter::InfoGetter;
pub use self::animation::Animation;
pub use self::playeract::DoPlayerAction;
use self::turnloop::TurnLoopData;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    /// In this state, call advance_turn()
    WaitingForNextTurn,
    PlayerTurn,
}

pub struct Game {
    pub gd: GameData,
    state: GameState,
    turn_loop_data: TurnLoopData,
    anim_queue: VecDeque<Animation>,
    dying_charas: Vec<CharaId>,
}

impl Game {
    pub fn new() -> Game {
        let new_gamedata = self::newgame::create_newgame();
        let game = Game {
            gd: new_gamedata,
            state: GameState::WaitingForNextTurn,
            turn_loop_data: TurnLoopData::new(),
            anim_queue: VecDeque::new(),
            dying_charas: Vec::new(),
        };
        
        game
    }

    pub fn empty() -> Game {        
        Game {
            gd: GameData::empty(),
            state: GameState::PlayerTurn,
            turn_loop_data: TurnLoopData::new(),
            anim_queue: VecDeque::new(),
            dying_charas: Vec::new(),
        }
    }

    pub fn get_state(&self) -> GameState {
        self.state
    }

    pub fn advance_turn(&mut self) {
        ::log::new_line(); // Insert break to log lines
        turnloop::turn_loop(self);
    }

    pub fn finish_player_turn(&mut self) {
        assert!(self.state == GameState::PlayerTurn);
        self.state = GameState::WaitingForNextTurn;
    }

    pub fn pop_animation(&mut self) -> Option<Animation> {
        self.anim_queue.pop_front()
    }
}

