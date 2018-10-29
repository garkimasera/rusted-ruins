
pub mod playeract;
pub mod item;
pub mod frequent_tex;
mod npc;
mod action;
mod command;
mod region;
pub mod site;
mod map;
pub mod chara;
mod skill;
mod infogetter;
mod animation;
mod anim_queue;
pub mod newgame;
mod combat;
mod town;
mod turnloop;
pub mod view;
mod script;
mod eval_expr;
pub mod shop;
mod dungeon_gen;

use std::borrow::Cow;
use array2d::Vec2d;
use common::gamedata::*;
pub use self::command::Command;
pub use self::infogetter::InfoGetter;
pub use self::animation::Animation;
pub use self::playeract::DoPlayerAction;
pub use self::script::TalkText;
use self::turnloop::TurnLoopData;
use self::script::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    /// In this state, call advance_turn()
    WaitingForNextTurn,
    PlayerTurn,
}

/// Holds all game state.
/// The difference to GameData is that Game includes temporary data in this process.
pub struct Game {
    pub gd: GameData,
    state: GameState,
    turn_loop_data: TurnLoopData,
    anim_queue: anim_queue::AnimQueue,
    dialog_open_request: Option<DialogOpenRequest>,
    dying_charas: Vec<CharaId>,
    script: Option<ScriptEngine>,
    /// Player's current target of shot and similer actions
    target_chara: Option<CharaId>,
    pub view_map: view::ViewMap,
    pub frequent_tex: self::frequent_tex::FrequentTextures,
}

impl Game {
    pub fn new(gd: GameData) -> Game {
        let game = Game {
            gd: gd,
            state: GameState::PlayerTurn,
            turn_loop_data: TurnLoopData::new(),
            anim_queue: anim_queue::AnimQueue::default(),
            dialog_open_request: None,
            dying_charas: Vec::new(),
            script: None,
            target_chara: None,
            view_map: view::ViewMap::new(),
            frequent_tex: self::frequent_tex::FrequentTextures::new(),
        };
        
        game
    }

    /// Create empty Game. This is used before starting actual gameplay.
    pub fn empty() -> Game {        
        Game {
            gd: GameData::empty(),
            state: GameState::PlayerTurn,
            turn_loop_data: TurnLoopData::new(),
            anim_queue: anim_queue::AnimQueue::default(),
            dialog_open_request: None,
            dying_charas: Vec::new(),
            script: None,
            target_chara: None,
            view_map: view::ViewMap::new(),
            frequent_tex: self::frequent_tex::FrequentTextures::new(),
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
        self.anim_queue.pop()
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

    pub fn start_script(&mut self, id: &str, cid: Option<CharaId>) {
        self.script = Some(ScriptEngine::new(id, cid));
        self.advance_script(None);
    }

    /// Advance current script.
    /// When called by advance_talk, give player's choice.
    pub fn advance_script(&mut self, choice: Option<Option<u32>>) -> AdvanceScriptResult {
        let result = { // TODO: may be able to simplify if NLL enabled
            let script = self.script.as_mut().expect("advance_script() when script is None");
            if let Some(choice) = choice {
                script.continue_talk(&mut self.gd, choice)
            } else {
                script.exec(&mut self.gd)
            }
        };
        
        match result {
            ExecResult::Quit => {
                self.script = None;
                AdvanceScriptResult::Quit
            }
            ExecResult::Talk(cid, talk_text, need_open_talk_dialog) => {
                if need_open_talk_dialog {
                    self.request_dialog_open(
                        DialogOpenRequest::Talk { cid, talk_text }
                    );
                }
                AdvanceScriptResult::UpdateTalkText(talk_text)
            }
            ExecResult::ShopBuy(cid) => {
                self.request_dialog_open(DialogOpenRequest::ShopBuy { cid });
                AdvanceScriptResult::Continue
            }
            ExecResult::ShopSell => {
                self.request_dialog_open(DialogOpenRequest::ShopSell);
                AdvanceScriptResult::Continue
            }
        }
    }

    /// Set target chara by position.
    /// If given tile position is empty, returns false.
    pub fn set_target(&mut self, pos: Vec2d) -> bool {
        
        let map = self.gd.get_current_map();
        if let Some(cid) = map.get_chara(pos) {
            let player = self.gd.chara.get(CharaId::Player);
            let target = self.gd.chara.get(cid);
            game_log_i!("target-chara"; chara=player, target=target);
            self.target_chara = Some(cid);
            true
        } else {
            false
        }
    }
}

pub enum DialogOpenRequest {
    YesNo { callback: Box<FnMut(&mut DoPlayerAction, bool)>, msg: Cow<'static, str> },
    Talk { cid: CharaId, talk_text: TalkText },
    ShopBuy { cid: CharaId },
    ShopSell,
    GameOver,
}

pub enum AdvanceScriptResult {
    Continue,
    UpdateTalkText(TalkText),
    Quit,
}

pub mod extrait {
    pub use super::chara::CharaEx;
    pub use super::item::ItemEx;
    pub use super::item::ItemListEx;
    pub use super::map::MapEx;
    pub use super::site::SiteEx;
    pub use super::chara::status::{CharaStatusOperation, CharaStatusEx};
    pub use super::skill::SkillListEx;
}
