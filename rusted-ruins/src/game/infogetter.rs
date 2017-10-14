
use array2d::*;
use common::gamedata::GameData;
use common::gamedata::chara::CharaId;

/// Helper functions to get information for event processing and drawing
pub trait InfoGetter {
    fn player_pos(&self) -> Vec2d;
}

impl InfoGetter for GameData {
    fn player_pos(&self) -> Vec2d {
        self.get_current_map().chara_pos(CharaId::Player).expect("Internal Error: Player position undefined")
    }
}

