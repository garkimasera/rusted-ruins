
use array2d::*;
use common::gamedata::*;

/// Helper functions to get information for event processing and drawing
pub trait InfoGetter {
    fn player_pos(&self) -> Vec2d;
    /// Get player's (maxhp, hp)
    fn player_hp(&self) -> (i32, i32);
    /// Get current map size
    fn map_size(&self) -> (u32, u32);
    /// Player's current tile is entrance/exit or not
    fn on_map_entrance(&self) -> bool;
    /// Return item list in the tile that player stands on
    fn item_on_player_tile(&self) -> Option<&::common::gamedata::item::ItemList>;
    /// Return any item exist or not on player tile
    fn is_item_on_player_tile(&self) -> bool;
    /// Judge given map is open-air or not
    fn is_open_air(&self, mid: MapId) -> bool;
}

impl InfoGetter for GameData {
    fn player_pos(&self) -> Vec2d {
        self.get_current_map().chara_pos(CharaId::Player).expect("Internal Error: Player position undefined")
    }
    
    fn player_hp(&self) -> (i32, i32) {
        let player =self.chara.get(CharaId::Player);
        (player.params.max_hp, player.hp)
    }

    fn map_size(&self) -> (u32, u32) {
        let map = self.get_current_map();
        (map.w, map.h)
    }
    
    fn on_map_entrance(&self) -> bool {
        use common::gamedata::map::SpecialTileKind;
        
        let map = self.get_current_map();
        let tile = &map.tile[self.player_pos()];
        match tile.special {
            SpecialTileKind::Stairs { .. } | SpecialTileKind::SiteSymbol { .. } => {
                true
            }
            _ => false,
        }
    }

    fn item_on_player_tile(&self) -> Option<&::common::gamedata::item::ItemList> {
        let player_pos = self.player_pos();
        self.get_current_map().tile[player_pos].item_list.as_ref()
    }

    fn is_item_on_player_tile(&self) -> bool {
        let list = self.item_on_player_tile();
        !(list.is_none() || list.unwrap().is_empty())
    }

    fn is_open_air(&self, mid: MapId) -> bool {
        match mid {
            MapId::SiteMap { sid, floor } => {
                match sid.kind {
                    SiteKind::AutoGenDungeon => false,
                    SiteKind::Town => {
                        floor == 0
                    }
                    SiteKind::Other => false,
                }
            }
            MapId::RegionMap { .. } => true
        }
    }
}

