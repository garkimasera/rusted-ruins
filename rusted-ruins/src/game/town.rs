use common::gamedata::*;
use common::gobj;
use common::obj::SiteGenObject;
use geom::Vec2d;

/// Create town from SiteGenObect and add it to region map
pub fn add_town(gd: &mut GameData, rid: RegionId, pos: Vec2d, town_id: &str) {
    let sid = crate::game::site::gen::add_site_from_obj(gd, rid, pos, town_id).unwrap();

    let town = Town::new(town_id);
    let site_content = SiteContent::Town {
        town: Box::new(town),
    };
    let mut site = gd.region.get_site_mut(sid);
    site.content = site_content;

    // Shop settings
    let sg: &SiteGenObject = gobj::get_by_id(town_id);
    {
        let town = match gd.region.get_site_mut(sid).content {
            SiteContent::Town { ref mut town } => town,
            _ => unreachable!(),
        };

        for shop_gen_data in &sg.shops {
            let shop = Shop {
                items: ItemList::default(),
                level: 1,
            };
            town.add_shop(shop, shop_gen_data.chara_n);
        }
    }
    update_shops(gd, sid, sg);
}

/// Update shop states
pub fn update_shops(gd: &mut GameData, sid: SiteId, sg: &SiteGenObject) {
    use crate::game::shop::update_items_on_shop;

    let site = gd.region.get_site_mut(sid);
    let town = match &mut site.content {
        SiteContent::Town { ref mut town } => town,
        _ => {
            warn!("Tried to update shops for a site which is not town.");
            return;
        }
    };

    for (i, shop) in town.iter_shops_mut().enumerate() {
        let shop_gen = &sg.shops[i];
        update_items_on_shop(shop, shop_gen);
    }
}
