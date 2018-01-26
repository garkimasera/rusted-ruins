
use array2d::*;
use common::gamedata::GameData;
use common::gamedata::site::{Site, SiteId, SiteContent, SiteKind, DungeonKind};
use common::gamedata::region::RegionId;
use common::gobj;
use rules::RULES;
use super::map::builder::MapBuilder;

pub fn add_dungeon_site(gd: &mut GameData, dungeon_kind: DungeonKind) -> SiteId {
    let mut site = Site::new("Ruin Hoge", 10);
    site.content = SiteContent::AutoGenDungeon { dungeon_kind };
    let sid = gd.add_site(site, SiteKind::Other, RegionId::default(), Vec2d::new(0, 0)).unwrap();
    extend_site_floor(gd, sid);
    
    sid
}

/// Extend dungion site by one floor
pub fn extend_site_floor(gd: &mut GameData, sid: SiteId) {
    let floor = gd.region.get_site(sid).floor_num();
    let map = match gd.region.get_site(sid).content {
        SiteContent::AutoGenDungeon { dungeon_kind } => {
            let tile_idx = gobj::id_to_idx(&RULES.dungeon_gen[&dungeon_kind].terrain[0][0]);
            let wall_idx = gobj::id_to_idx(&RULES.dungeon_gen[&dungeon_kind].terrain[0][1]);
            MapBuilder::new(40, 40).floor(floor).tile(tile_idx).wall(wall_idx).build()
        }
        _ => {
            MapBuilder::new(40, 40).floor(floor).build()
        }
    };
    
    gd.add_map(map, sid);
}

