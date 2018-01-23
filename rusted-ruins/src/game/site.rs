
use array2d::*;
use common::gamedata::GameData;
use common::gamedata::site::{Site, SiteId, SiteKind};
use common::gamedata::region::RegionId;

pub fn add_dungeon_site(gd: &mut GameData) -> SiteId {
    let site = Site::new("Ruin Hoge", 10);

    let sid = gd.add_site(site, SiteKind::Other, RegionId::default(), Vec2d::new(0, 0)).unwrap();
    
    extend_site_floor(gd, sid);
    
    sid
}

/// Extend dungion site by one floor
pub fn extend_site_floor(gd: &mut GameData, sid: SiteId) {
    let floor = gd.region.get_site(sid).floor_num();
    let map = super::map::builder::MapBuilder::new(40, 40).floor(floor).build();
    
    gd.add_map(map, sid);
}

