
use array2d::*;
use common::gamedata::GameData;
use common::gamedata::chara::CharaId;

/// Helper functions to get information for event processing and drawing
pub trait InfoGetter {
    fn player_pos(&self) -> Vec2d;
    /// Get player's (maxhp, hp)
    fn player_hp(&self) -> (i32, i32);
    /// Player's current tile is entrance/exit or not
    fn on_map_entrance(&self) -> bool;
}

impl InfoGetter for GameData {
    fn player_pos(&self) -> Vec2d {
        self.get_current_map().chara_pos(CharaId::Player).expect("Internal Error: Player position undefined")
    }

    
    fn player_hp(&self) -> (i32, i32) {
        let player =self.chara.get(CharaId::Player);
        (player.params.max_hp, player.hp)
    }
    
    fn on_map_entrance(&self) -> bool {
        use common::gamedata::map::SpecialTileKind;
        
        let map = self.get_current_map();
        let tile = &map.tile[self.player_pos()];
        match tile.special {
            SpecialTileKind::UpStairs | SpecialTileKind::DownStairs => {
                return true;
            },
            _ => (),
        }
        false
    }
}

