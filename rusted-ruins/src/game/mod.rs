mod action;
mod anim_queue;
mod animation;
pub mod building;
pub mod chara;
pub mod command;
pub mod creation;
pub mod damage;
mod debug_command;
mod dungeon_gen;
pub mod effect;
mod faction;
pub mod frequent_tex;
mod infogetter;
pub mod item;
pub mod map;
pub mod newgame;
mod npc;
pub mod party;
pub mod play_time;
pub mod playeract;
pub mod power;
pub mod quest;
mod region;
pub mod saveload;
pub mod script_exec;
pub mod script_methods;
pub mod shop;
pub mod site;
mod skill;
mod target;
mod time;
mod town;
mod turnloop;
pub mod view;

pub use self::animation::Animation;
pub use self::command::Command;
pub use self::infogetter::InfoGetter;
pub use self::playeract::DoPlayerAction;
use self::script_exec::ScriptState;
// pub use self::script::TalkText;
// use self::script::*;
pub use self::target::Target;
use common::gamedata::*;
use common::gobj;
use common::objholder::ScriptIdx;
use geom::Coords;
use script::{ScriptEngine, TalkText};
use std::collections::{HashMap, VecDeque};
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
    destroy_anim_queued: bool,
    dialog_open_request: Option<DialogOpenRequest>,
    ui_request: VecDeque<UiRequest>,
    closures: HashMap<ClosureTrigger, Vec<GameClosure>>,
    pub se: ScriptEngine,
    script_state: ScriptState,
    /// Player's current target of shot and similer actions
    target_chara: Option<CharaId>,
    save_dir: Option<PathBuf>,
    pub view_map: view::ViewMap,
    pub frequent_tex: self::frequent_tex::FrequentTextures,
}

impl Game {
    pub fn new(gd: GameData, se: ScriptEngine) -> Game {
        let save_dir = self::saveload::get_each_save_dir(&gd);

        rng::reseed(crate::config::CONFIG.fix_rand);

        Game {
            gd,
            state: GameState::PlayerTurn,
            anim_queue: anim_queue::AnimQueue::default(),
            destroy_anim_queued: false,
            dialog_open_request: None,
            ui_request: VecDeque::new(),
            closures: HashMap::new(),
            se,
            script_state: ScriptState::default(),
            target_chara: None,
            save_dir: Some(save_dir),
            view_map: view::ViewMap::new(),
            frequent_tex: self::frequent_tex::FrequentTextures::new(),
        }
    }

    /// Create empty Game. This is used before starting actual gameplay.
    pub fn empty(se: ScriptEngine) -> Game {
        Game {
            gd: GameData::empty(),
            state: GameState::PlayerTurn,
            script_state: ScriptState::default(),
            anim_queue: anim_queue::AnimQueue::default(),
            destroy_anim_queued: false,
            dialog_open_request: None,
            ui_request: VecDeque::new(),
            closures: HashMap::new(),
            se,
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
        use extrait::PlayTimeExt;
        self.gd.play_time.update();
        map::update_observed_map(self);
    }

    /// Update some parameters before starting player's turn
    pub fn update_before_player_turn(&mut self) {
        time::update_time(self);
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

    pub fn push_closure(&mut self, trigger: ClosureTrigger, closure: GameClosure) {
        self.closures
            .entry(trigger)
            .or_insert(Vec::new())
            .push(closure);
    }

    pub fn pop_closure(&mut self, trigger: ClosureTrigger) -> Vec<GameClosure> {
        self.closures.remove(&trigger).unwrap_or_default()
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

    /// Set target chara by position.
    /// If given tile position is empty, returns false.
    pub fn set_target(&mut self, pos: Coords) -> bool {
        let map = self.gd.get_current_map();
        if let Some(cid) = map.get_chara(pos) {
            let player = self.gd.chara.get(CharaId::Player);
            let target = self.gd.chara.get(cid);
            game_log!("target-chara"; chara=player, target=target);
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
        callback: Box<dyn FnMut(&mut DoPlayerAction<'_>, bool)>,
        msg: String,
    },
    Talk {
        cid: Option<CharaId>,
        talk_text: TalkText,
    },
    ItemInfo {
        il: ItemLocation,
    },
    BuildObj {
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
    RegisterAsShortcut {
        shortcut: ActionShortcut,
    },
    PickUpItem,
    QuestOffer,
    QuestReport,
    InstallAbilitySlot,
    InstallExtendSlot,
    InsertModule,
    GameOver,
}

/// User interface request from game
pub enum UiRequest {
    StopCentering,
    StartTargeting {
        effect: Effect,
        callback: Box<dyn Fn(&mut DoPlayerAction<'_>, self::target::Target) + 'static>,
    },
}

pub type GameClosure = Box<dyn FnOnce(&mut Game)>;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum ClosureTrigger {
    CharaRemove(CharaId),
}

pub mod extrait {
    pub use super::chara::status::{CharaStatusExt, CharaStatusOperation};
    pub use super::chara::CharaExt;
    pub use super::effect::BasePowerExt;
    pub use super::faction::FactionExt;
    pub use super::infogetter::InfoGetter;
    pub use super::item::{GameDataItemExt, ItemExt, ItemListExt};
    pub use super::map::MapExt;
    pub use super::party::GameDataPartyExt;
    pub use super::play_time::PlayTimeExt;
    pub use super::site::SiteExt;
    pub use super::skill::SkillListExt;
}
