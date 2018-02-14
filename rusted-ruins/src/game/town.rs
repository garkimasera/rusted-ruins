
use array2d::Vec2d;
use common::obj::SiteGenObject;
use common::gamedata::GameData;
use common::gamedata::region::RegionId;
use common::gamedata::site::*;
use common::gamedata::map::*;
use common::gamedata::town::*;
use common::gobj;

/// Create town from SiteGenObect and add it to region map
pub fn add_town(gd: &mut GameData, rid: RegionId, pos: Vec2d, town_id: &str) {
    let sg: &SiteGenObject = gobj::get_by_id(town_id);
    let town = Town::new(town_id);
    let mut site = Site::new(town_id, sg.map_template_id.len() as u32);
    let site_content = SiteContent::Town { town: Box::new(town) };
    site.content = site_content;
    let sid = if let Some(sid) = gd.add_site(site, SiteKind::Town, rid, pos) {
        sid
    } else {
        warn!("{:?} is already occupied, and failed to add a new town", pos);
        return;
    };

    for map_template_id in &sg.map_template_id {
        let map = super::map::from_template::from_template_id(map_template_id)
            .expect(&format!("Map template not found: {}", map_template_id));
        gd.add_map(map, sid);
    }

    super::site::gen::add_unique_citizens(gd, sid, sg);

    // Add symbol to region map
    {
        let map = gd.region.get_map_mut(MapId::from(rid));
        map.tile[pos].special = SpecialTileKind::SiteSymbol { kind: SiteSymbolKind::Town };
    }
}

