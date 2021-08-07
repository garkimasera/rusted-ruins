use crate::game::extrait::*;
use common::gamedata::*;
use common::gobj;
use common::objholder::*;
use geom::*;
use rules::RULES;

/// Helper functions to get information for event processing and drawing
#[extend::ext(pub, name = InfoGetter)]
impl GameData {
    /// Get player's name
    fn player_name(&self) -> &str {
        self.chara
            .get(CharaId::Player)
            .name
            .as_ref()
            .expect("player's name is None")
    }

    /// Get player's position
    fn player_pos(&self) -> Vec2d {
        self.get_current_map()
            .chara_pos(CharaId::Player)
            .expect("Internal Error: Player position undefined")
    }

    /// Get player's (maxhp, hp)
    fn player_hp(&self) -> (i32, i32) {
        let player = self.chara.get(CharaId::Player);
        (player.attr.max_hp, player.hp)
    }

    /// Get item location that player has
    fn player_item_location(&self, id: &str) -> Option<ItemLocation> {
        let idx: ItemIdx = gobj::id_to_idx_checked(id)?;
        let ill = ItemListLocation::Chara {
            cid: CharaId::Player,
        };
        let il = self.get_item_list(ill);

        let i = il.find(idx)?;
        Some((ill, i))
    }

    /// Get current map size
    fn map_size(&self) -> (u32, u32) {
        let map = self.get_current_map();
        (map.w, map.h)
    }

    /// Character position on the current map
    fn chara_pos(&self, cid: CharaId) -> Option<Vec2d> {
        let map = self.get_current_map();
        map.chara_pos(cid)
    }

    /// Player's current tile is entrance/exit or not
    fn on_map_entrance(&self) -> bool {
        let map = self.get_current_map();
        let tile = &map.tile[self.player_pos()];
        matches!(
            tile.special,
            SpecialTileKind::Stairs { .. } | SpecialTileKind::SiteSymbol { .. }
        )
    }

    /// Return item list in the tile that player stands on
    fn item_on_player_tile(&self) -> &ItemList {
        let player_pos = self.player_pos();
        &self.get_current_map().tile[player_pos].item_list
    }

    /// Return any item exist or not on player tile
    fn is_item_on_player_tile(&self) -> bool {
        !self.item_on_player_tile().is_empty()
    }

    /// Judge given map is open-air or not
    fn is_open_air(&self, mid: MapId) -> bool {
        match mid {
            MapId::SiteMap { sid, floor } => match sid.kind {
                SiteKind::AutoGenDungeon => false,
                SiteKind::Town | SiteKind::Player => floor == 0,
                SiteKind::Temp => {
                    let site = self.region.get_site(sid);
                    match site.content {
                        SiteContent::Temp { is_open_air, .. } => is_open_air,
                        _ => unreachable!(),
                    }
                }
                SiteKind::Other => false,
            },
            MapId::RegionMap { .. } => true,
        }
    }

    /// Get the number of specified item player has
    fn has_item(&self, idx: ItemIdx) -> u32 {
        let il = self.get_item_list(ItemListLocation::Chara {
            cid: CharaId::Player,
        });
        il.iter()
            .filter_map(|(item, n)| if item.idx == idx { Some(n) } else { None })
            .sum()
    }

    /// Get the number of specified item player has by id
    fn has_item_by_id(&self, id: &str) -> Option<u32> {
        let idx: ItemIdx = gobj::id_to_idx_checked(id)?;
        Some(self.has_item(idx))
    }

    /// Get the item location of specified item
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

    /// Get list of harvestable items
    fn search_harvestable_item(&self, tile: Vec2d) -> Vec<(ItemLocation, ItemIdx)> {
        let item_list = self.get_item_list_on_current_map(tile);

        let ill = ItemListLocation::OnMap {
            mid: self.get_current_mapid(),
            pos: tile,
        };

        let mut v = Vec::new();
        for (i, item) in item_list.items.iter().enumerate() {
            let item_idx = item.0.idx;
            let item_obj = gobj::get_obj(item_idx);

            if item_obj
                .attrs
                .iter()
                .any(|attr| matches!(attr, ItemObjAttr::Harvest { .. }))
            {
                v.push(((ill, i as u32), item_idx))
            }
        }
        v
    }

    /// Get relationship between two characters
    fn chara_relation(&self, chara: CharaId, other: CharaId) -> Relationship {
        let f1 = self.chara.get(chara).faction;
        let f2 = self.chara.get(other).faction;

        if f1 == f2 {
            return Relationship::Ally;
        }

        for status1 in &self.chara.get(chara).status {
            match status1 {
                CharaStatus::Hostile { faction } => {
                    if *faction == f2 {
                        return Relationship::Hostile;
                    }
                }
                _ => (),
            }
        }

        for status2 in &self.chara.get(other).status {
            match status2 {
                CharaStatus::Hostile { faction } => {
                    if *faction == f1 {
                        return Relationship::Hostile;
                    }
                }
                _ => (),
            }
        }

        let faction_relation = if f1 == FactionId::player() {
            self.faction.get(f2)
        } else if f2 == FactionId::player() {
            self.faction.get(f1)
        } else {
            RULES.faction.relation(f1, f2)
        };

        if faction_relation >= RULES.faction.relation_friend {
            Relationship::Friendly
        } else if faction_relation >= RULES.faction.relation_neutral {
            Relationship::Neutral
        } else {
            Relationship::Hostile
        }
    }

    /// Target is visible from given cid
    fn target_visible(&self, cid: CharaId, target: CharaId) -> bool {
        let p0 = if let Some(p0) = self.chara_pos(cid) {
            p0
        } else {
            return false;
        };

        let p1 = if let Some(p1) = self.chara_pos(target) {
            p1
        } else {
            return false;
        };

        if p0.mdistance(p1) > self.chara.get(cid).attr.view_range {
            return false;
        }

        let map = self.get_current_map();

        for p in LineIter::new(p0, p1) {
            if !map.tile[p].wall.is_empty() {
                return false;
            }
        }

        true
    }

    /// Get shortcut availability (available, remaining)
    fn shortcut_available(&self, n: usize) -> Option<(bool, Option<u32>)> {
        let shortcut = self.settings.action_shortcuts[n]?;

        match shortcut {
            ActionShortcut::Throw(idx)
            | ActionShortcut::Drink(idx)
            | ActionShortcut::Eat(idx)
            | ActionShortcut::Use(idx)
            | ActionShortcut::Read(idx) => {
                let sum = self
                    .search_item(idx)
                    .iter()
                    .map(|il| self.get_item(*il).1)
                    .sum();
                Some((sum > 0, Some(sum)))
            }
            ActionShortcut::Release(idx) => {
                let sum = self
                    .search_item(idx)
                    .iter()
                    .map(|il| self.get_item(*il).0.charge().unwrap_or(0))
                    .sum();
                Some((sum > 0, Some(sum)))
            }
        }
    }
}
