use crate::game::extrait::*;
use common::gamedata::*;
use common::gobj;
use common::objholder::*;
use geom::*;

/// Helper functions to get information for event processing and drawing
pub trait InfoGetter {
    /// Get player's name
    fn player_name(&self) -> &str;
    /// Get player's position
    fn player_pos(&self) -> Vec2d;
    /// Get player's (maxhp, hp)
    fn player_hp(&self) -> (i32, i32);
    /// Get item location that player has
    fn player_item_location(&self, id: &str) -> Option<ItemLocation>;
    /// Get current map size
    fn map_size(&self) -> (u32, u32);
    /// Character position on the current map
    fn chara_pos(&self, cid: CharaId) -> Option<Vec2d>;
    /// Player's current tile is entrance/exit or not
    fn on_map_entrance(&self) -> bool;
    /// Return item list in the tile that player stands on
    fn item_on_player_tile(&self) -> Option<&common::gamedata::item::ItemList>;
    /// Return any item exist or not on player tile
    fn is_item_on_player_tile(&self) -> bool;
    /// Judge given map is open-air or not
    fn is_open_air(&self, mid: MapId) -> bool;
    /// Get the number of specified item player has
    fn has_item(&self, idx: ItemIdx) -> u32;
    /// Get the item location of specified item
    fn search_item(&self, idx: ItemIdx) -> Vec<ItemLocation>;
}

impl InfoGetter for GameData {
    fn player_name(&self) -> &str {
        self.chara
            .get(CharaId::Player)
            .name
            .as_ref()
            .expect("player's name is None")
    }

    fn player_pos(&self) -> Vec2d {
        self.get_current_map()
            .chara_pos(CharaId::Player)
            .expect("Internal Error: Player position undefined")
    }

    fn player_hp(&self) -> (i32, i32) {
        let player = self.chara.get(CharaId::Player);
        (player.attr.max_hp, player.hp)
    }

    fn player_item_location(&self, id: &str) -> Option<ItemLocation> {
        let idx: ItemIdx = gobj::id_to_idx_checked(id)?;
        let ill = ItemListLocation::Chara {
            cid: CharaId::Player,
        };
        let il = self.get_item_list(ill);

        let i = il.find(idx)?;
        Some((ill, i))
    }

    fn map_size(&self) -> (u32, u32) {
        let map = self.get_current_map();
        (map.w, map.h)
    }

    fn chara_pos(&self, cid: CharaId) -> Option<Vec2d> {
        let map = self.get_current_map();
        map.chara_pos(cid)
    }

    fn on_map_entrance(&self) -> bool {
        use common::gamedata::map::SpecialTileKind;

        let map = self.get_current_map();
        let tile = &map.tile[self.player_pos()];
        match tile.special {
            SpecialTileKind::Stairs { .. } | SpecialTileKind::SiteSymbol { .. } => true,
            _ => false,
        }
    }

    fn item_on_player_tile(&self) -> Option<&common::gamedata::item::ItemList> {
        let player_pos = self.player_pos();
        self.get_current_map().tile[player_pos].item_list.as_ref()
    }

    fn is_item_on_player_tile(&self) -> bool {
        let list = self.item_on_player_tile();
        !(list.is_none() || list.unwrap().is_empty())
    }

    fn is_open_air(&self, mid: MapId) -> bool {
        match mid {
            MapId::SiteMap { sid, floor } => match sid.kind {
                SiteKind::AutoGenDungeon => false,
                SiteKind::Town => floor == 0,
                SiteKind::Other => false,
            },
            MapId::RegionMap { .. } => true,
        }
    }

    fn has_item(&self, idx: ItemIdx) -> u32 {
        let il = self.get_item_list(ItemListLocation::Chara {
            cid: CharaId::Player,
        });
        il.iter()
            .filter_map(|(item, n)| if item.idx == idx { Some(n) } else { None })
            .sum()
    }

    fn search_item(&self, idx: ItemIdx) -> Vec<ItemLocation> {
        let ill = ItemListLocation::Chara {
            cid: CharaId::Player,
        };
        let list = self.get_item_list(ill);
        let mut il = Vec::new();
        for (i, (item, _)) in list.iter().enumerate() {
            if item.idx == idx {
                il.push((ill, i as u32))
            }
        }
        il
    }
}
