
pub mod playeract;
pub mod item;
mod npc;
mod action;
mod command;
mod region;
pub mod site;
mod map;
pub mod chara;
mod infogetter;
mod animation;
pub mod newgame;
mod combat;
mod town;
mod turnloop;
pub mod talk;
pub mod view;

use std::collections::VecDeque;
use std::borrow::Cow;
use common::gamedata;
use common::gamedata::GameData;
use common::gamedata::chara::CharaId;
pub use self::command::Command;
pub use self::infogetter::InfoGetter;
pub use self::animation::Animation;
pub use self::playeract::DoPlayerAction;
pub use self::talk::TalkManager;
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
    pub view_map: view::ViewMap,
}

impl Game {
    pub fn new(gd: GameData) -> Game {
        let game = Game {
            gd: gd,
            state: GameState::PlayerTurn,
            turn_loop_data: TurnLoopData::new(),
            anim_queue: VecDeque::new(),
            dialog_open_request: None,
            dying_charas: Vec::new(),
            view_map: view::ViewMap::new(),
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
            view_map: view::ViewMap::new(),
        }
    }

    pub fn get_state(&self) -> GameState {
        self.state
    }

    pub fn advance_turn(&mut self) {
        ::log::new_line(); // Insert break to log lines
        turnloop::turn_loop(self);
    }

    /// Update drawing data
    pub fn update_before_drawing(&mut self) {
        map::update_observed_map(self);
    }

    /// Update some parameters before starting player's turn
    pub fn update_before_player_turn(&mut self) {
        view::update_view_map(self);
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
    YesNo { callback: Box<FnMut(&mut DoPlayerAction, bool)>, msg: Cow<'static, str> },
    Talk { chara_talk: gamedata::chara::CharaTalk, cid: gamedata::chara::CharaId },
    GameOver,
}

pub mod extrait {
    pub use super::chara::CharaEx;
    pub use super::item::ItemEx;
    pub use super::site::SiteEx;
}
