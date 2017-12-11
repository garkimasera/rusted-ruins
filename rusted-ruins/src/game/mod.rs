
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
pub mod talk;

use std::collections::VecDeque;
use common::gamedata;
use common::gamedata::GameData;
use common::gamedata::chara::CharaId;
pub use self::command::Command;
pub use self::infogetter::InfoGetter;
pub use self::animation::Animation;
pub use self::playeract::DoPlayerAction;
pub use self::talk::TalkStatus;
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
    dialog_open_request: Option<DialogOpenRequest>,
    dying_charas: Vec<CharaId>,
}

impl Game {
    pub fn new() -> Game {
        let new_gamedata = self::newgame::create_newgame();
        let game = Game {
            gd: new_gamedata,
            state: GameState::PlayerTurn,
            turn_loop_data: TurnLoopData::new(),
            anim_queue: VecDeque::new(),
            dialog_open_request: None,
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
            dialog_open_request: None,
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

    pub fn request_dialog_open(&mut self, req: DialogOpenRequest) {
        self.dialog_open_request = Some(req);
    }

    pub fn pop_dialog_open_request(&mut self) -> Option<DialogOpenRequest> {
        if self.dialog_open_request.is_some() {
            ::std::mem::replace(&mut self.dialog_open_request, None)
        } else {
            None
        }
    }
}

pub enum DialogOpenRequest {
    Talk { chara_talk: gamedata::chara::CharaTalk, cid: gamedata::chara::CharaId },
}
