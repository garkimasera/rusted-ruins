pub mod builder;
pub mod from_template;
pub mod search;
pub mod tile_info;
mod update;
pub mod wall_damage;
pub mod wilderness;

use super::chara::gen::create_npc_chara;
use super::extrait::*;
use super::item::gen::gen_dungeon_item;
use super::Game;
use crate::text::ToText;
use common::basic::MAX_ITEM_FOR_DRAW;
use common::gamedata::*;
use common::gobj;
use common::obj::TileKind;
use geom::*;
use rules::RULES;

#[extend::ext(pub)]
impl Map {
    /// The tile is passable for given character or not.
    fn is_passable(&self, _chara: &Chara, pos: Vec2d) -> bool {
        if !self.is_inside(pos) {
            return false;
        }

        if self.tile[pos].wall.is_empty() {
            let tile = gobj::get_obj(self.tile[pos].main_tile());
            match tile.kind {
                TileKind::Ground => true,
                TileKind::Water => false,
            }
        } else {
            false
        }
    }

    fn move_chara(&mut self, cid: CharaId, dir: Direction) -> bool {
        if let Some(p) = self.chara_pos(cid) {
            let new_p = p + dir.as_vec();
            self.swap_chara(p, new_p)
        } else {
            false
        }
    }

    /// Locate item at the specified tile.
    /// Usually should use GameData functions instead of this to move and append item.
    fn locate_item(&mut self, item: Item, pos: Vec2d, n: u32) {
        self.tile[pos].item_list.append(item, n);
    }

    /// Reveal map
    fn reveal<F: FnMut(Vec2d) -> bool>(&mut self, mut visible: F) {
        for p in self.tile.iter_idx() {
            if !visible(p) {
                continue;
            }

            let tile = &self.tile[p];
            let observed_tile = &mut self.observed_tile[p];
            observed_tile.tile = true;
            observed_tile.wall = tile.wall;
            observed_tile.deco = tile.deco;
            observed_tile.special = tile.special;
            observed_tile.items.clear();

            for &(ref item, _) in tile.item_list.iter().take(MAX_ITEM_FOR_DRAW) {
                observed_tile.items.push((item.idx, item.img_variation()));
            }
        }
    }

    fn tile_fertility(&self, pos: Vec2d) -> u8 {
        if let Some(tile) = self.tile.get(pos) {
            let mut fertility = 0u8;
            for t in tile.tile.iter_idx() {
                let tile_obj = gobj::get_obj(t);
                fertility = fertility.saturating_add(tile_obj.fertility);
            }
            fertility
        } else {
            0
        }
    }

    fn is_empty_tile(&self, pos: Vec2d) -> bool {
        if self.is_inside(pos) {
            let tile = &self.tile[pos];
            is_empty_tile(tile)
        } else {
            false
        }
    }

    fn empty_tile_around(&self, pos: Vec2d) -> Option<Vec2d> {
        for pos in SpiralIter::new(pos).take(100) {
            if self.is_inside(pos) && self.is_empty_tile(pos) {
                return Some(pos);
            }
        }

        None
    }
}

/// Function to determine the tile is empty or not
fn is_empty_tile(tile: &TileInfo) -> bool {
    if tile.wall.is_empty() && tile.chara.is_none() && tile.special.is_none() {
        let tile_idx = tile.main_tile();
        let tile_obj = gobj::get_obj(tile_idx);
        tile_obj.kind == TileKind::Ground
    } else {
        false
    }
}

pub fn switch_map(game: &mut Game<'_>, destination: Destination) {
    game.ui_request.push_back(super::UiRequest::StopCentering);
    game.clear_target();

    let save_dir = game.save_dir.as_ref().unwrap();
    let old_mid = game.gd.get_current_mapid();
    let new_mid = destination_to_mid(&game.gd, destination);

    if !game.gd.region.map_exist(new_mid) {
        assert!(!new_mid.is_region_map());
        info!("{:?} is not exist, so try to create new floor", new_mid);
        super::dungeon_gen::extend_site_floor(&mut game.gd, new_mid.sid());
    } else {
        game.gd.region.preload_map(new_mid, save_dir.join("maps"));
    }
    let new_player_pos = destination_to_pos(&game.gd, destination);

    let gd = &mut game.gd;

    info!("Switch map to {:?}", new_mid);

    gd.get_current_map_mut().last_visit = crate::game::time::current_time();

    process_map_before_switch(gd, old_mid);

    // Change current mapid
    gd.set_current_mapid(new_mid);

    // Locate party members
    gd.get_current_map_mut()
        .locate_chara(CharaId::Player, new_player_pos);

    if !new_mid.is_region_map() {
        let cids = gd.player.party.clone();
        let map = gd.get_current_map_mut();
        for cid in cids {
            if let Some(pos) = map.empty_tile_around(new_player_pos) {
                map.locate_chara(cid, pos);
            }
        }
    }

    // Remove temp site
    if !old_mid.is_region_map() {
        let sid = old_mid.sid();
        if sid.kind == SiteKind::Temp {
            // If new site is not the same as old site.
            if new_mid.is_region_map() || new_mid.sid() != sid {
                gd.remove_site(sid);
            }
        }
    }

    crate::audio::play_sound("floor-change");
    crate::audio::play_music(&gd.get_current_map().music);
    update::update_map(game);
    super::view::update_view_map(game);
}

