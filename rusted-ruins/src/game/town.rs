use crate::game::item::gen::gen_item_from_id;
use common::gamedata::*;
use common::gobj;
use common::obj::SiteGenObject;
use geom::Coords;

/// Create town from SiteGenObect and add it to region map
pub fn add_town(gd: &mut GameData, rid: RegionId, pos: Coords, town_id: &str) {
    let sid = crate::game::site::gen::add_site_from_obj(gd, rid, pos, town_id).unwrap();

    let town = Town::new(town_id);
    let site_content = SiteContent::Town {
        town: Box::new(town),
    };
    let mut site = gd.region.get_site_mut(sid);
    site.content = site_content;

    let sg: &SiteGenObject = gobj::get_by_id(town_id);

    // Locate delivery chest
    if let Some((floor, pos, ref id)) = sg.delivery_chest {
        let mut item = gen_item_from_id(id, 0);
        item.flags |= ItemFlags::FIXED;
        let ill = ItemListLocation::OnMap {
            mid: MapId::SiteMap { sid, floor },
            pos,
        };
        let item_list = gd.get_item_list_mut(ill);
        item_list.clear();
        item_list.append_simple(item, 1);
    }
}
