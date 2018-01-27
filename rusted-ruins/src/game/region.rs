
use common::basic::MAX_AUTO_GEN_DUNGEONS;
use common::gamedata::GameData;
use common::gamedata::region::*;
use common::gamedata::site::{DungeonKind, SiteKind};
use common::gamedata::map::*;
use super::map::choose_empty_tile;

pub fn add_region(gd: &mut GameData, id: &str) {
    let map = super::map::from_template::from_template_id("!east-coast");
    
    let region = Region::new(id, map);
    gd.region.add_region(region);
}

/// Generate one dungeon and add it to the region
pub fn gen_dungeon(gd: &mut GameData, rid: RegionId) {
    if MAX_AUTO_GEN_DUNGEONS <= gd.region.get(rid).get_site_n(SiteKind::AutoGenDungeon) { return; }
    
    let pos = {
        let region_map = gd.region.get_map(MapId::from(rid));
        match choose_empty_tile(region_map) {
            Some(pos) => pos,
            None => {
                warn!("Dungeon generation failed: No empty tile");
                return;
            }
        }
    };
    let dungeon_kind = DungeonKind::Cave;
    
    super::site::add_dungeon_site(gd, dungeon_kind, pos);
    let region_map = gd.region.get_map_mut(MapId::from(rid));
    region_map.tile[pos].special = SpecialTileKind::SiteSymbol { kind: SiteSymbolKind::Cave };
}