fn process_map_before_switch(gd: &mut GameData, mid: MapId) {
    // Remove party members from old map
    if !mid.is_region_map() {
        for cid in gd.player.party.clone() {
            gd.remove_chara_from_map(cid);
        }
    }

    // Reset npc ai state
    for cid in gd.get_charas_on_map() {
        let chara = gd.chara.get_mut(cid);
        if chara.ai.state.is_combat() {
            chara.ai.state = AiState::default_search();
        }
    }
}

/// Convert Destination to map id.
pub fn destination_to_mid(gd: &GameData, dest: Destination) -> MapId {
    let prev_mid = gd.get_current_mapid();

    match dest {
        Destination::Floor(n) => {
            if n != FLOOR_OUTSIDE {
                MapId::SiteMap {
                    sid: prev_mid.sid(),
                    floor: n,
                }
            } else {
                MapId::RegionMap {
                    rid: prev_mid.rid(),
                }
            }
        }
        Destination::Exit => MapId::RegionMap {
            rid: prev_mid.rid(),
        },
        Destination::MapId(mid) => mid,
        Destination::MapIdWithPos(mid, _) => mid,
        Destination::MapIdWithEntrance(mid, _) => mid,
    }
}

/// Get destination position.
pub fn destination_to_pos(gd: &GameData, dest: Destination) -> Vec2d {
    let prev_mid = gd.get_current_mapid();
    let new_mid = destination_to_mid(gd, dest);
    let pos = match dest {
        Destination::Floor(_) => None,
        Destination::Exit => {
            let pos = gd
                .region
                .get_site_pos(prev_mid.sid())
                .expect("tried to exit from site that don't have pos");
            Some(pos)
        }
        Destination::MapId(mid) => {
            assert_eq!(new_mid, mid);
            None
        }
        Destination::MapIdWithPos(mid, pos) => {
            assert_eq!(new_mid, mid);
            Some(pos)
        }
        Destination::MapIdWithEntrance(mid, entrance) => {
            assert_eq!(new_mid, mid);
            let map = gd.region.get_map(new_mid);
            Some(map.entrance[entrance as usize])
        }
    };
    if let Some(pos) = pos {
        return pos;
    }

    assert!(gd.region.map_exist(new_mid));

    // Search position automatically
    let dest_map = gd.region.get_map(new_mid);
    let pos = if let Some(p) = dest_map.search_stairs(prev_mid.floor()) {
        p
    } else {
        dest_map.entrance.get(0).copied().unwrap_or(Vec2d(0, 0))
    };
    pos
}

pub fn gen_npcs(gd: &mut GameData, mid: MapId, n: u32, floor_level: u32) {
    let dungeon_kind = match gd.region.get_site(mid.sid()).content {
        SiteContent::AutoGenDungeon { dungeon_kind } => dungeon_kind,
        _ => {
            return;
        }
    };

    for _ in 0..n {
        if let Some(p) = choose_empty_tile(gd.region.get_map(mid)) {
            if let Some(chara) = create_npc_chara(dungeon_kind, floor_level) {
                trace!("Generate new npc {}", chara.to_text());
                let cid = gd.add_chara_to_map(chara, mid);
                let map = gd.region.get_map_mut(mid);
                map.locate_chara(cid, p);
            }
        } else {
            warn!("Failed npc generating because empty tile not found");
            return;
        }
    }
}

/// Choose one empty tile in random
pub fn choose_empty_tile(map: &Map) -> Option<Vec2d> {
    use rng::gen_range;
    const MAX_TRY: usize = 10;

    for _ in 0..MAX_TRY {
        let p = Vec2d(gen_range(0..map.w) as i32, gen_range(0..map.h) as i32);
        let tile = &map.tile[p];

        // Empty tile don't has wall, chara, and isn't special tile.
        if is_empty_tile(tile) {
            return Some(p);
        }
    }

    // If random tile choosing is failed many times, count empty tiles and choose
    let n_empty_tile = map.tile.iter().filter(|t| is_empty_tile(t)).count();
    if n_empty_tile == 0 {
        None
    } else {
        let r = gen_range(0..n_empty_tile);
        let p = map
            .tile
            .iter_with_idx()
            .filter(|&(_, t)| is_empty_tile(t))
            .nth(r)
            .unwrap()
            .0;
        Some(p)
    }
}

/// Locate some items for a new map
pub fn gen_items(gd: &mut GameData, mid: MapId) {
    use rng::*;
    let dungeon_kind = {
        let site = gd.region.get_site(mid.sid());
        match site.content {
            SiteContent::AutoGenDungeon { dungeon_kind } => dungeon_kind,
            _ => {
                return;
            } // No item generation
        }
    };
    let item_gen_probability = RULES.dungeon_gen[&dungeon_kind].item_gen_probability;
    let item_gen_probability = if (0.0..=1.0).contains(&item_gen_probability) {
        item_gen_probability
    } else {
        warn!(
            "invalid value {} for item_gen_probablility",
            item_gen_probability
        );
        return;
    };
    let map = gd.region.get_map_mut(mid);

    for p in map.tile.iter_idx() {
        let tile = &mut map.tile[p];
        if !tile.wall.is_empty() {
            continue;
        }

        if get_rng().gen_bool(item_gen_probability) {
            if let Some(item) = gen_dungeon_item(mid.floor(), dungeon_kind) {
                map.locate_item(item, p, 1);
            }
        }
    }
}

pub fn update_observed_map(game: &mut Game<'_>) {
    let view_map = &game.view_map;
    let map = game.gd.get_current_map_mut();
    map.reveal(|pos| view_map.get_tile_visible(pos));
}
