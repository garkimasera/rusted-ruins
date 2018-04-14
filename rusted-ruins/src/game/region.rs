
use common::basic::MAX_AUTO_GEN_DUNGEONS;
use common::gamedata::*;
use common::regiongen::*;
use common::gobj;
use super::map::choose_empty_tile;
use rng::{self, Rng};

pub fn add_region(gd: &mut GameData, id: &str) {
    let rg: &RegionGenObject = gobj::get_by_id(id);
    
    let map = if let Some(map) = super::map::from_template::from_template_id(&rg.id) {
        map
    } else {
        error!("Map generation failed from \"{}\"", rg.id);
        panic!();
    };
    
    let region = Region::new(id, map);
    let rid = gd.region.add_region(region);
    add_sites_from_genobj(gd, rg, rid);
}

/// Generate dungeons up to the max
pub fn gen_dungeon_max(gd: &mut GameData, rid: RegionId) {
    use common::basic::MAX_AUTO_GEN_DUNGEONS;
    let n_autogen_dungeons = gd.region.get(rid).get_site_n(SiteKind::AutoGenDungeon);

    if n_autogen_dungeons == MAX_AUTO_GEN_DUNGEONS {
        return;
    }
    
    for _ in 0..(MAX_AUTO_GEN_DUNGEONS - n_autogen_dungeons) {
        gen_dungeon(gd, rid);
    }
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
    let dungeon_kind = *rng::get_rng().choose(&[DungeonKind::Cave, DungeonKind::Ruin]).unwrap();
    
    super::site::add_dungeon_site(gd, dungeon_kind, pos);
    let region_map = gd.region.get_map_mut(MapId::from(rid));
    region_map.tile[pos].special = SpecialTileKind::SiteSymbol { kind: SiteSymbolKind::Cave };
}

fn add_sites_from_genobj(gd: &mut GameData, rg: &RegionGenObject, rid: RegionId) {
    // Add towns
    for &(ref site_gen_id, pos) in &rg.towns {
        super::town::add_town(gd, rid, pos, site_gen_id);
        debug!("Created new town \"{}\" at {} in {:?}", site_gen_id, pos, rid);
    }
}

