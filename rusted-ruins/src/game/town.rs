
use array2d::Vec2d;
use common::obj::SiteGenObject;
use common::gamedata::*;
use common::gobj;
use super::saveload::gen_box_id;

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

        let map_random_id = gen_box_id(gd);
        gd.add_map(map, sid, map_random_id);
    }

    super::site::gen::add_unique_citizens(gd, sid, sg);

    // Add symbol to region map
    {
        let map = gd.region.get_map_mut(MapId::from(rid));
        map.tile[pos].special = SpecialTileKind::SiteSymbol { kind: SiteSymbolKind::Town };
    }

    // Shop settings
    let sg: &SiteGenObject = gobj::get_by_id(town_id);
    {
        let town = match gd.region.get_site_mut(sid).content {
            SiteContent::Town { ref mut town } => town,
            _ => unreachable!(),
        };
        
        for shop_gen_data in &sg.shops {
            let shop = Shop {
                kind: shop_gen_data.kind,
                items: ItemList::new(),
                level: 1,
            };
            town.add_shop(shop, shop_gen_data.chara_n);
        }
    }
    update_shops(gd, sid);
}

/// Update shop states
pub fn update_shops(gd: &mut GameData, sid: SiteId) {
    use crate::game::shop::update_items_on_shop;
    
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

