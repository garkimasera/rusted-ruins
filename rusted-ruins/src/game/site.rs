
use common::gamedata::GameData;
use common::gamedata::site::{Site, SiteId, SiteKind};

pub fn add_dungeon_site(gd: &mut GameData) -> SiteId {
    let site = Site::new("Ruin Hoge", 10);

    let sid = gd.add_site(site, SiteKind::Start);
    
    extend_site_floor(gd, sid);
    
    sid
}

pub fn extend_site_floor(gd: &mut GameData, sid: SiteId) {
    let map = super::map::builder::MapBuilder::new(40, 40).build();
    
    gd.add_map(map, sid);
}

