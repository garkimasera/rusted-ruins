
pub mod playeract;
pub mod item;
mod npc;
mod action;
mod command;
mod infogetter;
mod animation;
mod combat;
mod turnloop;

use std::collections::VecDeque;
use common::gamedata::GameData;
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
}

impl Game {
    pub fn new() -> Game {
        let mut game = Game {
            gd: GameData::empty(),
            state: GameState::WaitingForNextTurn,
            turn_loop_data: TurnLoopData::new(),
            anim_queue: VecDeque::new(),
        };
        use common::gamedata;
        
        let map = gamedata::map::Map::new(10, 10);
        let site = gamedata::site::Site::new("はじまり");
        let sid = game.gd.add_site(site, gamedata::site::SiteKind::Start);
        let mid = game.gd.add_map(map, sid);

        let mut chara = gamedata::chara::Chara::default();
        chara.params.spd = 100;
        chara.params.str = 25;
        chara.rel = gamedata::chara::Relationship::ALLY;
        game.gd.add_chara_to_map(chara, gamedata::chara::CharaKind::Player, mid, ::array2d::Vec2d(0, 0));

        let mut chara = gamedata::chara::Chara::default();
        chara.params.spd = 100;
        chara.params.str = 20;
        chara.rel = gamedata::chara::Relationship::HOSTILE;
        game.gd.add_chara_to_map(chara, gamedata::chara::CharaKind::OnMap, mid, ::array2d::Vec2d(2, 2));
        
        game
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

