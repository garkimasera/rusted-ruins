
use common::gamedata::GameData;
use common::gamedata::site::{Site, SiteId, SiteKind};

pub fn add_dungeon_site(gd: &mut GameData) -> SiteId {
    let site = Site::new("Ruin Hoge", 1);

    let sid = gd.add_site(site, SiteKind::Start);
    
    let mut map = super::map::builder::MapBuilder::new(40, 40).build();
    
    gd.add_map(map, sid);
    
    sid
}

