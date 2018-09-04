
use array2d::Vec2d;
use common::obj::SiteGenObject;
use common::gamedata::*;
use common::gobj;
use rules::RULES;

/// Create town from SiteGenObect and add it to region map
pub fn add_town(gd: &mut GameData, rid: RegionId, pos: Vec2d, town_id: &str) {
    let sg: &SiteGenObject = gobj::get_by_id(town_id);
    let town = Town::new(town_id);
    let mut site = Site::new(sg.map_template_id.len() as u32);
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

/// Update shop states
pub fn update_shops(gd: &mut GameData, sid: SiteId) {
    let site = gd.region.get_site_mut(sid);
    let town = match &mut site.content {
        SiteContent::Town { ref mut town } => town,
        _ => {
            warn!("Tried to update shops for a site which is not town.");
            return;
        }
    };

    for shop in town.iter_shops_mut() {
        update_items_on_shop(shop)
    }
}

/// Update items on a shop
fn update_items_on_shop(shop: &mut Shop) {
    use std::cmp::{min, max};
    use game::item::gen::gen_dungeon_item;
    
    shop.items.clear();

    let n_gen_item = ::rng::gen_range(RULES.town.min_shop_items, RULES.town.max_shop_items);
    
    for _ in 0..n_gen_item {
        shop.items.append(gen_dungeon_item(1), 1);
    }
}

