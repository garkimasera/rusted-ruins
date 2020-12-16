mod action;
mod anim_queue;
mod animation;
mod building;
pub mod chara;
pub mod command;
pub mod creation;
pub mod damage;
mod debug_command;
mod dungeon_gen;
pub mod effect;
mod eval_expr;
pub mod frequent_tex;
mod infogetter;
pub mod item;
pub mod map;
pub mod newgame;
mod npc;
pub mod playeract;
pub mod quest;
mod region;
pub mod saveload;
mod script;
pub mod shop;
pub mod site;
mod skill;
mod target;
mod town;
mod turnloop;
pub mod view;

pub use self::animation::Animation;
pub use self::command::Command;
pub use self::infogetter::InfoGetter;
pub use self::playeract::DoPlayerAction;
pub use self::script::TalkText;
use self::script::*;
pub use self::target::Target;
use common::gamedata::*;
use common::gobj;
use common::objholder::ScriptIdx;
use geom::Vec2d;
use std::collections::VecDeque;
use std::path::PathBuf;

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
    anim_queue: anim_queue::AnimQueue,
    pub damage_view: damage::DamageView,
    destroy_anim_queued: bool,
    dialog_open_request: Option<DialogOpenRequest>,
    ui_request: VecDeque<UiRequest>,
    script: Option<ScriptEngine>,
    /// Player's current target of shot and similer actions
    target_chara: Option<CharaId>,
    save_dir: Option<PathBuf>,
    pub view_map: view::ViewMap,
    pub frequent_tex: self::frequent_tex::FrequentTextures,
}

impl Game {
    pub fn new(gd: GameData) -> Game {
        let save_dir = self::saveload::get_each_save_dir(&gd);

        rng::reseed(crate::config::CONFIG.fix_rand);

        Game {
            gd,
            state: GameState::PlayerTurn,
            anim_queue: anim_queue::AnimQueue::default(),
            damage_view: damage::DamageView::new(),
            destroy_anim_queued: false,
            dialog_open_request: None,
            ui_request: VecDeque::new(),
            script: None,
            target_chara: None,
            save_dir: Some(save_dir),
            view_map: view::ViewMap::new(),
            frequent_tex: self::frequent_tex::FrequentTextures::new(),
        }
    }

    /// Create empty Game. This is used before starting actual gameplay.
    pub fn empty() -> Game {
        Game {
            gd: GameData::empty(),
            state: GameState::PlayerTurn,
            anim_queue: anim_queue::AnimQueue::default(),
            damage_view: damage::DamageView::new(),
            destroy_anim_queued: false,
            dialog_open_request: None,
            ui_request: VecDeque::new(),
            script: None,
            target_chara: None,
            save_dir: None,
            view_map: view::ViewMap::new(),
            frequent_tex: self::frequent_tex::FrequentTextures::new(),
        }
    }

    pub fn get_state(&self) -> GameState {
        self.state
    }

    pub fn advance_turn(&mut self) {
        crate::log::new_line(); // Insert break to log lines
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

    pub fn pop_ui_request(&mut self) -> Option<UiRequest> {
        self.ui_request.pop_front()
    }

    pub fn request_dialog_open(&mut self, req: DialogOpenRequest) {
        self.dialog_open_request = Some(req);
    }

    pub fn pop_dialog_open_request(&mut self) -> Option<DialogOpenRequest> {
        if self.dialog_open_request.is_some() {
            std::mem::replace(&mut self.dialog_open_request, None)
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
        let script = self
            .script
            .as_mut()
            .expect("advance_script() when script is None");
        let result = if let Some(choice) = choice {
            script.continue_talk(&mut self.gd, choice)
        } else {
            script.exec(&mut self.gd)
        };

        match result {
            ExecResult::Quit => {
                self.script = None;
                AdvanceScriptResult::Quit
            }
            ExecResult::Talk(cid, talk_text, need_open_talk_dialog) => {
                if need_open_talk_dialog {
                    self.request_dialog_open(DialogOpenRequest::Talk { cid, talk_text });
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
            ExecResult::Quest => {
                self.request_dialog_open(DialogOpenRequest::Quest);
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

    pub fn target_chara(&self) -> Option<CharaId> {
        self.target_chara
    }

    pub fn clear_target(&mut self) {
        self.target_chara = None;
    }

    /// Start new generated game
    pub fn start_new_game(&mut self) {
        const START_SCRIPT_ID: &str = "!start";
        if gobj::id_to_idx_checked::<ScriptIdx>(START_SCRIPT_ID).is_some() {
            self.start_script(START_SCRIPT_ID, None);
        }
    }

    pub fn end_game(&mut self) {
        self.clean_save_data()
    }
}

pub enum DialogOpenRequest {
    YesNo {
        callback: Box<dyn FnMut(&mut DoPlayerAction, bool)>,
        msg: String,
    },
    Talk {
        cid: Option<CharaId>,
        talk_text: TalkText,
    },
    ItemInfo {
        il: ItemLocation,
    },
    CharaStatus {
        cid: CharaId,
    },
    Read {
        title: String,
    },
    ShopBuy {
        cid: CharaId,
    },
    ShopSell,
    PickUpItem,
    Quest,
    GameOver,
}

/// User interface request from game
pub enum UiRequest {
    StopCentering,
    StartTargeting {
        effect: Effect,
        callback: Box<dyn Fn(&mut DoPlayerAction, self::target::Target) + 'static>,
    },
}

pub enum AdvanceScriptResult {
    Continue,
    UpdateTalkText(TalkText),
    Quit,
}

pub mod extrait {
    pub use super::chara::status::{CharaStatusEx, CharaStatusOperation};
    pub use super::chara::CharaEx;
    pub use super::infogetter::InfoGetter;
    pub use super::item::ItemEx;
    pub use super::item::ItemListEx;
    pub use super::map::MapEx;
    pub use super::site::SiteEx;
    pub use super::skill::SkillListEx;
}
