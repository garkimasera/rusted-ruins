
use array2d::*;
use common::gamedata::chara::CharaId;
use super::Game;

/// Helper functions to get information for drawing or other displaying
pub trait InfoGetter {
    fn player_pos(&self) -> Vec2d;
}

impl InfoGetter for Game {
    fn player_pos(&self) -> Vec2d {
        self.gd.get_current_map().chara_pos(CharaId::Player).expect("Internal Error: Player position undefined")
    }
}

