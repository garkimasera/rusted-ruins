
use array2d::*;
use super::Game;

/// Helper functions to get information for drawing or other displaying
pub trait InfoGetter {
    fn player_pos(&self) -> Vec2d;
}

impl InfoGetter for Game {
    fn player_pos(&self) -> Vec2d {
        use game::CharaId;
        self.current_map.chara_pos(CharaId::Player).unwrap()
    }
}

