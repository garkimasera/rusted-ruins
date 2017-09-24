
pub mod playeract;
pub mod item;
mod map;
mod npc;
mod action;
mod command;
mod infogetter;
mod animation;
mod chara;
mod combat;
mod turnloop;

use std::collections::VecDeque;
pub use self::command::Command;
pub use self::infogetter::InfoGetter;
pub use self::animation::Animation;
pub use self::playeract::DoPlayerAction;
pub use self::chara::*;
use self::turnloop::TurnLoopData;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    /// In this state, call advance_turn()
    WaitingForNextTurn,
    PlayerTurn,
}

pub struct Game {
    pub current_map: map::Map,
    pub chara_holder: CharaHolder,
    state: GameState,
    turn_loop_data: TurnLoopData,
    anim_queue: VecDeque<Animation>,
}

impl Game {
    pub fn new() -> Game {
        let mut current_map = map::MapBuilder::new(10, 10).build();
        
        let mut game = Game {
            current_map: current_map,
            anim_queue: VecDeque::new(),
            state: GameState::WaitingForNextTurn,
            turn_loop_data: TurnLoopData::new(),
            chara_holder: CharaHolder::default(),
        };
        let mut chara = ::common::chara::Chara::default();
        chara.params.spd = 100;
        chara.params.str = 25;
        chara.rel = ::common::chara::Relationship::ALLY;
        chara::add_chara(&mut game, chara, Some(::array2d::Vec2d::new(0,0)), chara::CharaType::Player);
        let mut chara = create_chara(::common::objholder::CharaTemplateIdx(1), 10);
        chara.params.spd = 100;
        chara.rel = ::common::chara::Relationship::HOSTILE;
        add_chara(&mut game, chara, Some(::array2d::Vec2d::new(2,2)), chara::CharaType::OnMap);
        game
    }

    pub fn get_state(&self) -> GameState {
        self.state
    }

    pub fn advance_turn(&mut self) {
        ::log::new_line(); // Insert break to log lines
        let turn_loop_data = self.turn_loop_data;
        self.turn_loop_data = turnloop::turn_loop(self, turn_loop_data);
    }

    pub fn finish_player_turn(&mut self) {
        assert!(self.state == GameState::PlayerTurn);
        self.state = GameState::WaitingForNextTurn;
    }

    pub fn pop_animation(&mut self) -> Option<Animation> {
        self.anim_queue.pop_front()
    }
}

